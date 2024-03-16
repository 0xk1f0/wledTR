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
pub struct Wifi {
    pub bssid: String,
    pub rssi: isize,
    pub signal: usize,
    pub channel: usize,
}

#[derive(Deserialize)]
pub struct Segments {
    pub start: usize,
    pub stop: usize,
    pub length: usize,
    pub col: Vec<Vec<(u8, u8, u8)>>,
}

#[derive(Deserialize)]
pub struct StateResponse {
    pub on: bool,
    pub bri: usize,
    pub ps: usize,
    pub seg: Vec<Segments>,
}

#[derive(Deserialize)]
pub struct InfoResponse {
    pub ver: String,
    pub name: String,
    pub arch: String,
    pub core: String,
    pub freeheap: usize,
    pub uptime: usize,
    pub opt: usize,
    pub brand: String,
    pub product: String,
    pub mac: String,
    pub ip: String,
    pub wifi: Wifi,
}
