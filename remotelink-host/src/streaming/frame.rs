pub struct VideoFrame {
    pub timestamp: u64,
    pub frame_type: u8,
    pub width: u16,
    pub height: u16,
    pub bitrate: u32,
    pub data: Vec<u8>,
}
