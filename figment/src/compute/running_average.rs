// use ringbuf::ring_buffer::RbRead;
use ringbuf::Rb;
use ringbuf::SharedRb;
use std::iter::Sum;
use std::mem::MaybeUninit;
use std::ops::Div;
use std::sync::RwLock;

//Note: we need to be Sync to be able to be static

pub(crate) struct RunningAverage<T>
where
    T: Copy + for<'t> Sum<&'t T> + Div<u32, Output = T>,
{
    window_size: u16,
    durations: RwLock<SharedRb<T, Vec<MaybeUninit<T>>>>,
}

impl<T> RunningAverage<T>
where
    T: Copy + for<'t> Sum<&'t T> + Div<u32, Output = T>,
{
    pub fn new(window_size: u16) -> Self {
        Self {
            window_size,
            durations: RwLock::new(SharedRb::<T, Vec<_>>::new(window_size as usize)),
        }
    }

    pub fn record(&self, duration: T) {
        let mut slot = self.durations.write().unwrap();
        slot.push_overwrite(duration);
    }

    pub fn average(&self) -> Option<T> {
        let reader = self.durations.read().unwrap();
        let measurements_sum: T = reader.iter().sum();
        match reader.len() {
            0 => None,
            l => Some(measurements_sum.div(l as u32)),
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(dead_code)]
    #![allow(unused_variables)]
    #![allow(unused_imports)]

    use crate::compute::running_average::RunningAverage;
    use std::time::Duration;

    #[test]
    fn empty_measurements_check() {
        assert_eq!(
            RunningAverage::<Duration>::new(5).average(),
            None::<Duration>
        );
    }

    #[test]
    fn measurement_inside_window_ok() {
        let da = RunningAverage::<Duration>::new(5);
        da.record(Duration::new(1, 0));
        da.record(Duration::new(2, 0));
        da.record(Duration::new(3, 0));

        assert_eq!(da.average(), Some(Duration::new(2, 0)));
    }

    #[test]
    fn measurement_outside_window_dropped() {
        let da = RunningAverage::<Duration>::new(5);
        da.record(Duration::new(1, 0));
        da.record(Duration::new(2, 0));
        da.record(Duration::new(3, 0));
        da.record(Duration::new(4, 0));
        da.record(Duration::new(5, 0));
        da.record(Duration::new(6, 0));

        assert_eq!(da.average(), Some(Duration::new(4, 0)));

        //assert_eq!(da.durations.len(), 5);
    }

    #[test]
    fn measurement_outside_capacity_dropped() {
        let da = RunningAverage::<Duration>::new(5);
        //assert_eq!(da.durations.capacity(), 5); // TODO : shouldnt that be ^2 for speed ??

        da.record(Duration::new(1, 0));
        da.record(Duration::new(2, 0));
        da.record(Duration::new(3, 0));
        da.record(Duration::new(4, 0));
        da.record(Duration::new(5, 0));
        da.record(Duration::new(6, 0));

        assert_eq!(da.average(), Some(Duration::new(4, 0)));
        // assert_eq!(da.per_second(), Some(0.25));

        //assert_eq!(da.durations.len(), 5);
    }

    //TODO : benchmarks (capcity size, etc.) !
}
