pub use crate::response::chat::Chat;
pub use crate::response::description::Description;
pub use crate::response::player::Player;
pub use crate::response::player_list::PlayerList;
pub use crate::response::version::Version;

use serde::Deserialize;

mod chat;
mod description;
mod player;
mod player_list;
mod version;

#[derive(Deserialize)]
pub struct Response {
    pub version: Version,
    pub players: PlayerList,
    pub description: Description,
    pub favicon: Option<String>,
}
