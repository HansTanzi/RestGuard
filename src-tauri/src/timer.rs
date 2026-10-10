//! 工作/休息的状态机。不依赖 Tauri，所有方法都显式传入当前时间，便于测试。

use crate::config::Config;
use serde::Serialize;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Phase {
    /// 工作中，倒计时结束后进入休息
    Working,
    /// 强制休息中，遮罩无法关闭
    Resting,
    /// 休息时间已到，等待用户点击“开始工作”
    RestOver,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TimerEvent {
    RestStarted,
    RestFinished,
}

/// 发给前端和托盘的状态快照
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub phase: Phase,
    pub remaining_secs: u64,
    pub total_secs: u64,
    pub can_postpone: bool,
    pub postpones_left: u32,
}

pub struct Timer {
    work: Duration,
    rest: Duration,
    postpone: Duration,
    max_postpones: u32,

    phase: Phase,
    started: Instant,
    deadline: Instant,
    postpones_used: u32,
    /// 本次休息是用户主动开始的，主动休息不提供推迟
    voluntary: bool,
}

impl Timer {
    pub fn new(cfg: &Config, now: Instant) -> Self {
        Self {
            work: cfg.work(),
            rest: cfg.rest(),
            postpone: cfg.postpone(),
            max_postpones: cfg.max_postpones,
            phase: Phase::Working,
            started: now,
            deadline: now + cfg.work(),
            postpones_used: 0,
            voluntary: false,
        }
    }

    pub fn tick(&mut self, now: Instant) -> Option<TimerEvent> {
        if now < self.deadline {
            return None;
        }
        match self.phase {
            Phase::Working => {
                self.begin_rest(now, false);
                Some(TimerEvent::RestStarted)
            }
            Phase::Resting => {
                self.phase = Phase::RestOver;
                Some(TimerEvent::RestFinished)
            }
            Phase::RestOver => None,
        }
    }

    /// 托盘“提前休息”
    pub fn rest_now(&mut self, now: Instant) -> bool {
        if self.phase != Phase::Working {
            return false;
        }
        self.begin_rest(now, true);
        true
    }

    /// 休息中点击“过会儿再休息”
    pub fn postpone(&mut self, now: Instant) -> bool {
        if !self.can_postpone() {
            return false;
        }
        self.postpones_used += 1;
        self.begin_work(now, self.postpone);
        true
    }

    /// 休息结束后点击“开始工作”。完整休息过一次，推迟次数清零
    pub fn start_work(&mut self, now: Instant) -> bool {
        if self.phase != Phase::RestOver {
            return false;
        }
        self.postpones_used = 0;
        self.begin_work(now, self.work);
        true
    }

    /// 开发调试用：无论休息是否结束，直接回到工作
    pub fn skip_rest(&mut self, now: Instant) -> bool {
        if self.phase == Phase::Working {
            return false;
        }
        self.postpones_used = 0;
        self.begin_work(now, self.work);
        true
    }

    /// 设置面板保存后立即应用新配置。
    /// 正常工作期间修改了工作时长时，按新时长重新开始倒计时；推迟期间的倒计时不受影响
    pub fn apply_config(&mut self, cfg: &Config, now: Instant) {
        let work_changed = cfg.work() != self.work;
        self.work = cfg.work();
        self.rest = cfg.rest();
        self.postpone = cfg.postpone();
        self.max_postpones = cfg.max_postpones;
        // 推迟次数清零前，postpones_used > 0 说明当前处于推迟后的倒计时
        if self.phase == Phase::Working && self.postpones_used == 0 && work_changed {
            self.begin_work(now, self.work);
        }
    }

    pub fn snapshot(&self, now: Instant) -> Snapshot {
        let remaining = self.deadline.saturating_duration_since(now);
        Snapshot {
            phase: self.phase,
            remaining_secs: ceil_secs(remaining),
            total_secs: ceil_secs(self.deadline - self.started),
            can_postpone: self.can_postpone(),
            postpones_left: self.max_postpones.saturating_sub(self.postpones_used),
        }
    }

    fn can_postpone(&self) -> bool {
        self.phase == Phase::Resting && !self.voluntary && self.postpones_used < self.max_postpones
    }

    fn begin_rest(&mut self, now: Instant, voluntary: bool) {
        self.phase = Phase::Resting;
        self.voluntary = voluntary;
        self.started = now;
        self.deadline = now + self.rest;
    }

    fn begin_work(&mut self, now: Instant, length: Duration) {
        self.phase = Phase::Working;
        self.voluntary = false;
        self.started = now;
        self.deadline = now + length;
    }
}

