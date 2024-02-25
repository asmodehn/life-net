use crate::compute::running_average::RunningAverage;
use crate::compute::timer::Timer;
use crate::graphics::quad::Drawable;
use crate::graphics::view::Viewable;
use std::sync::Mutex;
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

pub(crate) trait Computable {
    fn compute(&mut self, elapsed: Duration);
}

pub(crate) trait PartialComputable {
    fn compute_partial(&mut self, elapsed: Duration, until: impl Fn() -> bool);

    fn update_completed(&self) -> bool; // TODO some kind of progress measurement ?
}

pub(crate) fn compute<C>(mut ctx: ComputeCtx, computable: &mut C) -> ComputeCtx
where
    C: Computable,
{
    // static COMPUTE_TIMER: Mutex<Timer> = Mutex::new(Timer::default());

    let elapsed = ctx.record_last_duration();

    computable.compute(elapsed);

    ctx
}

pub(crate) struct ComputeCtx {
    compute_timer: Timer,
    average_duration: RunningAverage<Duration>,
    pub last_elapsed: Duration,
    constraint: Option<Duration>,
    inner_timer: Timer,
}

impl Default for ComputeCtx {
    fn default() -> Self {
        Self {
            compute_timer: Timer::default(),
            average_duration: RunningAverage::<Duration>::new(5 * 60),
            last_elapsed: Duration::MAX,
            constraint: None,
            inner_timer: Timer::default(),
        }
    }
}

impl ComputeCtx {
    fn with_constraint(self, duration: Duration) -> Self {
        Self {
            constraint: Some(duration),
            ..self
        }
    }

    pub(crate) fn set_constraint(&mut self, duration: Duration) {
        self.constraint = Some(duration);
    }

    pub(crate) fn record_last_duration(&mut self) -> Duration {
        let elapsed = self.compute_timer.elapsed_and_reset();
        self.average_duration.record(elapsed);
        elapsed
    }

    fn reset_timer(&self) {
        self.inner_timer.elapsed_and_reset(); //ignoring elapsed measurement
    }

    fn until_closure<'s>(&'s self) -> impl Fn() -> bool + 's {
        let this_constraint = self.constraint;
        let this_inner_timer = &self.inner_timer;

        move || -> bool { this_constraint.is_some_and(|d| d <= this_inner_timer.elapsed()) }

        // move |_pc: &PC| {
        //     //return bool to decide to stop or not (because of one compute constraint, or global update per second limit)
        //     self.constraint.is_some_and(|d| d <= self.inner_timer.elapsed())
        // }
    }
}

pub(crate) fn compute_partial<PC>(mut ctx: ComputeCtx, computable: &mut PC) -> ComputeCtx
where
    PC: PartialComputable,
{
    //inner partial compute timer
    ctx.reset_timer();

    if computable.update_completed() {
        let elapsed = ctx.record_last_duration();
        ctx.last_elapsed = elapsed;
    }

    // Note last_elapsed is the update timer
    computable.compute_partial(ctx.last_elapsed, ctx.until_closure());

    ctx
}

#[cfg(test)]
mod tests {
    use crate::compute;
    use crate::compute::{Computable, ComputeCtx};
    use crate::life::cell;
    use crate::life::quad::Quad;

    #[test]
    fn lonely_dying_quad_compute() {
        let mut q = Quad::new(1, 1);
        q.image.update(&[cell::ALIVE]);

        //one update
        compute::compute(ComputeCtx::default(), &mut q);

        assert_eq!(q.image.get_pixel(0, 0), cell::DEAD)
    }
    #[test]
    fn lonely_dying_quad_compute_partial() {
        let mut q = Quad::new(1, 1);
        q.image.update(&[cell::ALIVE]);

        //one update
        compute::compute_partial(ComputeCtx::default(), &mut q);

        assert_eq!(q.image.get_pixel(0, 0), cell::DEAD)
    }

    #[test]
    fn check_stationary_one_compute() {
        let mut q = Quad::new(2, 2);
        //permanent square in quad
        q.image.update(&[cell::ALIVE; 4]);

        //one update
        compute::compute(ComputeCtx::default(), &mut q);

        assert_eq!(q.image.get_pixel(0, 0), cell::ALIVE);
        assert_eq!(q.image.get_pixel(0, 1), cell::ALIVE);
        assert_eq!(q.image.get_pixel(1, 0), cell::ALIVE);
        assert_eq!(q.image.get_pixel(1, 1), cell::ALIVE);
    }
    #[test]
    fn check_stationary_one_compute_partial() {
        let mut q = Quad::new(2, 2);
        //permanent square in quad
        q.image.update(&[cell::ALIVE; 4]);

        //one update
        compute::compute_partial(ComputeCtx::default(), &mut q);

        assert_eq!(q.image.get_pixel(0, 0), cell::ALIVE);
        assert_eq!(q.image.get_pixel(0, 1), cell::ALIVE);
        assert_eq!(q.image.get_pixel(1, 0), cell::ALIVE);
        assert_eq!(q.image.get_pixel(1, 1), cell::ALIVE);
    }
}
