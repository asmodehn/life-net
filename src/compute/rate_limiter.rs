use crate::compute::running_average::RunningAverage;
use crate::life::quad::Quad;
use std::cmp::min;
use std::time::{Duration, Instant};

pub(crate) struct RateLimiter {
    pub(crate) max_duration: Option<Duration>,
    pub(crate) average_duration: RunningAverage<Duration>,
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self {
            max_duration: None,
            average_duration: RunningAverage::<Duration>::new(1),
        }
    }
}
impl RateLimiter {
    pub fn with_sample_window(self, window_size: u16) -> Self {
        Self {
            average_duration: RunningAverage::<Duration>::new(window_size),
            ..self
        }
    }

    pub fn with_maximum_duration(self, maxd: Duration) -> Self {
        Self {
            max_duration: Some(maxd),
            ..self
        }
    }

    pub fn with_maximum_rate(self, maximum_per_second: f32) -> Self {
        let max_duration: Duration = Duration::from_secs_f32(1. / maximum_per_second);
        Self {
            max_duration: Some(max_duration),
            ..self
        }
    }

    pub fn limit_rate(&self) -> Option<f32> {
        self.max_duration.and_then(|d| Some(d.as_secs_f32()))
    }

    pub fn window_size(&self) -> u16 {
        self.average_duration.window_size
    }

    //TODO : adaptative, PID, or so ?? => benchmark needed !
    pub fn with_constraint(&self, constraint: Duration) -> Option<Duration> {
        self.max_duration.and_then(|md| Some(min(constraint, md)))
    }

    pub fn as_until_closure(&self) -> impl Fn(&Quad) -> bool {
        let compute_timer = Instant::now();
        let max_duration = self.max_duration;

        move |_quad: &Quad| {
            //return bool to decide to stop or not (because of max_duration constraint)
            max_duration.is_some_and(|d| d <= compute_timer.elapsed())
        }
    }

    pub fn record_duration(&mut self, duration: Duration) {
        self.average_duration.record(duration);
    }
}
