use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::iter::Sum;
use std::ops::Div;
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

#[derive(Debug, PartialEq, Default)]
pub(crate) struct RunningAverage<T>
where
    T: Sum<T> + Copy + Div<u32>,
{
    durations: VecDeque<T>,
    window_size: u16, // to never overflow usize (on any platform)
}

impl<T> RunningAverage<T>
where
    T: Sum<T> + Copy + Div<u32>,
{
    pub fn default() -> Self {
        Self {
            // timed_since: None,
            durations: VecDeque::new(),
            window_size: 1,
        }
    }
    pub fn new(window_size: u16) -> Self {
        Self {
            window_size,
            durations: VecDeque::with_capacity(window_size as usize),
        }
    }

    pub fn with_measured(self, duration: T) -> Self {
        let mut s = self;
        s.record(duration);
        s
    }

    pub fn record(&mut self, duration: T) {
        self.durations.push_back(duration);
        if self.durations.len() > self.window_size as usize {
            self.durations.pop_front();
        }
    }

    pub fn average(&self) -> Option<<T as Div<u32>>::Output> {
        let measurements_sum: T = self.durations.clone().into_iter().sum::<T>();
        match self.durations.len() {
            0 => None,
            l => Some(measurements_sum.div(l as u32)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::perf::RunningAverage;
    use std::time::Duration;

    #[test]
    fn default_duration_check() {
        assert!(RunningAverage::<Duration>::default().durations.is_empty());
        assert_eq!(RunningAverage::<Duration>::default().average(), None);
    }

    #[test]
    fn measurement_inside_window_ok() {
        let mut da = RunningAverage::<Duration>::new(5);
        da = da
            .with_measured(Duration::new(1, 0))
            .with_measured(Duration::new(2, 0))
            .with_measured(Duration::new(3, 0));

        assert_eq!(da.average(), Some(Duration::new(2, 0)));
        // assert_eq!(da.per_second(), Some(0.5))
    }

    #[test]
    fn measurement_outside_window_dropped() {
        let mut da = RunningAverage::<Duration>::new(5);
        da = da
            .with_measured(Duration::new(1, 0))
            .with_measured(Duration::new(2, 0))
            .with_measured(Duration::new(3, 0))
            .with_measured(Duration::new(4, 0))
            .with_measured(Duration::new(5, 0))
            .with_measured(Duration::new(6, 0));

        assert_eq!(da.average(), Some(Duration::new(4, 0)));
        // assert_eq!(da.per_second(), Some(0.25));

        assert_eq!(da.durations.len(), 5);
    }
}
