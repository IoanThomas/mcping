use crate::server::description::Description;
use crate::server::player_list::PlayerList;
use crate::server::version::Version;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Response {
    pub version: Version,
    pub players: PlayerList,
    pub description: Description,
    pub favicon: Option<String>,
}
