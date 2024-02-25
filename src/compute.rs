use crate::actor::Computable;
use crate::graphics::view::Viewable;
use crate::life::quad::Quad;
use std::time::{Duration, Instant};

mod continuous;
pub(crate) mod discrete;
pub(crate) mod rate_limiter;
pub(crate) mod running_average;
mod timer;

pub(crate) trait Compute: Viewable {
    fn update_timer_tick(&mut self);

    fn get_updates_per_second(&self) -> Option<f32>;
    fn get_max_update_duration(&self) -> Option<Duration>;
    fn is_ups_over_max(&self) -> bool;
}

//TODO : top caller for compute => Same API as Computable !

#[cfg(test)]
mod tests {
    use crate::actor::Computable;
    use crate::compute;
    use crate::compute::Compute;

    use crate::compute::rate_limiter::RateLimiter;
    use crate::life::cell;
    use crate::life::quad::Quad;
    use std::time::Duration;

    #[test]
    fn lonely_dying_quad() {
        let mut q = Quad::new(1, 1);
        q.image.update(&[cell::ALIVE]);

        let mut s = compute::discrete::DiscreteTime::new(q)
            .with_limiter(RateLimiter::default().with_maximum_rate(1.));

        //one update
        s.compute(Duration::new(0, 0), None);

        assert_eq!(s.world.image.get_pixel(0, 0), cell::DEAD)
    }

    #[test]
    fn check_stationary_one() {
        let mut q = Quad::new(2, 2);
        //permanent square in quad
        q.image.update(&[cell::ALIVE; 4]);

        let mut s = compute::discrete::DiscreteTime::new(q)
            .with_limiter(RateLimiter::default().with_maximum_rate(1.));

        //one update
        s.compute(Duration::new(0, 0), None);

        assert_eq!(s.world.image.get_pixel(0, 0), cell::ALIVE);
        assert_eq!(s.world.image.get_pixel(0, 1), cell::ALIVE);
        assert_eq!(s.world.image.get_pixel(1, 0), cell::ALIVE);
        assert_eq!(s.world.image.get_pixel(1, 1), cell::ALIVE);
    }
}
