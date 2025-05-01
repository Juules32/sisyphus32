use std::time::Instant;

#[derive(Clone)]
pub struct Timer {
    start_time: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Timer::default()
    }

    pub fn get_time_passed_millis(&self) -> u128 {
        Instant::now().duration_since(self.start_time).as_millis()
    }

    pub fn get_time_passed_secs(&self) -> f64 {
        Instant::now().duration_since(self.start_time).as_secs_f64()
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self { start_time: Instant::now() }
    }
}
