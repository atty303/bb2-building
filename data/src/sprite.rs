use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Sprite {
    pub x: u16,
    pub y: u16,
    pub width: u8,
    pub height: u8,
}
