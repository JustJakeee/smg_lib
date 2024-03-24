// move all the structs that are used in both client and server into here
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use glam::Vec2;


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Packet {
    Connect(Uuid),
    Disconnect(Uuid),
    Message(String),
    Player(PlayerState),
    List(),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct PlayerState {
    pub uuid: Uuid,
    pub x: f32,
    pub y: f32,
}

impl PlayerState {
    // new take Vec2
    pub fn new(uuid: Uuid, pos: Vec2) -> Self {
        Self {
            uuid,
            x: pos.x,
            y: pos.y,
        }
    }
}