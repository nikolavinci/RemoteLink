pub mod rtt;
pub mod throughput;
pub mod adaptation;

use rtt::RttTracker;
use throughput::ThroughputTracker;
use adaptation::BitrateAdapter;

pub struct MetricsOrchestrator {
    pub rtt: RttTracker,
    pub throughput: ThroughputTracker,
    pub adapter: BitrateAdapter,
}

impl MetricsOrchestrator {
    pub fn new() -> Self {
        Self {
            rtt: RttTracker::new(),
            throughput: ThroughputTracker::new(),
            adapter: BitrateAdapter::new(),
        }
    }

    pub fn evaluate_network(&mut self) -> u32 {
        let mbps = self.throughput.calculate_mbps();
        let new_bitrate = self.adapter.adapt(self.rtt.current_rtt, mbps);
        
        println!("Metrics | RTT: {}ms | Throughput: {:.2} Mbps | Target Bitrate: {} bps", 
                 self.rtt.current_rtt.as_millis(), mbps, new_bitrate);
                 
        new_bitrate
    }
}
