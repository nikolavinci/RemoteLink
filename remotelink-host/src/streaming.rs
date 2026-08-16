pub mod frame;
pub mod framing;
pub mod stats;

use std::time::{SystemTime, UNIX_EPOCH};
use frame::VideoFrame;
use stats::StreamStats;

pub struct StreamOrchestrator {
    stats: StreamStats,
}

impl StreamOrchestrator {
    pub fn new() -> Self {
        Self {
            stats: StreamStats::new(),
        }
    }

    pub fn process_encoded_nalu(&mut self, nalu: Vec<u8>, is_keyframe: bool, bitrate: u32) -> Vec<u8> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
            
        let frame = VideoFrame {
            timestamp,
            frame_type: if is_keyframe { 1 } else { 0 },
            width: 1920,
            height: 1080,
            bitrate,
            data: nalu.clone(),
        };

        let wire_data = framing::serialize(&frame);
        self.stats.record_frame(nalu.len(), is_keyframe);
        
        wire_data
    }
}
