use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct MessagePayload {
    pub room: String,
    pub user: String,
    pub text: String,
}

#[derive(Deserialize, Clone)]
pub struct ExportQuery {
    pub room: String,
}
#[derive(Deserialize, Clone)]
pub struct ImportQuery {
    pub room: String,
}
#[derive(Deserialize, Clone)]
pub struct MessageQuery {
    pub room: String,
}