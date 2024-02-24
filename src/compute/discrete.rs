use crate::actor;
use crate::compute::Compute;
use crate::graphics::view::Viewable;
use crate::life::quad::Quad;
use crate::perf::{RunningAverage, Timer};
use macroquad::texture::Image;
use std::cmp::min;
use std::time::{Duration, Instant};

//TODO : make it just a struct somehow ??
pub(crate) struct DiscreteTime {
    pub world: Quad,
    pub max_update_rate: Option<f32>,
    average_duration: RunningAverage<Duration>,
    full_update_timer: Timer,
}

impl DiscreteTime {
    pub fn new(quad: Quad) -> DiscreteTime {
        DiscreteTime {
            world: quad,
            max_update_rate: None,
            average_duration: RunningAverage::<Duration>::default(),
            full_update_timer: Timer::default(),
        }
    }
    pub(crate) fn with_max_update_rate(self: Self, per_second: f32) -> Self {
        Self {
            max_update_rate: Some(per_second),
            ..self
        }
    }
}

impl Viewable for DiscreteTime {
    fn render(&self) -> &Image {
        // self.world.render()
        // no need to render with a quad
        &self.world.image
    }
}

impl Compute for DiscreteTime {
    fn update_timer_tick(&mut self) {
        self.average_duration
            .record(self.full_update_timer.elapsed_and_reset())
    }

    fn get_updates_per_second(&self) -> Option<f32> {
        self.average_duration
            .average()
            .and_then(|d| Some(1. / d.as_secs_f32()))
    }

    fn get_max_update_duration(&self) -> Option<Duration> {
        match self.max_update_rate {
            None => None,
            Some(update_rate) => Some(Duration::from_secs_f32(1. / update_rate)),
        }
    }

    fn is_ups_over_max(&self) -> bool {
        match (self.max_update_rate, self.get_updates_per_second()) {
            (None, _) => false,
            (_, None) => false,
            (Some(max_ups), Some(ups)) => ups >= max_ups as f32,
        }
    }
}

impl actor::Computable for DiscreteTime {
    fn update(&mut self, elapsed: Duration, constraint: Option<Duration>) {
        let update_constraint = match (constraint, self.get_max_update_duration()) {
            (c, None) => c,
            (None, mud) => mud,
            (Some(d), Some(upd)) => Some(min(d, upd)),
        };

        let until_closure = {
            let compute_timer = Instant::now();
            move |quad: &Quad| {
                //return bool to decide to stop or not (because of one compute constraint, or global update per second limit)
                update_constraint.is_some_and(|d| d <= compute_timer.elapsed())
            }
        };

        //one update maximum :
        // - if faster than required, lets rely on caller to call us again
        // - if slower than needed, let's get out early.
        self.world.compute_once_or_until(until_closure);

        //if update was finished, increment timer
        if self.world.is_updated() {
            self.update_timer_tick();
        }
    }
}

//TODO : test timers behavior (via injection of deterministic timer...)
