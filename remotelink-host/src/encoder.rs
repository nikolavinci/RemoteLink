mod nvenc;
mod intel_qsv;
mod libx264;

use std::time::{Instant, Duration};

pub enum EncoderBackend {
    Nvenc(nvenc::NvencEncoder),
    Qsv(intel_qsv::QsvEncoder),
    Libx264(libx264::Libx264Encoder),
}

pub struct VideoEncoder {
    backend: EncoderBackend,
    frame_counter: u64,
    last_log: Instant,
    accumulated_latency: Duration,
}

impl VideoEncoder {
    pub fn new() -> Self {
        // 1. Try NVENC
        if let Ok(nv) = nvenc::NvencEncoder::new() {
            println!("Encoder initialized: NVIDIA NVENC");
            return Self {
                backend: EncoderBackend::Nvenc(nv),
                frame_counter: 0,
                last_log: Instant::now(),
                accumulated_latency: Duration::ZERO,
            };
        }

        // 2. Try QSV
        if let Ok(qsv) = intel_qsv::QsvEncoder::new() {
            println!("Encoder initialized: Intel QSV");
            return Self {
                backend: EncoderBackend::Qsv(qsv),
                frame_counter: 0,
                last_log: Instant::now(),
                accumulated_latency: Duration::ZERO,
            };
        }

        // 3. Fallback to libx264
        println!("Hardware encoders unavailable. Falling back to software encoding.");
        let x264 = libx264::Libx264Encoder::new().unwrap();
        
        Self {
            backend: EncoderBackend::Libx264(x264),
            frame_counter: 0,
            last_log: Instant::now(),
            accumulated_latency: Duration::ZERO,
        }
    }

    pub fn encode_frame(&mut self, frame_data: &[u8]) -> Vec<u8> {
        let start = Instant::now();
        
        // Keyframe every 2 seconds (assuming 30 FPS, that's every 60 frames)
        let is_keyframe = self.frame_counter % 60 == 0;
        
        let nalu = match &mut self.backend {
            EncoderBackend::Nvenc(e) => e.encode(frame_data, is_keyframe),
            EncoderBackend::Qsv(e) => e.encode(frame_data, is_keyframe),
            EncoderBackend::Libx264(e) => e.encode(frame_data, is_keyframe),
        };

        let latency = start.elapsed();
        self.accumulated_latency += latency;
        self.frame_counter += 1;

        if self.frame_counter % 10 == 0 {
            let avg_latency = self.accumulated_latency.as_millis() as f64 / 10.0;
            println!("Encoded 10 frames. Avg latency: {:.2} ms per frame", avg_latency);
            self.accumulated_latency = Duration::ZERO;
        }

        // Simulate adaptive bitrate: swap bitrate at frame 30 to prove dynamic adaptation
        if self.frame_counter == 30 {
            self.set_bitrate(8_000_000); // Jump to 8 Mbps
        }

        nalu
    }

    pub fn set_bitrate(&mut self, bitrate: u32) {
        match &mut self.backend {
            EncoderBackend::Nvenc(e) => e.set_bitrate(bitrate),
            EncoderBackend::Qsv(e) => e.set_bitrate(bitrate),
            EncoderBackend::Libx264(e) => e.set_bitrate(bitrate),
        }
    }
}

pub fn init() {
    println!("Starting Encoder integration test...");
    let mut encoder = VideoEncoder::new();
    
    // Simulate capturing and encoding 60 frames (2 seconds)
    let dummy_frame = vec![0u8; 1920 * 1080 * 4]; // Dummy BGRA32 buffer
    
    for i in 1..=60 {
        let nalu = encoder.encode_frame(&dummy_frame);
        if i == 1 {
            println!("First frame NALU length: {} bytes (IDR)", nalu.len());
        } else if i == 2 {
            println!("Second frame NALU length: {} bytes (P-Frame)", nalu.len());
        }
    }
}
