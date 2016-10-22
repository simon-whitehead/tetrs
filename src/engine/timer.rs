use std::cell::Cell;
use std::rc::Rc;

pub enum TimerTickResult {
    None,
    Reset,
}

/// Timer represents a timed event
pub struct Timer {
    interval: f64,
    time: f64,
    global_time: Rc<Cell<f64>>,
}

impl Timer {
    pub fn new(interval: f64, global_time: Rc<Cell<f64>>) -> Timer {
        Timer {
            interval: interval,
            global_time: global_time,
            time: 0.0,
        }
    }

    /// Checks whether this timer has elapsed
    pub fn elapsed(&self) -> bool {
        self.global_time.get() - self.time >= self.interval
    }

    /// Updates the timer global time and resets the interval
    pub fn reset(&mut self) {
        self.time = self.global_time.get() + self.interval;
    }
}