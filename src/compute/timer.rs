use std::cell::Cell;
use std::time::{Duration, Instant};

//encapsulating often used,hidden, mutating value...
#[derive(Debug, PartialEq)]
pub(crate) struct Timer {
    since: Cell<Instant>,
}

impl Default for Timer {
    fn default() -> Self {
        Self {
            since: Cell::new(Instant::now()),
        }
    }
}

impl Timer {
    pub fn elapsed_and_reset(&self) -> Duration {
        let elapsed = self.elapsed();
        self.since.replace(Instant::now());
        elapsed
    }

    pub fn elapsed(&self) -> Duration {
        self.since.get().elapsed()
    }
}
