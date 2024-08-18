use serde::Deserialize;

#[derive(Deserialize)]
pub struct NormalResponse {
    pub success: bool,
}

#[derive(Deserialize)]
pub struct PowerResponse {
    pub on: bool,
}

#[derive(Deserialize)]
pub struct InfoResponse {
    pub ip: String,
}
