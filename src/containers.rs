use crate::commands::CommandManager;
use std::sync::Arc;
pub struct CommandManagerContainer;
use serenity::prelude::TypeMapKey;
use tokio::sync::Mutex;
impl TypeMapKey for CommandManagerContainer
{
    type Value = Arc<Mutex<CommandManager>>;
}
