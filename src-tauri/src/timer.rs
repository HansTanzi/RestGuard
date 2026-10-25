//! Work/break state machine. Independent of Tauri; every method takes the current time explicitly, for testability.

use crate::config::{Config, Profile};
use serde::Serialize;
use std::time::{Duration, Instant};

/// How long after clicking "I'm in a meeting" to remind again; matches the button text in src/lib/i18n.ts
pub const MEETING_SNOOZE: Duration = Duration::from_secs(15 * 60);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum Phase {
    /// Working; a break starts when the countdown ends
    Working,
    /// On a forced break; the overlay can't be closed
    Resting,
    /// Break time is over, waiting for the user to click "Start working"
    RestOver,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TimerEvent {
    RestStarted,
    RestFinished,
}

/// State snapshot sent to the frontend and tray
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Snapshot {
    pub phase: Phase,
    pub remaining_secs: u64,
    pub total_secs: u64,
    pub can_postpone: bool,
    pub postpones_left: u32,
    /// Counting down a postponed break; like a break, quitting and changing settings or profiles aren't allowed
    pub postponed: bool,
    /// A break is due but held off because this exempt app is in the foreground
    pub held_by: Option<String>,
    /// Meeting software running when the break started; the overlay then offers "I'm in a meeting"
    pub meeting_app: Option<String>,
    /// Forced break; when false the overlay can be closed
    pub forced: bool,
    /// Offer "Skip this break" on the overlay
    pub can_skip: bool,
    /// Offer "Urgent? Shrink it" on the overlay
    pub can_shrink: bool,
    /// The overlay can be dragged within its screen when it doesn't fill it
    pub movable: bool,
    /// The overlay fills the screen in normal mode (so it can't be dragged even if movable)
    pub full_screen: bool,
}

pub struct Timer {
    work: Duration,
    rest: Duration,
    postpone: Duration,
    /// Profile of the current enforcement level
    profile: Profile,
    /// No input for this long counts as away; None disables idle detection
    idle_threshold: Option<Duration>,

    phase: Phase,
    started: Instant,
    deadline: Instant,
    postpones_used: u32,
    /// The work countdown is a postponed break
    postponed: bool,
    /// This break was started by the user; voluntary breaks can't be postponed
    voluntary: bool,
    held_by: Option<String>,
    meeting_app: Option<String>,
    /// Remaining work time frozen while the user is away
    away_remaining: Option<Duration>,
}

impl Timer {
    pub fn new(cfg: &Config, now: Instant) -> Self {
        Self {
            work: cfg.work(),
            rest: cfg.rest(),
            postpone: cfg.postpone(),
            profile: cfg.profile(),
            idle_threshold: cfg.idle(),
            phase: Phase::Working,
            started: now,
            deadline: now + cfg.work(),
            postpones_used: 0,
            postponed: false,
            voluntary: false,
            held_by: None,
            meeting_app: None,
            away_remaining: None,
        }
    }

    /// Reports how long the user has been inactive. Must be called before tick.
    /// While working, pauses the countdown once away past the threshold (the whole time away doesn't count as work);
    /// being away for a full break length counts as a full break, and the timer restarts on return
    pub fn idle(&mut self, now: Instant, idle: Duration) {
        if self.phase != Phase::Working {
            return;
        }
        let Some(threshold) = self.idle_threshold else {
            self.away_remaining = None;
            return;
        };
        if idle < threshold {
            // Back: continue from the frozen remaining time
            self.away_remaining = None;
        } else if idle >= self.rest {
            self.postpones_used = 0;
            self.begin_work(now, self.work);
        } else {
            let remaining = *self.away_remaining.get_or_insert_with(|| {
                let left_at = now.checked_sub(idle).unwrap_or(self.started);
                self.deadline.saturating_duration_since(left_at)
            });
            let total = self.deadline - self.started;
            self.deadline = now + remaining;
            self.started = self.deadline.checked_sub(total).unwrap_or(now);
        }
    }

    /// The work countdown has ended and a break is about to start
    pub fn rest_due(&self, now: Instant) -> bool {
        self.phase == Phase::Working && now >= self.deadline
    }

    /// Sets the exempt app holding off the break, None for no hold. Must be called before tick
    pub fn hold(&mut self, app: Option<String>) {
        self.held_by = app;
    }

    pub fn tick(&mut self, now: Instant) -> Option<TimerEvent> {
        if now < self.deadline {
            return None;
        }
        match self.phase {
            Phase::Working if self.held_by.is_some() => None,
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

    /// Tray "Rest now"
    pub fn rest_now(&mut self, now: Instant) -> bool {
        if self.phase != Phase::Working {
            return false;
        }
        self.begin_rest(now, true);
        true
    }

    /// "Rest later" clicked during a break
    pub fn postpone(&mut self, now: Instant) -> bool {
        if !self.can_postpone() {
            return false;
        }
        self.postpones_used += 1;
        self.begin_work(now, self.postpone);
        self.postponed = true;
        true
    }

    /// Records meeting software running when the break starts. Whether a meeting is on can't be determined, so the user decides
    pub fn set_meeting_app(&mut self, app: Option<String>) {
        self.meeting_app = app;
    }

    /// "I'm in a meeting" clicked during a break: back to work, remind again later, without using a postponement
    pub fn in_meeting(&mut self, now: Instant) -> bool {
        if !self.can_claim_meeting() {
            return false;
        }
        self.begin_work(now, MEETING_SNOOZE);
        true
    }

    /// "Start working" clicked after the break. A full break was taken, so postponements reset
    pub fn start_work(&mut self, now: Instant) -> bool {
        if self.phase != Phase::RestOver {
            return false;
        }
        self.postpones_used = 0;
        self.begin_work(now, self.work);
        true
    }

    /// When the break isn't forced, closing the overlay skips it
    pub fn dismiss(&mut self, now: Instant) -> bool {
        if self.phase == Phase::Working || self.profile.force_rest {
            return false;
        }
        self.postpones_used = 0;
        self.begin_work(now, self.work);
        true
    }

    /// "Skip this break" clicked on the overlay. Refused when the enforcement level doesn't allow skipping
    pub fn skip(&mut self, now: Instant) -> bool {
        if self.phase == Phase::Working || !self.profile.can_skip() {
            return false;
        }
        self.postpones_used = 0;
        self.begin_work(now, self.work);
        true
    }

    /// For development: go straight back to work whether or not the break is over
    pub fn skip_rest(&mut self, now: Instant) -> bool {
        if self.phase == Phase::Working {
            return false;
        }
        self.postpones_used = 0;
        self.begin_work(now, self.work);
        true
    }

    /// Applies the new config right after the settings panel saves.
    /// If the work duration changed during normal work, restart the countdown with the new duration; a postponement countdown is unaffected
    pub fn apply_config(&mut self, cfg: &Config, now: Instant) {
        let work_changed = cfg.work() != self.work;
        self.work = cfg.work();
        self.rest = cfg.rest();
        self.postpone = cfg.postpone();
        self.profile = cfg.profile();
        self.idle_threshold = cfg.idle();
        // Before postponements are reset, postpones_used > 0 means we're in a post-postponement countdown
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
            postpones_left: self.profile.max_postpones.saturating_sub(self.postpones_used),
            postponed: self.phase == Phase::Working && self.postponed,
            held_by: self.held_by.clone(),
            meeting_app: self
                .can_claim_meeting()
                .then(|| self.meeting_app.clone())
                .flatten(),
            forced: self.profile.force_rest,
            can_skip: self.phase != Phase::Working && self.profile.can_skip(),
            can_shrink: self.phase != Phase::Working
                && self.profile.force_rest
                && self.profile.urgent_mode,
            movable: self.profile.movable,
            full_screen: self.profile.overlay_coverage >= 1.0,
        }
    }

    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    fn can_postpone(&self) -> bool {
        self.phase == Phase::Resting
            && !self.voluntary
            && self.postpones_used < self.profile.max_postpones
    }

    fn can_claim_meeting(&self) -> bool {
        self.phase == Phase::Resting && !self.voluntary && self.meeting_app.is_some()
    }

    fn begin_rest(&mut self, now: Instant, voluntary: bool) {
        self.phase = Phase::Resting;
        self.voluntary = voluntary;
        self.held_by = None;
        self.meeting_app = None;
        self.away_remaining = None;
        self.started = now;
        self.deadline = now + self.rest;
    }

    fn begin_work(&mut self, now: Instant, length: Duration) {
        self.phase = Phase::Working;
        self.postponed = false;
        self.voluntary = false;
        self.held_by = None;
        self.meeting_app = None;
        self.away_remaining = None;
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
        assert!(!t.start_work(t0 + 61 * MIN), "Cannot start work before the break is over");

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
        assert!(t.snapshot(now).can_postpone, "Postponements should reset after a full break");
    }

    #[test]
    fn postponed_flag_lasts_until_next_break() {
        let t0 = Instant::now();
        let mut t = timer(t0);
        assert!(!t.snapshot(t0).postponed);
        let now = t0 + 60 * MIN;
        t.tick(now);
        assert!(!t.snapshot(now).postponed, "A break isn't a postponement");
        assert!(t.postpone(now));
        assert!(t.snapshot(now).postponed);
        assert_eq!(t.tick(now + 5 * MIN), Some(TimerEvent::RestStarted));
        assert!(!t.snapshot(now + 5 * MIN).postponed);

        // A long absence during a postponement counts as a full break and ends it
        assert!(t.postpone(now + 5 * MIN));
        let now = now + 20 * MIN;
        t.idle(now, 15 * MIN);
        assert!(!t.snapshot(now).postponed);
    }

    #[test]
    fn voluntary_rest_cannot_be_postponed() {
        let t0 = Instant::now();
        let mut t = timer(t0);
        assert!(t.rest_now(t0 + MIN));
        assert!(!t.rest_now(t0 + MIN), "Already on a break");
        assert!(!t.snapshot(t0 + MIN).can_postpone);
        assert!(!t.postpone(t0 + MIN));
    }

    #[test]
    fn held_rest_starts_once_app_leaves() {
        let t0 = Instant::now();
        let mut t = timer(t0);
        assert!(!t.rest_due(t0 + 59 * MIN));
        let now = t0 + 60 * MIN;
        assert!(t.rest_due(now));

        t.hold(Some("Zoom".into()));
        assert_eq!(t.tick(now), None);
        let snap = t.snapshot(now + 20 * MIN);
        assert_eq!(snap.phase, Phase::Working);
        assert_eq!(snap.held_by.as_deref(), Some("Zoom"));

        t.hold(None);
        assert_eq!(t.tick(now + 20 * MIN), Some(TimerEvent::RestStarted));
        assert_eq!(t.snapshot(now + 20 * MIN).held_by, None);
    }

    #[test]
    fn meeting_snooze_does_not_use_postpones() {
        let t0 = Instant::now();
        let mut t = timer(t0);
        let now = t0 + 60 * MIN;
        assert_eq!(t.tick(now), Some(TimerEvent::RestStarted));
        assert!(!t.in_meeting(now), "No meeting software running");

        t.set_meeting_app(Some("Zoom".into()));
        assert_eq!(t.snapshot(now).meeting_app.as_deref(), Some("Zoom"));
        assert!(t.in_meeting(now));
        let snap = t.snapshot(now);
        assert_eq!(snap.phase, Phase::Working);
        assert_eq!(snap.remaining_secs, MEETING_SNOOZE.as_secs());
        assert_eq!(snap.postpones_left, 3);
        assert_eq!(snap.meeting_app, None);

        // Must detect again on the next break
        assert_eq!(t.tick(now + 15 * MIN), Some(TimerEvent::RestStarted));
        assert!(!t.in_meeting(now + 15 * MIN));
    }

    #[test]
    fn voluntary_rest_has_no_meeting_option() {
        let t0 = Instant::now();
        let mut t = timer(t0);
        assert!(t.rest_now(t0));
        t.set_meeting_app(Some("Zoom".into()));
        assert_eq!(t.snapshot(t0).meeting_app, None);
        assert!(!t.in_meeting(t0));
    }

    #[test]
    fn skip_rest_works_mid_rest() {
        let t0 = Instant::now();
        let mut t = timer(t0);
        assert!(!t.skip_rest(t0), "Nothing to skip while working");
        assert!(t.rest_now(t0 + MIN));
        assert!(t.skip_rest(t0 + 2 * MIN));
        assert_eq!(t.snapshot(t0 + 2 * MIN).phase, Phase::Working);
    }

    #[test]
    fn apply_config_restarts_work_only_when_needed() {
        let t0 = Instant::now();
        let mut t = timer(t0);
        let now = t0 + 10 * MIN;

        // Only the break duration changed: work countdown unchanged
        let cfg = Config {
            rest_minutes: 3.0,
            ..Config::default()
        };
        t.apply_config(&cfg, now);
        assert_eq!(t.snapshot(now).remaining_secs, 50 * 60);

        // Work duration changed: restart with the new duration
        let cfg = Config {
            work_minutes: 25.0,
            ..cfg
        };
        t.apply_config(&cfg, now);
        assert_eq!(t.snapshot(now).remaining_secs, 25 * 60);

        // The new break duration takes effect on the next break
        let now = now + 25 * MIN;
        assert_eq!(t.tick(now), Some(TimerEvent::RestStarted));
        assert_eq!(t.snapshot(now).total_secs, 3 * 60);

        // Changing the work duration during a postponement doesn't interrupt its countdown
        assert!(t.postpone(now));
        let cfg = Config {
            work_minutes: 30.0,
            ..cfg
        };
        t.apply_config(&cfg, now + MIN);
        assert_eq!(t.snapshot(now + MIN).remaining_secs, 4 * 60);
    }

    #[test]
    fn switching_level_takes_effect() {
        let t0 = Instant::now();
        let mut t = timer(t0);
        let cfg = Config {
            level: "high".into(),
            ..Config::default()
        };
        t.apply_config(&cfg, t0);
        t.tick(t0 + 60 * MIN);
        let snap = t.snapshot(t0 + 60 * MIN);
        assert!(snap.can_postpone);
        assert_eq!(snap.postpones_left, 1);
        assert!(!snap.can_shrink);
        assert!(snap.forced);
        assert!(!snap.can_skip);
        assert!(!t.dismiss(t0 + 60 * MIN), "A forced break cannot be dismissed");
        assert!(!t.skip(t0 + 60 * MIN), "The high level cannot skip");
    }

    #[test]
    fn forced_break_can_be_skipped_when_allowed() {
        let t0 = Instant::now();
        let cfg = Config {
            level: "Mine".into(),
            profiles: vec![Profile {
                name: "Mine".into(),
                allow_skip: true,
                ..Profile::default()
            }],
            ..Config::default()
        };
        let mut t = Timer::new(&cfg, t0);
        assert!(!t.skip(t0), "Nothing to skip while working");
        assert!(!t.snapshot(t0).can_skip);
        let now = t0 + 60 * MIN;
        assert_eq!(t.tick(now), Some(TimerEvent::RestStarted));
        let snap = t.snapshot(now);
        assert!(snap.forced);
        assert!(snap.can_skip);
        assert!(!t.dismiss(now), "Allowing skip still does not allow dismissing the overlay");
        assert!(t.skip(now + MIN));
        let snap = t.snapshot(now + MIN);
        assert_eq!(snap.phase, Phase::Working);
        assert_eq!(snap.remaining_secs, 60 * 60);
    }

    #[test]
    fn custom_profile_postpones() {
        let t0 = Instant::now();
        let cfg = Config {
            level: "Mine".into(),
            profiles: vec![Profile {
                name: "Mine".into(),
                max_postpones: 1,
                ..Profile::default()
            }],
            ..Config::default()
        };
        let mut t = Timer::new(&cfg, t0);
        let now = t0 + 60 * MIN;
        t.tick(now);
        assert!(t.postpone(now));
        t.tick(now + 5 * MIN);
        assert!(!t.snapshot(now + 5 * MIN).can_postpone);
    }

    #[test]
    fn unforced_break_can_be_dismissed() {
        let t0 = Instant::now();
        let cfg = Config {
            level: "low".into(),
            ..Config::default()
        };
        let mut t = Timer::new(&cfg, t0);
        assert!(!t.dismiss(t0), "No overlay to dismiss while working");
        let now = t0 + 60 * MIN;
        assert_eq!(t.tick(now), Some(TimerEvent::RestStarted));
        let snap = t.snapshot(now);
        assert!(!snap.forced);
        assert!(snap.can_skip);
        assert!(!snap.can_shrink);
        assert!(t.dismiss(now + MIN));
        let snap = t.snapshot(now + MIN);
        assert_eq!(snap.phase, Phase::Working);
        assert_eq!(snap.remaining_secs, 60 * 60);
    }

    #[test]
    fn short_absence_pauses_countdown() {
        let t0 = Instant::now();
        let mut t = timer(t0);
        // Away after 20 minutes of work, crossing the 5-minute threshold after 6 minutes: none of the 6 minutes away count
        let left = t0 + 20 * MIN;
        let mut now = left + 6 * MIN;
        t.idle(now, now - left);
        assert_eq!(t.snapshot(now).remaining_secs, 40 * 60);
        assert_eq!(t.snapshot(now).total_secs, 60 * 60);

        now += 2 * MIN;
        t.idle(now, now - left);
        assert_eq!(t.tick(now), None);
        assert_eq!(t.snapshot(now).remaining_secs, 40 * 60);

        // On return, the countdown continues from 40 minutes
        t.idle(now, Duration::ZERO);
        now += 40 * MIN;
        t.idle(now, Duration::ZERO);
        assert_eq!(t.tick(now), Some(TimerEvent::RestStarted));
    }

    #[test]
    fn brief_inactivity_still_counts_as_work() {
        let t0 = Instant::now();
        let mut t = timer(t0);
        let now = t0 + 30 * MIN;
        t.idle(now, 4 * MIN);
        assert_eq!(t.snapshot(now).remaining_secs, 30 * 60);
    }

    #[test]
    fn long_absence_counts_as_full_rest() {
        let t0 = Instant::now();
        let mut t = timer(t0);
        let mut now = t0 + 60 * MIN;
        assert_eq!(t.tick(now), Some(TimerEvent::RestStarted));
        assert!(t.postpone(now));

        // Away for 10 minutes (one break length) during a postponement: full countdown on return, postponements reset
        let left = now + MIN;
        now = left + 10 * MIN;
        t.idle(now, now - left);
        assert_eq!(t.tick(now), None);
        let snap = t.snapshot(now);
        assert_eq!(snap.remaining_secs, 60 * 60);
        assert_eq!(snap.postpones_left, 3);

        now += 30 * MIN;
        t.idle(now, now - left);
        t.idle(now, Duration::ZERO);
        assert_eq!(t.snapshot(now).remaining_secs, 60 * 60);
    }

    #[test]
    fn idle_does_not_affect_rest() {
        let t0 = Instant::now();
        let mut t = timer(t0);
        assert!(t.rest_now(t0));
        t.idle(t0 + 9 * MIN, 9 * MIN);
        assert_eq!(t.snapshot(t0 + 9 * MIN).phase, Phase::Resting);
        assert_eq!(t.tick(t0 + 10 * MIN), Some(TimerEvent::RestFinished));
        t.idle(t0 + 30 * MIN, 30 * MIN);
        assert_eq!(t.snapshot(t0 + 30 * MIN).phase, Phase::RestOver);
    }

    #[test]
    fn idle_detection_can_be_disabled() {
        let t0 = Instant::now();
        let cfg = Config {
            idle_minutes: 0.0,
            ..Config::default()
        };
        let mut t = Timer::new(&cfg, t0);
        let now = t0 + 60 * MIN;
        t.idle(now, 30 * MIN);
        assert_eq!(t.tick(now), Some(TimerEvent::RestStarted));
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
