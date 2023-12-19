use serde::Deserialize;

#[derive(Deserialize)]
pub struct Chat {
    pub text: String,
}
