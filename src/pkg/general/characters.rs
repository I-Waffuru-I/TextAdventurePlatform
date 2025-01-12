use serde::{Deserialize,Serialize};

#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct GameCharacterData {
    pub full_name: String,
    pub short_name: String,
    pub c_r : u8,
    pub c_g : u8,
    pub c_b : u8,
}