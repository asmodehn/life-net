use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

//encapsulating often used,hidden, mutating value...
#[derive(Debug, PartialEq)]
pub(crate) struct Timer {
    since: Cell<Instant>,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            since: Cell::new(Instant::now()),
        }
    }
}

impl Timer {
    pub fn elapsed_and_reset(&self) -> Duration {
        let elapsed = self.elapsed();
        self.since.replace(Instant::now());
        elapsed
    }

    pub fn elapsed(&self) -> Duration {
        self.since.get().elapsed()
    }
}

//TODO : RunningAverage<T>

#[derive(Debug, PartialEq, Default)]
pub(crate) struct DurationAverage {
    timed_since: Option<Timer>,
    durations: VecDeque<Duration>,
    window_size: u16, // to never overflow usize (on any platform)
}

//TODO : rename to have average() an explicit function...
impl DurationAverage {
    pub fn default() -> Self {
        DurationAverage {
            timed_since: None,
            durations: VecDeque::new(),
            window_size: 1,
        }
    }
    pub fn new(window_size: u16) -> Self {
        DurationAverage {
            window_size,
            durations: VecDeque::with_capacity(window_size as usize),
            ..DurationAverage::default()
        }
    }

    //TODO : review the start/ stop API ...
    pub fn timed_start(&mut self) {
        self.timed_since = Some(Timer::default());
    }

    pub fn timed_stop(&mut self) {
        let elapsed = self.timed_elapsed();

        self.durations.push_back(elapsed);
        if self.durations.len() > self.window_size as usize {
            self.durations.pop_front();
        }
        self.timed_since = None;
    }

    pub fn with_measured(self, duration: Duration) -> Self {
        let mut s = self;
        s.record(duration);
        s
    }

    pub fn record(&mut self, duration: Duration) {
        self.durations.push_back(duration);
        if self.durations.len() > self.window_size as usize {
            self.durations.pop_front();
        }
    }

    pub fn avg(&self) -> Option<Duration> {
        let measurements_sum = self.durations.clone().into_iter().sum::<Duration>();
        measurements_sum.checked_div(self.durations.len() as u32)
    }

    pub fn per_second(&self) -> Option<f32> {
        let measurements_sum = self.durations.clone().into_iter().sum::<Duration>();
        if measurements_sum.is_zero() {
            None
        } else {
            Some((self.durations.len() as f32) / (measurements_sum.as_secs_f32()))
        }
    }

    //TODO : maybe we just want to expose the internal instant instead somehow ?
    pub fn timed_elapsed(&self) -> Duration {
        let tmr = self.timed_since.as_ref().expect("timer must be started !");
        tmr.elapsed()
    }
}

#[cfg(test)]
mod tests {
    use crate::perf::DurationAverage;
    use std::time::Duration;

    #[test]
    fn default_duration_check() {
        assert!(DurationAverage::default().durations.is_empty());
        assert_eq!(DurationAverage::default().avg(), None);
        assert_eq!(DurationAverage::default().per_second(), None);
    }

    #[test]
    fn measurement_inside_window_ok() {
        let mut da = DurationAverage::new(5);
        da = da
            .with_measured(Duration::new(1, 0))
            .with_measured(Duration::new(2, 0))
            .with_measured(Duration::new(3, 0));

        assert_eq!(da.avg(), Some(Duration::new(2, 0)));
        assert_eq!(da.per_second(), Some(0.5))
    }

    #[test]
    fn measurement_outside_window_dropped() {
        let mut da = DurationAverage::new(5);
        da = da
            .with_measured(Duration::new(1, 0))
            .with_measured(Duration::new(2, 0))
            .with_measured(Duration::new(3, 0))
            .with_measured(Duration::new(4, 0))
            .with_measured(Duration::new(5, 0))
            .with_measured(Duration::new(6, 0));

        assert_eq!(da.avg(), Some(Duration::new(4, 0)));
        assert_eq!(da.per_second(), Some(0.25));

        assert_eq!(da.durations.len(), 5);
    }
}
