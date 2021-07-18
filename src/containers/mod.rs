use std::sync::Arc;

use serenity::prelude::TypeMapKey;
use tokio::sync::Mutex;

use crate::commands::{CommandManager, ReadyOptions};

pub struct CommandManagerContainer;

pub struct ReadyOptionsContainer;

impl TypeMapKey for ReadyOptionsContainer {
    type Value = Arc<ReadyOptions>;
}

impl TypeMapKey for CommandManagerContainer {
    type Value = Arc<Mutex<CommandManager>>;
}
