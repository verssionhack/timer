use std::{sync::{Arc, RwLock}, time::{Duration, Instant}};


#[derive(Debug, Clone, Copy)]
pub struct Timer {
    timer: Instant
}

impl Default for Timer {
    fn default() -> Self {
        Self { timer: Instant::now() }
    }
}

impl Timer {
    pub fn new() -> Self {
        Self { timer: Instant::now() }
    }

    pub fn reset(&mut self) {
        self.timer = Instant::now();
    }

    pub fn timer(&self) -> Duration {
        Instant::now().duration_since(self.timer)
    }
}