fn ceil_secs(d: Duration) -> u64 {
    d.as_millis().div_ceil(1000) as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const MIN: Duration = Duration::from_secs(60);

    fn timer(t0: Instant) -> Timer {
        Timer::new(&Config::default(), t0)
    }

    #[test]
    fn full_cycle() {
        let t0 = Instant::now();
        let mut t = timer(t0);

        assert_eq!(t.tick(t0 + 59 * MIN), None);
        assert_eq!(t.tick(t0 + 60 * MIN), Some(TimerEvent::RestStarted));
        assert!(!t.start_work(t0 + 61 * MIN), "休息没结束不能开始工作");

        assert_eq!(t.tick(t0 + 70 * MIN), Some(TimerEvent::RestFinished));
        assert_eq!(t.tick(t0 + 71 * MIN), None);
        assert!(t.start_work(t0 + 72 * MIN));
        assert_eq!(t.snapshot(t0 + 72 * MIN).remaining_secs, 3600);
    }

    #[test]
    fn postpone_is_limited_and_resets_after_full_rest() {
        let t0 = Instant::now();
        let mut t = timer(t0);
        let mut now = t0 + 60 * MIN;
        t.tick(now);

        for left in (0..3).rev() {
            assert!(t.postpone(now));
            assert_eq!(t.snapshot(now).postpones_left, left);
            now += 5 * MIN;
            assert_eq!(t.tick(now), Some(TimerEvent::RestStarted));
        }
        assert!(!t.snapshot(now).can_postpone);
        assert!(!t.postpone(now));

        now += 10 * MIN;
        t.tick(now);
        assert!(t.start_work(now));
        now += 60 * MIN;
        t.tick(now);
        assert!(t.snapshot(now).can_postpone, "完整休息后推迟次数应清零");
    }

    #[test]
    fn voluntary_rest_cannot_be_postponed() {
        let t0 = Instant::now();
        let mut t = timer(t0);
        assert!(t.rest_now(t0 + MIN));
        assert!(!t.rest_now(t0 + MIN), "已经在休息");
        assert!(!t.snapshot(t0 + MIN).can_postpone);
        assert!(!t.postpone(t0 + MIN));
    }

    #[test]
    fn skip_rest_works_mid_rest() {
        let t0 = Instant::now();
        let mut t = timer(t0);
        assert!(!t.skip_rest(t0), "工作中无需跳过");
        assert!(t.rest_now(t0 + MIN));
        assert!(t.skip_rest(t0 + 2 * MIN));
        assert_eq!(t.snapshot(t0 + 2 * MIN).phase, Phase::Working);
    }

    #[test]
    fn apply_config_restarts_work_only_when_needed() {
        let t0 = Instant::now();
        let mut t = timer(t0);
        let now = t0 + 10 * MIN;

        // 只改休息时长：工作倒计时不变
        let cfg = Config {
            rest_minutes: 3.0,
            ..Config::default()
        };
        t.apply_config(&cfg, now);
        assert_eq!(t.snapshot(now).remaining_secs, 50 * 60);

        // 改工作时长：按新时长重新计时
        let cfg = Config {
            work_minutes: 25.0,
            ..cfg
        };
        t.apply_config(&cfg, now);
        assert_eq!(t.snapshot(now).remaining_secs, 25 * 60);

        // 新的休息时长在下次休息生效
        let now = now + 25 * MIN;
        assert_eq!(t.tick(now), Some(TimerEvent::RestStarted));
        assert_eq!(t.snapshot(now).total_secs, 3 * 60);

        // 推迟期间改工作时长，不打断推迟倒计时
        assert!(t.postpone(now));
        let cfg = Config {
            work_minutes: 30.0,
            ..cfg
        };
        t.apply_config(&cfg, now + MIN);
        assert_eq!(t.snapshot(now + MIN).remaining_secs, 4 * 60);
    }

    #[test]
    fn lowering_max_postpones_takes_effect() {
        let t0 = Instant::now();
        let mut t = timer(t0);
        let cfg = Config {
            max_postpones: 0,
            ..Config::default()
        };
        t.apply_config(&cfg, t0);
        t.tick(t0 + 60 * MIN);
        assert!(!t.snapshot(t0 + 60 * MIN).can_postpone);
    }

    #[test]
    fn remaining_rounds_up() {
        let t0 = Instant::now();
        let t = timer(t0);
        let snap = t.snapshot(t0 + Duration::from_millis(500));
        assert_eq!(snap.remaining_secs, 3600);
        assert_eq!(snap.total_secs, 3600);
    }
}
