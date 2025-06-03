#[cfg(not(target_arch = "wasm32"))]
use std::time::Instant;

#[cfg(not(target_arch = "wasm32"))]
#[derive(Clone)]
pub struct Timer {
    start_time: Instant,
}

#[cfg(not(target_arch = "wasm32"))]
impl Default for Timer {
    fn default() -> Self {
        Self { start_time: Instant::now() }
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl Timer {
    pub fn new() -> Self {
        Timer::default()
    }

    #[inline(always)]
    pub fn get_time_passed_millis(&self) -> u128 {
        Instant::now().duration_since(self.start_time).as_millis()
    }

    #[inline(always)]
    pub fn get_time_passed_secs(&self) -> f64 {
        Instant::now().duration_since(self.start_time).as_secs_f64()
    }
}

#[cfg(target_arch = "wasm32")]
use js_sys::Date;

#[cfg(target_arch = "wasm32")]
#[derive(Clone)]
pub struct Timer {
    start_time: f64,
}

#[cfg(target_arch = "wasm32")]
impl Default for Timer {
    fn default() -> Self {
        Self { start_time: Date::now() }
    }
}

#[cfg(target_arch = "wasm32")]
impl Timer {
    pub fn new() -> Self {
        Timer::default()
    }

    #[inline(always)]
    pub fn get_time_passed_millis(&self) -> u128 {
        (Date::now() - self.start_time) as u128
    }

    #[inline(always)]
    pub fn get_time_passed_secs(&self) -> u128 {
        self.get_time_passed_millis() / 1000
    }
}
