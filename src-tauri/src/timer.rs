//! Work/break state machine. Independent of Tauri; every method takes the current time explicitly, for testability.

use crate::config::{Config, FixedTime, Profile};
use chrono::NaiveTime;
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
    /// A break is due but held off until this fixed break (HH:MM)
    pub deferred_until: Option<String>,
    /// Name of that fixed break: a built-in one (see config::BUILTIN_FIXED_BREAKS) or the user's own
    pub deferred_for: Option<String>,
    /// Name of the fixed break the next break will be, which remaining_secs then counts down to: a built-in one or the user's own
    pub next_fixed: Option<String>,
    /// Name of the fixed break this break is for, named in the overlay's title: a built-in one or the user's own
    pub fixed_break: Option<String>,
    /// Meeting software running when the break started; the overlay then offers "I'm in a meeting"
    pub meeting_app: Option<String>,
    /// Forced break; when false the overlay can be closed
    pub forced: bool,
    /// Offer "Skip this break" on the overlay
    pub can_skip: bool,
    /// Offer "Urgent? Shrink it" on the overlay
    pub can_shrink: bool,
    /// On a fixed break longer than a regular one: how long until "Keep using the computer" can be clicked, 0 once it can
    pub keep_using_in_secs: Option<u64>,
    /// While shrunk, how long until the overlay restores itself; filled in by lib.rs, which tracks urgent mode
    pub urgent_left_secs: Option<u64>,
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
    /// Enabled fixed breaks; a break falling due just before one waits for it
    fixed_times: Vec<FixedTime>,

    phase: Phase,
    started: Instant,
    deadline: Instant,
    postpones_used: u32,
    /// The work countdown is a postponed break
    postponed: bool,
    /// This break was started by the user; voluntary breaks can't be postponed
    voluntary: bool,
    held_by: Option<String>,
    /// The due break waits for this fixed break
    deferred: Option<FixedTime>,
    /// Local time of day at the last defer call, to notice when a fixed break's time comes
    last_clock: Option<NaiveTime>,
    /// A fixed break whose time has come but hasn't started yet (e.g. held by a meeting app), and the time left of it
    fixed_due: Option<(FixedTime, Duration)>,
    /// This break is a fixed break; it can't be postponed, as postponing would end it early
    fixed_rest: bool,
    /// The fixed break this break is for. Also set while a break that waited for one is about to start,
    /// which at the end of the day is a regular break, not a fixed one
    rest_for: Option<String>,
    /// When the fixed break the user went back to the computer during ends; that time isn't work
    free_until: Option<Instant>,
    /// Away from the computer past the idle threshold; a fixed break then doesn't start, the time away is the break
    away: bool,
    meeting_app: Option<String>,
    /// Remaining work time frozen while the user is away
    away_remaining: Option<Duration>,
    /// When the break was paused for urgent mode (overlay shrunk); the break starts over, and time spent shrunk doesn't count as rest
    paused_at: Option<Instant>,
}

