use crate::compute::Compute;
use crate::graphics::view::Viewable;
use crate::life::quad::Quad;
use crate::perf::DurationAverage;
use macroquad::texture::Image;
use std::cmp::min;
use std::time::{Duration, Instant};

//TODO : make it just a trait somehow ??
pub(crate) struct DiscreteTime {
    pub world: Quad,
    pub max_update_rate: Option<f32>,
    average_duration: DurationAverage,
    full_update_timer: Option<Instant>,
}

impl DiscreteTime {
    pub fn new(quad: Quad) -> DiscreteTime {
        DiscreteTime {
            world: quad,
            max_update_rate: None,
            average_duration: DurationAverage::default(),
            full_update_timer: None,
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
        if self.full_update_timer.is_some() {
            self.average_duration
                .record(self.full_update_timer.unwrap().elapsed())
        }
        self.full_update_timer = Some(Instant::now());
    }

    fn update(&mut self, elapsed: Duration, constraint: Duration) {
        let update_constraint = match (constraint, self.get_max_update_duration()) {
            (c, None) => c,
            (c, Some(upd)) => min(c, upd),
        };

        let until_closure = {
            let compute_timer = Instant::now();
            move |quad: &Quad| {
                //return bool to decide to stop or not (because of one compute constraint, or global update per second limit)
                compute_timer.elapsed() >= update_constraint
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

    fn get_updates_per_second(&self) -> Option<f32> {
        self.average_duration.per_second()
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

//TODO : test timers behavior (via injection of deterministic timer...)
