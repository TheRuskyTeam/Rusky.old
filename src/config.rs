use crate::RuskyResult;
use serde::Deserialize;
use std::{fs::File, io::Read};
#[derive(Deserialize)]
pub struct DiscordConfig
{
    pub token: String,
    pub auto_sharded: bool,
    pub shard_amount: u64,
    pub id: u64,
}
#[derive(Deserialize)]
pub struct ApiConfig
{
    pub update_interval: String,
}
#[derive(Deserialize)]
pub struct Config
{
    pub discord: DiscordConfig,
    pub api: ApiConfig,
}
impl Config
{
    pub fn load(path: &str) -> RuskyResult<Self>
    {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(toml::from_str(&contents)?)
    }
}
