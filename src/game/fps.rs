use std::time::Duration;
use std::thread::sleep;
use std::time::Instant;
pub struct FPSRegulator {
    cap: u32,
    wait_time: Duration,
    start: Instant
}

impl FPSRegulator {
    pub fn new(cap: u32) -> FPSRegulator {
        FPSRegulator {
            cap,
            wait_time: Duration::from_micros((1.0 / (cap as f64) * 1_000_000 as f64) as u64),
            start: Instant::now()
        }
    }

    pub fn start(&mut self) {
        self.start = Instant::now();
    }

    pub fn wait(&mut self) {
        let time_elapsed = self.start.elapsed();
        let wait = self.wait_time - time_elapsed;
        sleep(wait);
    }
}