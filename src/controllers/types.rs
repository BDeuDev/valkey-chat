use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MessagePayload {
    pub room: String,
    pub user: String,
    pub text: String,
}