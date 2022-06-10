use serde::Deserialize;

#[derive(Deserialize)]
pub struct Version {
    pub name: String,
    pub protocol: u32,
}
