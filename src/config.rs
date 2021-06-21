use crate::typings::RuskyResult;
use serde::Deserialize;
use std::collections::HashSet;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize, Clone)]
pub struct Discord {
    pub token: String,
    pub owners: HashSet<u64>,
    pub shard_amount: u64,
    pub id: u64,
    pub prefix: String,
}
#[derive(Deserialize, Clone)]
pub struct RuskyConfig {
    pub discord: Discord,
}

impl RuskyConfig {
    pub fn load(path: &str) -> RuskyResult<RuskyConfig> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(toml::from_str(&content)?)
    }
}
