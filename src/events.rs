use serenity::model::prelude::Ready;
use serenity::{async_trait, prelude::*};

pub struct Handler;
#[async_trait]
impl EventHandler for Handler {
	
    async fn ready(&self, _: Context, _: Ready) {
        println!("Rusky is ready!");
    }
}
