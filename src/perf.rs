use std::time::{Duration, Instant};

#[allow(dead_code)]
pub(crate) struct PerfCounter {
    last_second: Instant,
    call_number_since_last_second: u32,
    last_cps: f32,
}

impl PerfCounter {
    #[allow(dead_code)]
    pub fn new() -> PerfCounter {
        PerfCounter {
            last_second: Instant::now(),
            call_number_since_last_second: 0,
            last_cps: 0.,
        }
    }

    #[allow(dead_code)]
    pub fn incr(self: &mut PerfCounter) {
        if self.last_second.elapsed() > Duration::new(1, 0) {
            self.last_cps = self.call_number_since_last_second as f32
                / self.last_second.elapsed().as_secs_f32();
            self.last_second = Instant::now();
            self.call_number_since_last_second = 0;
        }
        self.call_number_since_last_second += 1;
    }

    #[allow(dead_code)]
    //TODO : clean up this (derive debug/display ??)
    pub fn print(self: &PerfCounter) {
        println!("{}", self.last_cps)
    }
}

#[derive(Debug, PartialEq, Default)]
pub(crate) struct DurationAverage {
    count: u32,
    duration: Duration,
    timed_since: Option<Instant>,
}

impl DurationAverage {
    pub fn timed_start(&mut self) {
        self.timed_since = Some(Instant::now());
    }

    pub fn timed_stop(&mut self) {
        let (new_count, overflowed) = self.count.overflowing_add(1);
        if overflowed {
            *self = DurationAverage::default(); // reset
        } else {
            self.count = new_count;
            self.duration += self.timed_since.unwrap().elapsed();
            self.timed_since = None;
        }
    }

    pub fn avg(&self) -> Option<Duration> {
        self.duration.checked_div(self.count)
    }

    //TODO : maybe we just want to expose the internal instant instead ?
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
    fn default_duration_average_check() {
        assert_eq!(
            DurationAverage::default(),
            DurationAverage {
                count: 0,
                duration: Duration::new(0, 0),
                timed_since: None,
            }
        );
        assert_eq!(DurationAverage::default().avg(), None)
    }
}
