use std::time::Duration;

pub struct BitrateAdapter {
    pub target_bitrate: u32,
    min_bitrate: u32,
    max_bitrate: u32,
}

impl BitrateAdapter {
    pub fn new() -> Self {
        Self {
            target_bitrate: 4_000_000, // 4 Mbps default
            min_bitrate: 500_000,      // 500 kbps min
            max_bitrate: 8_000_000,    // 8 Mbps max
        }
    }

    pub fn adapt(&mut self, rtt: Duration, throughput_mbps: f64) -> u32 {
        let rtt_ms = rtt.as_millis();
        
        // Basic adaptation algorithm
        if rtt_ms > 150 {
            // High latency, drop bitrate by 20%
            self.target_bitrate = (self.target_bitrate as f64 * 0.8) as u32;
        } else if throughput_mbps > (self.target_bitrate as f64 / 1_000_000.0) + 1.0 {
            // Good throughput, ramp up by 10%
            self.target_bitrate = (self.target_bitrate as f64 * 1.1) as u32;
        }

        // Clamp to min/max
        self.target_bitrate = self.target_bitrate.clamp(self.min_bitrate, self.max_bitrate);
        
        self.target_bitrate
    }
}
