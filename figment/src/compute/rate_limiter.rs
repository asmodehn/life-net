use crate::compute::running_average::RunningAverage;
use std::cmp::min;
use std::time::{Duration, Instant};

pub(crate) struct RateLimiter {
    pub(crate) max_duration: Option<Duration>,
    pub(crate) average_duration: RunningAverage<Duration>, // TODO : This should be passed as argument in functions that need it..
}

impl Default for RateLimiter {
    fn default() -> Self {
        Self {
            max_duration: None,
            average_duration: RunningAverage::<Duration>::new(60),
        }
    }
}
impl RateLimiter {
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

    //TODO : adaptative, PID, or so ?? => benchmark needed !
    pub fn with_constraint(&self, constraint: Duration) -> Option<Duration> {
        self.max_duration.and_then(|md| Some(min(constraint, md)))
    }

    pub fn as_until_closure(&self) -> impl Fn() -> bool {
        let compute_timer = Instant::now();
        let max_duration = self.max_duration;

        move || {
            //return bool to decide to stop or not (because of max_duration constraint)
            max_duration.is_some_and(|d| d <= compute_timer.elapsed())
        }
    }

    pub fn record_duration(&mut self, duration: Duration) {
        self.average_duration.record(duration);
    }
}

//TODO : test timers behavior (via injection of deterministic timer...)
