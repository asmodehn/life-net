use crate::compute::running_average::RunningAverage;
use crate::compute::timer::Timer;
use crate::graphics::quad::Drawable;
use crate::life::cell;
use once_cell::sync::Lazy;
use std::time::Duration;

pub(crate) mod rate_limiter;
pub(crate) mod running_average;
mod timer;

fn last_update_duration() -> Duration {
    let elapsed = COMPUTE_TIMER.elapsed_and_reset();
    DURATION_AVERAGE.record(elapsed);
    elapsed
}

fn get_updates_per_second() -> Option<f32> {
    DURATION_AVERAGE
        .average()
        .and_then(|d: Duration| Some(1. / d.as_secs_f32()))
}

pub(crate) trait Computable {
    fn compute(&mut self, elapsed: Duration);
}

pub(crate) trait PartialComputable {
    type Idx;
    type El;

    fn compute_partial<'p, 'c, Remaining>(
        &'c mut self,
        elapsed: Duration,
        until: impl Fn() -> bool,
        progress: Option<Remaining>,
    ) -> Option<Remaining>
    where
        Remaining: Iterator<Item = (Self::Idx, &'p Self::El)>;

    fn update_completed(&self) -> bool;
}

static DURATION_AVERAGE: Lazy<RunningAverage<Duration>> =
    Lazy::new(|| RunningAverage::<Duration>::new(5 * 60));

const COMPUTE_TIMER: Lazy<Timer> = Lazy::new(|| Timer::default());

pub(crate) fn compute<C>(computable: &mut C)
where
    C: Computable,
{
    let elapsed = COMPUTE_TIMER.elapsed_and_reset();
    DURATION_AVERAGE.record(elapsed);

    computable.compute(elapsed);
}

pub(crate) struct ComputeCtx<'s, PC: PartialComputable> {
    // compute_timer: Timer,
    // average_duration: RunningAverage<Duration>,
    pub last_elapsed: Duration,
    constraint: Option<Duration>,
    inner_timer: Timer,
    cont: Option<Box<dyn Iterator<Item = (PC::Idx, PC::El)>>>,
}

impl<'s, PC: PartialComputable> Default for ComputeCtx<'s, PC> {
    fn default() -> Self {
        Self {
            // compute_timer: Timer::default(),
            // average_duration: RunningAverage::<Duration>::new({5 * 60}),
            last_elapsed: Duration::MAX,
            constraint: None,
            inner_timer: Timer::default(),
            cont: None::<Box<dyn Iterator<Item = (PC::Idx, PC::El)>>>,
        }
    }
}

impl<PC: PartialComputable> ComputeCtx<'_, PC> {
    pub(crate) fn with_constraint(self, duration: Duration) -> Self {
        Self {
            constraint: Some(duration),
            ..self
        }
    }

    pub(crate) fn set_constraint(&mut self, duration: Duration) {
        self.constraint = Some(duration);
    }

    // fn get_max_update_duration(&self) -> Option<Duration> {
    //     match self.limiter.limit_rate() {
    //         None => None,
    //         Some(update_rate) => Some(Duration::from_secs_f32(1. / update_rate)),
    //     }
    // }
    //
    // fn is_ups_over_max(&self) -> bool {
    //     match (self.limiter.limit_rate(), self.get_updates_per_second()) {
    //         (None, _) => false,
    //         (_, None) => false,
    //         (Some(max_ups), Some(ups)) => ups >= max_ups as f32,
    //     }
    // }

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

pub(crate) fn compute_partial<'s, PC>(
    computable: &mut PC,
    mut ctx: ComputeCtx<'s, PC>,
) -> ComputeCtx<'s, PC>
where
    PC: PartialComputable,
{
    ctx.reset_timer();

    if computable.update_completed() {
        let elapsed = COMPUTE_TIMER.elapsed_and_reset();
        DURATION_AVERAGE.record(elapsed);
        ctx.last_elapsed = elapsed;
    }

    // Note last_elapsed is the update timer
    let remaining = computable.compute_partial(ctx.last_elapsed, ctx.until_closure(), ctx.cont);

    ctx.cont = remaining;

    ctx
}

#[cfg(test)]
mod tests {
    // use crate::compute;
    // use crate::compute::ComputeCtx;
    // use crate::life::cell;
    // use crate::life::quad::Quad;
}
