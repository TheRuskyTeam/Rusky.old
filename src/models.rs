use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Discord {
    pub token: String,
    pub id: u64,
    pub prefix: String,
}
#[derive(Debug, Deserialize)]
pub struct RuskyConfig {
    pub discord: Discord,
}
