use std::time::Instant;

pub struct StreamStats {
    pub frame_count: u64,
    pub total_bytes: u64,
    last_log: Instant,
}

impl StreamStats {
    pub fn new() -> Self {
        Self {
            frame_count: 0,
            total_bytes: 0,
            last_log: Instant::now(),
        }
    }

    pub fn record_frame(&mut self, bytes: usize, is_keyframe: bool) {
        self.frame_count += 1;
        self.total_bytes += bytes as u64;

        if self.frame_count % 10 == 0 {
            let elapsed = self.last_log.elapsed().as_secs_f64();
            if elapsed > 0.0 {
                let fps = 10.0 / elapsed;
                // Calculate Mbps for the last 10 frames
                // bytes * 8 (bits) / 1,000,000 / elapsed
                let bits = (bytes * 10) as f64 * 8.0;
                let mbps = bits / 1_000_000.0 / elapsed;

                let ftype = if is_keyframe { "I" } else { "P" };
                println!("Frame {} | Type:{} | {} bytes | {:.2} Mbps | {:.2} FPS", 
                         self.frame_count, ftype, bytes, mbps, fps);
            }
            self.last_log = Instant::now();
        }
    }
}
