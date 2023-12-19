use crate::response::player::Player;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct PlayerList {
    pub max: u32,
    pub online: u32,
    pub sample: Option<Vec<Player>>,
}
