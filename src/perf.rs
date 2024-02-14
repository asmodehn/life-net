use std::collections::VecDeque;
use std::time::{Duration, Instant};

// #[allow(dead_code)]
// pub(crate) struct PerfCounter {
//     last_second: Instant,
//     call_number_since_last_second: u32,
//     last_cps: f32,
// }
//
// impl PerfCounter {
//     #[allow(dead_code)]
//     pub fn new() -> PerfCounter {
//         PerfCounter {
//             last_second: Instant::now(),
//             call_number_since_last_second: 0,
//             last_cps: 0.,
//         }
//     }
//
//     #[allow(dead_code)]
//     pub fn incr(self: &mut PerfCounter) {
//         if self.last_second.elapsed() > Duration::new(1, 0) {
//             self.last_cps = self.call_number_since_last_second as f32
//                 / self.last_second.elapsed().as_secs_f32();
//             self.last_second = Instant::now();
//             self.call_number_since_last_second = 0;
//         }
//         self.call_number_since_last_second += 1;
//     }
//
//     #[allow(dead_code)]
//     //TODO : clean up this (derive debug/display ??)
//     pub fn print(self: &PerfCounter) {
//         println!("{}", self.last_cps)
//     }
// }

#[derive(Debug, PartialEq, Default)]
pub(crate) struct DurationAverage {
    timed_since: Option<Instant>, //TODO: or passed in constructor/reset method ??
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

    pub fn timed_start(&mut self) {
        self.timed_since = Some(Instant::now());
    }

    pub fn timed_stop(&mut self) {
        self.durations
            .push_back(self.timed_since.unwrap().elapsed());
        if self.durations.len() > self.window_size as usize {
            self.durations.pop_front();
        }
        self.timed_since = None;
    }

    pub fn with_measured(self, duration: Duration) -> Self {
        let mut s = self;
        s.durations.push_back(duration);
        if s.durations.len() > s.window_size as usize {
            s.durations.pop_front();
        }
        s
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
        let instant = self.timed_since.unwrap();
        instant.elapsed()
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
