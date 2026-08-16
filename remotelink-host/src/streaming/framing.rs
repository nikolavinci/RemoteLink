use super::frame::VideoFrame;

pub fn serialize(frame: &VideoFrame) -> Vec<u8> {
    let payload_len = frame.data.len() as u32;
    let mut buf = Vec::with_capacity(22 + payload_len as usize);
    
    // [type: u8 = 0x01]
    buf.push(0x01);
    
    // [length: u32]
    let total_len = 17 + payload_len; // 8+1+2+2+4 + payload
    buf.extend_from_slice(&total_len.to_be_bytes());
    
    // [timestamp: u64]
    buf.extend_from_slice(&frame.timestamp.to_be_bytes());
    
    // [frame_type: u8]
    buf.push(frame.frame_type);
    
    // [width: u16]
    buf.extend_from_slice(&frame.width.to_be_bytes());
    
    // [height: u16]
    buf.extend_from_slice(&frame.height.to_be_bytes());
    
    // [bitrate: u32]
    buf.extend_from_slice(&frame.bitrate.to_be_bytes());
    
    // [data: bytes]
    buf.extend_from_slice(&frame.data);
    
    buf
}
