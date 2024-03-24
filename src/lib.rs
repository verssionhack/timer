use std::{sync::{Arc, RwLock}, time::{Duration, Instant}};


#[derive(Debug, Clone)]
pub struct Timer {
    timer: Arc<RwLock<Instant>>
}

impl Default for Timer {
    fn default() -> Self {
        Self { timer: Arc::new(RwLock::new(Instant::now())) }
    }
}

impl Timer {
    pub fn new() -> Self {
        Self { timer: Arc::new(RwLock::new(Instant::now())) }
    }

    pub fn reset(&self) {
        *self.timer.write().unwrap() = Instant::now();
    }

    pub fn timer(&self) -> Duration {
        Instant::now().duration_since(*self.timer.read().unwrap())
    }
}
