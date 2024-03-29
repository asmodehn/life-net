use crate::compute::running_average::RunningAverage;
use crate::compute::timer::Timer;
use once_cell::sync::Lazy;
use std::iter::Peekable;
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

pub trait Computable {
    type Stepper: Iterator;

    fn compute_reset(&self) -> Peekable<Self::Stepper>;

    fn compute(&mut self, elapsed: Duration, remainder: &mut Peekable<Self::Stepper>);

    fn compute_until(
        &mut self,
        elapsed: Duration,
        remainder: &mut Peekable<Self::Stepper>,
        until: impl Fn() -> bool,
    );
}

static DURATION_AVERAGE: Lazy<RunningAverage<Duration>> =
    Lazy::new(|| RunningAverage::<Duration>::new(5 * 60));

const COMPUTE_TIMER: Lazy<Timer> = Lazy::new(|| Timer::default());

pub fn compute<C>(computable: &mut C)
where
    C: Computable,
{
    let elapsed = COMPUTE_TIMER.elapsed_and_reset();
    DURATION_AVERAGE.record(elapsed);

    let mut cit = computable.compute_reset();
    computable.compute(elapsed, &mut cit);
}

pub struct ComputeCtx {
    // compute_timer: Timer,
    // average_duration: RunningAverage<Duration>,
    pub last_elapsed: Duration,
    constraint: Option<Duration>,
    inner_timer: Timer,
}

impl Default for ComputeCtx {
    fn default() -> Self {
        Self {
            // compute_timer: Timer::default(),
            // average_duration: RunningAverage::<Duration>::new({5 * 60}),
            last_elapsed: Duration::MAX,
            constraint: None,
            inner_timer: Timer::default(),
        }
    }
}

impl ComputeCtx {
    pub fn with_constraint(self, duration: Duration) -> Self {
        Self {
            constraint: Some(duration),
            ..self
        }
    }

    pub fn set_constraint(&mut self, duration: Duration) {
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

        move || -> bool { this_constraint.is_some_and(move |d| d <= this_inner_timer.elapsed()) }

        // move |_pc: &PC| {
        //     //return bool to decide to stop or not (because of one compute constraint, or global update per second limit)
        //     self.constraint.is_some_and(|d| d <= self.inner_timer.elapsed())
        // }
    }
}

pub fn compute_reset<C>(computable: &C) -> Peekable<C::Stepper>
where
    C: Computable,
{
    computable.compute_reset()
}

//TODO : make compute code similar somehow...
pub fn compute_until<C>(
    computable: &mut C,
    stepper: &mut Option<Peekable<C::Stepper>>,
    ctx: &mut ComputeCtx,
) where
    C: Computable,
{
    ctx.reset_timer();

    if stepper.is_none() || stepper.as_mut().is_some_and(|s| s.peek().is_none()) {
        // println!("RESET !");
        *stepper = Some(computable.compute_reset());

        let elapsed = COMPUTE_TIMER.elapsed_and_reset();
        DURATION_AVERAGE.record(elapsed);
        ctx.last_elapsed = elapsed;
    }

    // Note last_elapsed is the update timer
    computable.compute_until(
        ctx.last_elapsed,
        stepper.as_mut().unwrap(),
        ctx.until_closure(),
    );
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    #![allow(unused_variables)]
    #![allow(unused_imports)]

    use crate::compute;
    use crate::compute::{Computable, ComputeCtx};
    use itertools::Itertools;
    use std::iter::{zip, Peekable};
    use std::ops::Range;
    use std::time::Duration;

    #[derive(Debug, Clone, PartialEq)]
    struct DoubleRange(Range<u16>, Range<u16>);

    #[derive(Debug, Clone, PartialEq)]
    struct DoubleCounter {
        range: DoubleRange, // not a ref to avoid requiring lifetime on Stepper
        // AND side step https://github.com/rust-lang/rust/issues/81823
        current: u16,
    }

    impl DoubleCounter {
        fn new(drange: DoubleRange) -> Self {
            let s = drange.1.start;
            Self {
                range: drange,
                current: s,
            }
        }
    }

    impl Iterator for DoubleCounter {
        type Item = u16;

        fn next(&mut self) -> Option<Self::Item> {
            match self.current.checked_add(1) {
                None => None,
                Some(res) => {
                    if self.range.1.contains(&res) {
                        self.current = res;
                        Some(res)
                    } else {
                        None
                    }
                }
            }
        }
    }

    impl Computable for DoubleRange {
        type Stepper = DoubleCounter;

        fn compute_reset(&self) -> Peekable<Self::Stepper> {
            let this = self.clone();
            Self::Stepper::new(this).peekable()
        }

        fn compute_until(
            &mut self,
            elapsed: Duration,
            remainder: &mut Peekable<Self::Stepper>,
            until: impl Fn() -> bool,
        ) {
            loop {
                //attempt an update step
                match remainder.next() {
                    None => {
                        break;
                    }
                    Some(i) => {
                        // just keep going
                    }
                }

                //late until call to ensure some progress
                if until() {
                    break;
                }
            }
        }

        fn compute(&mut self, elapsed: Duration, remainder: &mut Peekable<Self::Stepper>) {
            loop {
                //attempt an update step
                match remainder.next() {
                    None => {
                        break;
                    }
                    Some(i) => {
                        // just keep going
                    }
                }
            }
        }
    }

    //TODO : test that ensures actual progress on partial compute

    #[test]
    fn compute_reset_works() {
        let dr = DoubleRange(0..7u16, 0..42u16);

        let cnt = dr.compute_reset();

        let vrif = DoubleCounter {
            range: dr.clone(),
            current: dr.1.start,
        };

        for (c, v) in zip(cnt, vrif) {
            assert_eq!(c, v)
        }
    }

    #[test]
    fn compute_partial_actually_updates() {
        let mut dr = DoubleRange(0..7u16, 0..42u16);

        let mut cnt = dr.compute_reset();

        assert_eq!(cnt.peek(), Some(&1));

        dr.compute_until(Duration::new(0, 0), &mut cnt, || true);

        assert_eq!(cnt.peek(), Some(&2));

        dr.compute(Duration::new(0, 0), &mut cnt);

        assert_eq!(cnt.peek(), None)
    }

    //TODO : test that ensure compute actually update stuff (for both cases)
}
