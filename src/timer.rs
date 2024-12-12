use std::time::Instant;

pub struct Timer {
    start_time: Instant,
}

#[allow(dead_code)]
impl Timer {
    pub fn new() -> Self {
        Timer {
            start_time: Instant::now(),
        }
    }

    pub fn reset(&mut self) {
        self.start_time = Instant::now();
    }

    pub fn get_time_passed_millis(&self) -> u128 {
        Instant::now().duration_since(self.start_time).as_millis()
    }

    pub fn get_time_passed_secs(&self) -> f64 {
        Instant::now().duration_since(self.start_time).as_secs_f64()
    }
}
