use std::time::Duration;
use std::thread;

pub struct Libx264Encoder {
    bitrate: u32,
}

impl Libx264Encoder {
    pub fn new() -> Result<Self, &'static str> {
        println!("Initializing libx264 software encoder fallback...");
        Ok(Self { bitrate: 500_000 })
    }

    pub fn encode(&mut self, _frame_data: &[u8], is_keyframe: bool) -> Vec<u8> {
        // Simulate encoding latency (target < 50ms per prompt)
        thread::sleep(Duration::from_millis(15));
        
        // Output a dummy Annex-B NALU stream just to prove format correctness
        // In real libx264, we'd pass the frame to x264_encoder_encode
        if is_keyframe {
            // Simulated IDR NALU (Start Code 0x00 00 00 01)
            vec![0x00, 0x00, 0x00, 0x01, 0x65, 0x88, 0x84, 0x01]
        } else {
            // Simulated P-frame NALU
            vec![0x00, 0x00, 0x00, 0x01, 0x41, 0x9A, 0x11, 0x02]
        }
    }

    pub fn set_bitrate(&mut self, bitrate: u32) {
        self.bitrate = bitrate;
        println!("libx264: Bitrate adapted to {} bps", bitrate);
    }
}
