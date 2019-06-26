#[derive(Debug, Copy, Clone)]
pub enum Type {
    Wired,
    WiFi,
}

#[derive(Debug, Clone)]
pub struct NetworkInterface {
    pub name: String,
    pub display_name: Option<String>,
    pub itype: Type,
    pub is_up: bool,
    pub bssid: Option<String>,
    pub ssid: Option<String>,
    pub up: u64,
    pub down: u64,
}
