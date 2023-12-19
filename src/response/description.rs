use crate::response::chat::Chat;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Description {
    Text(String),
    Chat(Chat),
}
