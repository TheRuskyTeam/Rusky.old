use log::info;
use serenity::{async_trait, model::prelude::Ready, prelude::*};
pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, _: Ready) {
        info!("Shard {} is Ready.", context.shard_id + 1);
    }
}
