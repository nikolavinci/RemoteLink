use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct MouseMove {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MouseClick {
    pub button: String,
    pub down: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct KeyEvent {
    pub vk: u16,
    pub down: bool,
}
