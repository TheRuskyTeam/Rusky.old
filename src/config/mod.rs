use std::{fs::File, io::Read};
use std::env::set_var;
use std::env::var;
use serde::Deserialize;

use crate::RuskyResult;

#[derive(Deserialize)]
pub struct DiscordConfig {
    pub token: String,
    pub auto_sharded: bool,
    pub shard_amount: u64,
    pub id: u64,
}

#[derive(Deserialize)]
pub struct ApiConfig {
    pub update_interval: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub discord: DiscordConfig,
    pub api: ApiConfig,
}

impl Config {
    pub fn load(path: &str) -> RuskyResult<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Self = toml::from_str(&contents)?;
        set_var("DISCORD_TOKEN", config.discord.token);
        assert!(var("DISCORD_TOKEN").is_ok());
        Ok(config)
    }
}
