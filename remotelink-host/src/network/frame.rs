pub struct Frame {
    pub frame_type: u8,
    pub length: u32,
    pub payload: Vec<u8>,
}

impl Frame {
    pub fn parse(data: &[u8]) -> Option<Self> {
        if data.len() < 5 {
            return None;
        }
        let frame_type = data[0];
        let length = u32::from_be_bytes([data[1], data[2], data[3], data[4]]);
        if data.len() < (5 + length) as usize {
            return None; // Incomplete
        }
        let payload = data[5..(5 + length as usize)].to_vec();
        Some(Self {
            frame_type,
            length,
            payload,
        })
    }
}
