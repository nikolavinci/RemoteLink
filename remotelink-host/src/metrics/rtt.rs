use std::time::{Instant, Duration};

pub struct RttTracker {
    last_heartbeat: Option<Instant>,
    pub current_rtt: Duration,
}

impl RttTracker {
    pub fn new() -> Self {
        Self {
            last_heartbeat: None,
            current_rtt: Duration::from_millis(50), // Default safe assumption
        }
    }

    pub fn mark_sent(&mut self) {
        self.last_heartbeat = Some(Instant::now());
    }

    pub fn mark_received(&mut self) {
        if let Some(sent) = self.last_heartbeat {
            self.current_rtt = sent.elapsed();
            self.last_heartbeat = None;
        }
    }
}
