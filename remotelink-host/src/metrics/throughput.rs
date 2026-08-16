use std::time::Instant;

pub struct ThroughputTracker {
    start_time: Instant,
    total_bytes: u64,
}

impl ThroughputTracker {
    pub fn new() -> Self {
        Self {
            start_time: Instant::now(),
            total_bytes: 0,
        }
    }

    pub fn add_bytes(&mut self, bytes: usize) {
        self.total_bytes += bytes as u64;
    }

    pub fn calculate_mbps(&mut self) -> f64 {
        let elapsed = self.start_time.elapsed().as_secs_f64();
        if elapsed == 0.0 {
            return 0.0;
        }
        let mbps = (self.total_bytes as f64 * 8.0) / 1_000_000.0 / elapsed;
        
        // Reset for next window
        self.total_bytes = 0;
        self.start_time = Instant::now();
        
        mbps
    }
}