impl Timer {
    pub fn new(cfg: &Config, now: Instant) -> Self {
        Self {
            work: cfg.work(),
            rest: cfg.rest(),
            postpone: cfg.postpone(),
            profile: cfg.profile(),
            idle_threshold: cfg.idle(),
            fixed_times: cfg.fixed_times(),
            phase: Phase::Working,
            started: now,
            deadline: now + cfg.work(),
            postpones_used: 0,
            postponed: false,
            voluntary: false,
            held_by: None,
            deferred: None,
            last_clock: None,
            fixed_due: None,
            fixed_rest: false,
            rest_for: None,
            free_until: None,
            away: false,
            meeting_app: None,
            away_remaining: None,
            paused_at: None,
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
            self.away = false;
            self.away_remaining = None;
            return;
        };
        self.away = idle >= threshold;
        // Not working yet, and watching a video looks like being away
        if !self.free_left(now).is_zero() {
            return;
        }
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

    /// The work countdown has ended, or a fixed break's time has come, and a break is about to start
    pub fn rest_due(&self, now: Instant) -> bool {
        self.phase == Phase::Working && (now >= self.deadline || self.fixed_due.is_some())
    }

    /// Sets the exempt app holding off the break, None for no hold. Must be called before tick
    pub fn hold(&mut self, app: Option<String>) {
        self.held_by = app;
    }

    /// Reports the local time of day; holds off a due break while it falls in the quiet window before a fixed break,
    /// waiting for the nearest one when windows overlap, and starts a fixed break once its time comes.
    /// Must be called after idle and before tick
    pub fn defer(&mut self, now: Instant, clock: NaiveTime) {
        // Not when the clock first comes in (e.g. the app starts during lunch)
        if let Some(prev) = self.last_clock.replace(clock) {
            if let Some(f) = self.fixed_times.iter().find(|f| f.rest.is_some() && f.reached(prev, clock)) {
                self.fixed_due = Some((f.clone(), Duration::ZERO));
            }
            if let Some(f) = self.deferred.as_ref().filter(|f| f.reached(prev, clock)) {
                self.rest_for = Some(f.name.clone());
            }
        }
        // Stays due until it would have ended, e.g. while a meeting app holds it off
        self.fixed_due = self
            .fixed_due
            .take()
            .filter(|_| !self.away)
            .and_then(|(f, _)| f.rest_left(clock).map(|left| (f, left)));
        // Already on a break: it lasts until the fixed break ends, even if it was over and waiting for "Start working"
        if let (Phase::Resting | Phase::RestOver, Some((f, left))) = (self.phase, &self.fixed_due) {
            if self.phase == Phase::RestOver {
                self.phase = Phase::Resting;
                self.started = now;
            }
            self.deadline = self.deadline.max(now + *left);
            self.fixed_rest = true;
            self.rest_for = Some(f.name.clone());
            self.fixed_due = None;
        }
        self.deferred = if self.rest_due(now) && self.fixed_due.is_none() {
            self.fixed_times
                .iter()
                .filter_map(|f| Some((f.wait(clock)?, f)))
                .min_by_key(|(left, _)| *left)
                .map(|(_, f)| f.clone())
        } else {
            None
        };
    }

    pub fn tick(&mut self, now: Instant) -> Option<TimerEvent> {
        // Back at the computer during a fixed break: its end is still announced, as it is from the overlay
        if self.free_until.is_some_and(|end| now >= end) {
            self.free_until = None;
            return Some(TimerEvent::RestFinished);
        }
        match self.phase {
            Phase::Working if !self.rest_due(now) || self.held_by.is_some() || self.deferred.is_some() => None,
            Phase::Working => {
                let fixed = self.fixed_due.take();
                let waited_for = self.rest_for.take();
                self.begin_rest(now, false);
                self.rest_for = waited_for;
                if let Some((f, left)) = fixed {
                    self.deadline = now + left;
                    self.fixed_rest = true;
                    self.rest_for = Some(f.name);
                }
                Some(TimerEvent::RestStarted)
            }
            Phase::Resting if self.paused_at.is_some() => None,
            _ if now < self.deadline => None,
            Phase::Resting => {
                self.phase = Phase::RestOver;
                Some(TimerEvent::RestFinished)
            }
            Phase::RestOver => None,
        }
    }

    /// Urgent mode (overlay shrunk) interrupts the break: it starts over from its full length and stays paused there,
    /// counting down again once restored. Only a running break pauses; one that's over just waits for "Start working"
    pub fn pause_rest(&mut self, now: Instant, paused: bool) {
        if !paused {
            if let Some(at) = self.paused_at.take() {
                let shift = now.saturating_duration_since(at);
                self.started += shift;
                self.deadline += shift;
            }
        } else if self.phase == Phase::Resting && self.paused_at.is_none() {
            let total = self.deadline - self.started;
            self.started = now;
            self.deadline = now + total;
            self.paused_at = Some(now);
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

    /// "Keep using the computer" clicked once the regular break length of a longer fixed break has passed.
    /// The rest of the fixed break is free time, so the work countdown only starts when it ends
    pub fn keep_using(&mut self, now: Instant) -> bool {
        if self.keep_using_in(now) != Some(Duration::ZERO) {
            return false;
        }
        let end = self.deadline;
        self.postpones_used = 0;
        self.begin_work(now, end.saturating_duration_since(now) + self.work);
        self.free_until = Some(end);
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
        if self.keep_using(now) {
            return true;
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
        self.fixed_times = cfg.fixed_times();
        // Before postponements are reset, postpones_used > 0 means we're in a post-postponement countdown
        if self.phase == Phase::Working && self.postpones_used == 0 && work_changed {
            self.begin_work(now, self.free_left(now) + self.work);
        }
    }

    pub fn snapshot(&self, now: Instant) -> Snapshot {
        // A paused break shows the time left when it was paused
        let now = self.paused_at.unwrap_or(now);
        let remaining = self.deadline.saturating_duration_since(now);
        let next_fixed = self.next_fixed(now);
        Snapshot {
            phase: self.phase,
            remaining_secs: ceil_secs(next_fixed.as_ref().map_or(remaining, |(left, _)| *left)),
            total_secs: ceil_secs(self.deadline - self.started),
            can_postpone: self.can_postpone(),
            postpones_left: self.profile.max_postpones.saturating_sub(self.postpones_used),
            postponed: self.phase == Phase::Working && self.postponed,
            held_by: self.held_by.clone(),
            deferred_until: self.deferred.as_ref().map(|f| f.at.format("%H:%M").to_string()),
            deferred_for: self.deferred.as_ref().map(|f| f.name.clone()),
            next_fixed: next_fixed.map(|(_, f)| f.name.clone()),
            fixed_break: (self.phase != Phase::Working).then(|| self.rest_for.clone()).flatten(),
            meeting_app: self
                .can_claim_meeting()
                .then(|| self.meeting_app.clone())
                .flatten(),
            forced: self.profile.force_rest,
            can_skip: self.phase != Phase::Working && self.profile.can_skip(),
            // Not needed once the computer is free to use
            can_shrink: self.phase != Phase::Working
                && self.profile.force_rest
                && self.profile.urgent_mode
                && self.keep_using_in(now) != Some(Duration::ZERO),
            keep_using_in_secs: self.keep_using_in(now).map(ceil_secs),
            urgent_left_secs: None,
            movable: self.profile.movable,
            full_screen: self.profile.overlay_coverage >= 1.0,
        }
    }

    pub fn profile(&self) -> &Profile {
        &self.profile
    }

    /// The fixed break the next break will be and the time left until it: one that starts before the work countdown ends,
    /// or one the countdown's break will wait for
    fn next_fixed(&self, now: Instant) -> Option<(Duration, &FixedTime)> {
        if self.phase != Phase::Working || self.rest_due(now) {
            return None;
        }
        let clock = self.last_clock?;
        let work_left = self.deadline.saturating_duration_since(now);
        self.fixed_times
            .iter()
            .filter_map(|f| Some((f.next(clock, work_left)?, f)))
            .min_by_key(|(left, _)| *left)
    }

    fn can_postpone(&self) -> bool {
        self.phase == Phase::Resting
            && !self.voluntary
            && !self.fixed_rest
            && self.postpones_used < self.profile.max_postpones
    }

    fn can_claim_meeting(&self) -> bool {
        self.phase == Phase::Resting && !self.voluntary && self.meeting_app.is_some()
    }

    /// How long until the user can go back to the computer during a fixed break, which takes a regular break's length.
    /// None when the break ends first. While shrunk the break is on hold, so this doesn't count down
    fn keep_using_in(&self, now: Instant) -> Option<Duration> {
        let free_from = self.started + self.rest;
        (self.phase == Phase::Resting && self.fixed_rest && free_from < self.deadline)
            .then(|| free_from.saturating_duration_since(self.paused_at.unwrap_or(now)))
    }

    /// What's left of the fixed break the user went back to the computer during
    fn free_left(&self, now: Instant) -> Duration {
        self.free_until.map_or(Duration::ZERO, |end| end.saturating_duration_since(now))
    }

    fn begin_rest(&mut self, now: Instant, voluntary: bool) {
        self.phase = Phase::Resting;
        self.voluntary = voluntary;
        self.held_by = None;
        self.deferred = None;
        self.fixed_due = None;
        self.fixed_rest = false;
        self.rest_for = None;
        self.free_until = None;
        self.away = false;
        self.meeting_app = None;
        self.away_remaining = None;
        self.paused_at = None;
        self.started = now;
        self.deadline = now + self.rest;
    }

    fn begin_work(&mut self, now: Instant, length: Duration) {
        self.phase = Phase::Working;
        self.postponed = false;
        self.voluntary = false;
        self.held_by = None;
        self.deferred = None;
        self.fixed_rest = false;
        self.rest_for = None;
        self.meeting_app = None;
        self.away_remaining = None;
        self.paused_at = None;
        self.started = now;
        self.deadline = now + length;
    }
}

pub(crate) fn ceil_secs(d: Duration) -> u64 {
    d.as_millis().div_ceil(1000) as u64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::FixedBreak;

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
    fn shrunk_time_does_not_count_as_rest() {
        let t0 = Instant::now();
        let mut t = timer(t0);
        let rest = t0 + 60 * MIN;
        t.tick(rest);

        t.pause_rest(rest + 8 * MIN, true);
        let snap = t.snapshot(rest + 20 * MIN);
        assert_eq!(t.tick(rest + 20 * MIN), None, "A paused break doesn't finish");
        assert_eq!(snap.remaining_secs, 10 * 60, "Shrinking starts the break over");
        assert_eq!(snap.total_secs, 10 * 60);

        t.pause_rest(rest + 22 * MIN, false);
        assert_eq!(t.snapshot(rest + 22 * MIN).remaining_secs, 10 * 60);
        assert_eq!(t.tick(rest + 31 * MIN), None);
        assert_eq!(t.tick(rest + 32 * MIN), Some(TimerEvent::RestFinished));
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

    /// Config with the built-in fixed breaks enabled, plus `extra`
    fn with_fixed(quiet_minutes: f64, extra: Vec<FixedBreak>) -> Config {
        let mut cfg = Config::default();
        for b in &mut cfg.fixed_breaks {
            b.enabled = true;
            b.quiet_minutes = quiet_minutes;
        }
        cfg.fixed_breaks.extend(extra);
        cfg
    }

    fn at(h: u32, m: u32) -> NaiveTime {
        NaiveTime::from_hms_opt(h, m, 0).unwrap()
    }

    #[test]
    fn rest_waits_for_fixed_break() {
        let t0 = Instant::now();
        let mut t = Timer::new(&with_fixed(30.0, vec![]), t0);

        // Not due yet: nothing is deferred even inside the window
        t.defer(t0 + 50 * MIN, at(17, 40));
        assert_eq!(t.snapshot(t0 + 50 * MIN).deferred_until, None);

        // Due at 17:40, within 30 minutes of the end of the day at 18:00: waits
        let now = t0 + 60 * MIN;
        t.defer(now, at(17, 40));
        assert_eq!(t.tick(now), None);
        let snap = t.snapshot(now);
        assert_eq!(snap.phase, Phase::Working);
        assert_eq!(snap.deferred_until.as_deref(), Some("18:00"));
        assert_eq!(snap.deferred_for.as_deref(), Some("work_end"));

        // 18:00: the break goes ahead
        let now = now + 20 * MIN;
        t.defer(now, at(18, 0));
        assert_eq!(t.tick(now), Some(TimerEvent::RestStarted));
        assert_eq!(t.snapshot(now).deferred_until, None);
        assert_eq!(t.snapshot(now).fixed_break.as_deref(), Some("work_end"));

        // Voluntary breaks aren't affected
        assert_eq!(t.tick(now + 10 * MIN), Some(TimerEvent::RestFinished));
        assert_eq!(t.snapshot(now + 10 * MIN).fixed_break.as_deref(), Some("work_end"));
        assert!(t.start_work(now + 10 * MIN));
        assert_eq!(t.snapshot(now + 10 * MIN).fixed_break, None);
        t.defer(now + 11 * MIN, at(17, 50));
        assert!(t.rest_now(now + 11 * MIN));
        assert_eq!(t.snapshot(now + 11 * MIN).fixed_break, None);
    }

    #[test]
    fn lunch_and_custom_breaks_defer_too() {
        let t0 = Instant::now();
        let now = t0 + 60 * MIN;
        let tea = FixedBreak {
            name: "Tea".into(),
            time: "12:10".into(),
            rest_minutes: 15.0,
            ..FixedBreak::default()
        };
        let mut t = Timer::new(&with_fixed(30.0, vec![tea.clone()]), t0);
        t.defer(now, at(11, 45));
        let snap = t.snapshot(now);
        assert_eq!(snap.deferred_until.as_deref(), Some("12:00"));
        assert_eq!(snap.deferred_for.as_deref(), Some("lunch"), "Overlapping windows wait for the nearest");

        // A custom one alone: waits for it, then its own break starts at its time
        let cfg = Config {
            fixed_breaks: vec![tea],
            ..Config::default()
        };
        let mut t = Timer::new(&cfg, t0);
        t.defer(now, at(12, 5));
        assert_eq!(t.tick(now), None);
        assert_eq!(t.snapshot(now).deferred_for.as_deref(), Some("Tea"));
        t.defer(now + 5 * MIN, at(12, 10));
        assert_eq!(t.tick(now + 5 * MIN), Some(TimerEvent::RestStarted));
        assert_eq!(t.snapshot(now + 5 * MIN).remaining_secs, 15 * 60);
        assert_eq!(t.snapshot(now + 5 * MIN).fixed_break.as_deref(), Some("Tea"));
    }

    #[test]
    fn countdown_shows_the_fixed_break_that_comes_first() {
        let t0 = Instant::now();
        let mut t = Timer::new(&with_fixed(5.0, vec![]), t0);

        // Lunch comes before the work countdown ends
        t.defer(t0, at(11, 50));
        let snap = t.snapshot(t0);
        assert_eq!(snap.next_fixed.as_deref(), Some("lunch"));
        assert_eq!(snap.remaining_secs, 10 * 60);

        // The work countdown ends well before lunch
        t.defer(t0, at(10, 0));
        let snap = t.snapshot(t0);
        assert_eq!(snap.next_fixed, None);
        assert_eq!(snap.remaining_secs, t.work.as_secs());

        // work_end starts no break of its own, so a countdown ending after it comes first
        t.defer(t0, at(17, 50));
        assert_eq!(t.snapshot(t0).next_fixed, None);

        // The countdown ends in the quiet window before lunch, so that's when the break comes
        let work = t.work;
        t.defer(t0, at(12, 0) - chrono::Duration::from_std(work + 3 * MIN).unwrap());
        let snap = t.snapshot(t0);
        assert_eq!(snap.next_fixed.as_deref(), Some("lunch"));
        assert_eq!(snap.remaining_secs, (work + 3 * MIN).as_secs());

        // Likewise at the end of the day, though it's a regular break that then starts
        t.defer(t0, at(18, 0) - chrono::Duration::from_std(work + 3 * MIN).unwrap());
        assert_eq!(t.snapshot(t0).next_fixed.as_deref(), Some("work_end"));

        // Not during a break
        t.defer(t0, at(11, 50));
        t.rest_now(t0);
        assert_eq!(t.snapshot(t0).next_fixed, None);
    }

    #[test]
    fn fixed_break_starts_on_time() {
        let t0 = Instant::now();
        let lunch = FixedBreak {
            enabled: true,
            rest_minutes: 45.0,
            ..FixedBreak::builtin("lunch").unwrap()
        };
        let cfg = Config {
            fixed_breaks: vec![lunch],
            ..Config::default()
        };
        let mut t = Timer::new(&cfg, t0);

        // The first clock reading only sets the baseline, so starting the app during lunch doesn't start it
        t.defer(t0, at(11, 59));
        assert_eq!(t.tick(t0), None);

        // 12:00 comes in the middle of the work countdown: a 45-minute break starts that can't be postponed
        let now = t0 + MIN;
        t.defer(now, at(12, 0));
        assert_eq!(t.tick(now), Some(TimerEvent::RestStarted));
        let snap = t.snapshot(now);
        assert_eq!(snap.remaining_secs, 45 * 60);
        assert!(!snap.can_postpone);
        t.defer(now + MIN, at(12, 1));
        assert_eq!(t.tick(now + MIN), None, "Starts only once");
        assert_eq!(t.tick(now + 45 * MIN), Some(TimerEvent::RestFinished));
    }

    #[test]
    fn fixed_break_waits_for_meeting_and_extends_breaks() {
        let t0 = Instant::now();
        let mut t = Timer::new(&with_fixed(30.0, vec![]), t0);
        t.defer(t0, at(11, 50));

        // Held by a meeting app at 12:00; starts with the time left once it's gone
        let now = t0 + MIN;
        t.defer(now, at(12, 0));
        assert!(t.rest_due(now));
        t.hold(Some("Zoom".into()));
        assert_eq!(t.tick(now), None);
        let now = t0 + 11 * MIN;
        t.defer(now, at(12, 10));
        t.hold(None);
        assert_eq!(t.tick(now), Some(TimerEvent::RestStarted));
        assert_eq!(t.snapshot(now).remaining_secs, 20 * 60);
        assert_eq!(t.tick(now + 20 * MIN), Some(TimerEvent::RestFinished));

        // A regular break running at lunch time lasts until lunch ends
        let mut t = Timer::new(&with_fixed(30.0, vec![]), t0);
        t.defer(t0, at(11, 50));
        assert!(t.rest_now(t0));
        t.defer(t0 + 5 * MIN, at(12, 0));
        assert_eq!(t.snapshot(t0 + 5 * MIN).remaining_secs, 30 * 60);
        assert_eq!(t.snapshot(t0 + 5 * MIN).fixed_break.as_deref(), Some("lunch"));

        // Break over at 11:50, not back at work yet: at 12:00 it's a break again until lunch ends
        let mut t = Timer::new(&with_fixed(30.0, vec![]), t0);
        t.defer(t0, at(11, 40));
        assert!(t.rest_now(t0));
        assert_eq!(t.tick(t0 + 10 * MIN), Some(TimerEvent::RestFinished));
        t.defer(t0 + 20 * MIN, at(12, 0));
        let snap = t.snapshot(t0 + 20 * MIN);
        assert_eq!(snap.phase, Phase::Resting);
        assert_eq!(snap.remaining_secs, 30 * 60);
    }

    /// A 60-minute lunch with regular 10-minute breaks, started at 12:00
    fn on_long_lunch(cfg: Config, t0: Instant) -> Timer {
        let lunch = FixedBreak {
            enabled: true,
            rest_minutes: 60.0,
            ..FixedBreak::builtin("lunch").unwrap()
        };
        let cfg = Config {
            fixed_breaks: vec![lunch],
            ..cfg
        }
        .sanitized();
        let mut t = Timer::new(&cfg, t0);
        t.defer(t0, at(11, 59));
        t.defer(t0, at(12, 0));
        assert_eq!(t.tick(t0), Some(TimerEvent::RestStarted));
        t
    }

    #[test]
    fn computer_is_free_after_a_regular_break_length_of_a_fixed_break() {
        let t0 = Instant::now();
        let mut t = on_long_lunch(Config::default(), t0);

        let snap = t.snapshot(t0 + 4 * MIN);
        assert_eq!(snap.keep_using_in_secs, Some(6 * 60));
        assert!(snap.can_shrink);
        assert!(!t.keep_using(t0 + 4 * MIN), "The regular break length is forced");

        let now = t0 + 10 * MIN;
        let snap = t.snapshot(now);
        assert_eq!(snap.keep_using_in_secs, Some(0));
        assert_eq!(snap.remaining_secs, 50 * 60, "The overlay stays until clicked");
        assert!(!snap.can_shrink);
        assert_eq!(t.tick(now), None);
        assert!(t.keep_using(now));

        // Work only starts when lunch ends
        let snap = t.snapshot(now);
        assert_eq!(snap.phase, Phase::Working);
        assert_eq!(snap.remaining_secs, 50 * 60 + 60 * 60);
        assert_eq!(snap.keep_using_in_secs, None);

        // Watching a video: no input for long enough to count as away, or even as a full break
        let now = t0 + 40 * MIN;
        t.idle(now, 7 * MIN);
        assert_eq!(t.snapshot(now).remaining_secs, 20 * 60 + 60 * 60);
        t.idle(now, 25 * MIN);
        assert_eq!(t.snapshot(now).remaining_secs, 20 * 60 + 60 * 60);

        let now = t0 + 60 * MIN;
        t.idle(now, Duration::ZERO);
        assert_eq!(t.tick(now - MIN), None);
        assert_eq!(t.tick(now), Some(TimerEvent::RestFinished), "The end of lunch is still announced");
        assert_eq!(t.snapshot(now).phase, Phase::Working);
        assert_eq!(t.tick(now + MIN), None, "Announced only once");
        assert_eq!(t.tick(t0 + 119 * MIN), None);
        assert_eq!(t.tick(t0 + 120 * MIN), Some(TimerEvent::RestStarted));
    }

    #[test]
    fn end_of_fixed_break_is_announced_while_away() {
        let t0 = Instant::now();
        let mut t = on_long_lunch(Config::default(), t0);
        assert!(t.keep_using(t0 + 10 * MIN));
        let now = t0 + 60 * MIN;
        t.idle(now, 20 * MIN);
        assert_eq!(t.tick(now), Some(TimerEvent::RestFinished));
    }

    #[test]
    fn end_of_fixed_break_is_not_announced_after_another_break_took_over() {
        let t0 = Instant::now();
        let mut t = on_long_lunch(Config::default(), t0);
        assert!(t.keep_using(t0 + 10 * MIN));
        assert!(t.rest_now(t0 + 55 * MIN));
        assert_eq!(t.tick(t0 + 60 * MIN), None);
        assert_eq!(t.tick(t0 + 65 * MIN), Some(TimerEvent::RestFinished), "Only that break's own end");
    }

    #[test]
    fn fixed_break_not_left_runs_to_its_end() {
        let t0 = Instant::now();
        let mut t = on_long_lunch(Config::default(), t0);
        assert_eq!(t.tick(t0 + 59 * MIN), None);
        assert_eq!(t.tick(t0 + 60 * MIN), Some(TimerEvent::RestFinished));
        assert_eq!(t.snapshot(t0 + 60 * MIN).keep_using_in_secs, None);
        assert!(!t.keep_using(t0 + 60 * MIN));
    }

    #[test]
    fn fixed_break_no_longer_than_a_regular_one_stays_forced() {
        let t0 = Instant::now();
        let lunch = FixedBreak {
            enabled: true,
            rest_minutes: 10.0,
            ..FixedBreak::builtin("lunch").unwrap()
        };
        let cfg = Config {
            fixed_breaks: vec![lunch],
            ..Config::default()
        }
        .sanitized();
        let mut t = Timer::new(&cfg, t0);
        t.defer(t0, at(11, 59));
        t.defer(t0, at(12, 0));
        t.tick(t0);
        assert_eq!(t.snapshot(t0).keep_using_in_secs, None);
        assert_eq!(t.snapshot(t0 + 10 * MIN).keep_using_in_secs, None);

        // Regular breaks don't offer it either
        let mut t = timer(t0);
        t.tick(t0 + 60 * MIN);
        assert_eq!(t.snapshot(t0 + 65 * MIN).keep_using_in_secs, None);
    }

    #[test]
    fn regular_break_extended_by_a_fixed_break_counts_towards_the_forced_part() {
        let t0 = Instant::now();
        let mut t = Timer::new(&with_fixed(30.0, vec![]), t0);
        t.defer(t0, at(11, 55));
        assert!(t.rest_now(t0));
        t.defer(t0 + 5 * MIN, at(12, 0));
        assert_eq!(t.snapshot(t0 + 5 * MIN).keep_using_in_secs, Some(5 * 60));
    }

    #[test]
    fn shrinking_restarts_the_forced_part_of_a_fixed_break() {
        let t0 = Instant::now();
        let mut t = on_long_lunch(Config::default(), t0);
        t.pause_rest(t0 + 8 * MIN, true);
        assert_eq!(t.snapshot(t0 + 9 * MIN).keep_using_in_secs, Some(10 * 60));
        assert!(!t.keep_using(t0 + 20 * MIN), "Shrunk time doesn't count");
        t.pause_rest(t0 + 9 * MIN, false);
        assert_eq!(t.snapshot(t0 + 9 * MIN).keep_using_in_secs, Some(10 * 60));
    }

    #[test]
    fn closing_the_overlay_keeps_the_rest_of_a_fixed_break_free() {
        let t0 = Instant::now();
        let cfg = Config {
            level: "low".into(),
            ..Config::default()
        };
        let mut t = on_long_lunch(cfg, t0);
        assert!(t.dismiss(t0 + 15 * MIN));
        assert_eq!(t.snapshot(t0 + 15 * MIN).remaining_secs, 45 * 60 + 60 * 60);
    }

    #[test]
    fn changing_work_length_keeps_the_rest_of_a_fixed_break_free() {
        let t0 = Instant::now();
        let mut t = on_long_lunch(Config::default(), t0);
        assert!(t.keep_using(t0 + 10 * MIN));
        let cfg = Config {
            work_minutes: 30.0,
            ..Config::default()
        };
        t.apply_config(&cfg, t0 + 20 * MIN);
        assert_eq!(t.snapshot(t0 + 20 * MIN).remaining_secs, 40 * 60 + 30 * 60);
    }

    #[test]
    fn rest_now_ends_the_free_time() {
        let t0 = Instant::now();
        let mut t = on_long_lunch(Config::default(), t0);
        assert!(t.keep_using(t0 + 10 * MIN));
        assert!(t.rest_now(t0 + 20 * MIN));
        assert_eq!(t.tick(t0 + 30 * MIN), Some(TimerEvent::RestFinished));
        assert!(t.start_work(t0 + 30 * MIN));
        t.idle(t0 + 36 * MIN, 6 * MIN);
        assert_eq!(t.snapshot(t0 + 36 * MIN).remaining_secs, 60 * 60, "Being away pauses the countdown again");
    }

    #[test]
    fn no_fixed_break_while_away() {
        let t0 = Instant::now();
        let mut t = Timer::new(&with_fixed(30.0, vec![]), t0);
        t.defer(t0, at(11, 50));
        let now = t0 + 10 * MIN;
        t.idle(now, 6 * MIN);
        t.defer(now, at(12, 0));
        assert_eq!(t.tick(now), None);
        // Back at 12:05: the countdown carries on
        t.idle(now + 5 * MIN, Duration::ZERO);
        t.defer(now + 5 * MIN, at(12, 5));
        assert_eq!(t.tick(now + 5 * MIN), None);
    }

    #[test]
    fn rest_before_quiet_window_is_not_deferred() {
        let t0 = Instant::now();
        let mut t = Timer::new(&with_fixed(15.0, vec![]), t0);
        let now = t0 + 60 * MIN;
        t.defer(now, at(17, 40));
        assert_eq!(t.tick(now), Some(TimerEvent::RestStarted));

        // Disabled: never deferred
        let mut t = timer(t0);
        t.defer(now, at(17, 55));
        assert_eq!(t.tick(now), Some(TimerEvent::RestStarted));
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
