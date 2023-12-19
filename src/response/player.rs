use serde::Deserialize;

#[derive(Deserialize)]
pub struct Player {
    pub name: String,
    pub id: String,
}
