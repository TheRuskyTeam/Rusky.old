use crate::config::RuskyConfig;
use serenity::client::bridge::gateway::ShardManager;
use serenity::prelude::TypeMapKey;
use std::sync::Arc;
use tokio::sync::Mutex;

pub type RuskyError = Box<dyn std::error::Error>;
pub type RuskyResult<T> = Result<T, RuskyError>;
pub struct ShardManagerContainer;
impl TypeMapKey for ShardManagerContainer {
    type Value = Arc<Mutex<ShardManager>>;
}
pub struct RuskyConfigContainer;
impl TypeMapKey for RuskyConfigContainer {
    type Value = Arc<RuskyConfig>;
}
