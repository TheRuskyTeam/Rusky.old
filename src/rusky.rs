use std::sync::Arc;

use log::info;
use serenity::Client;
use tokio::sync::Mutex;

use crate::{
    commands::{CommandManager, ReadyOptions},
    config::Config,
    containers::{CommandManagerContainer, ReadyOptionsContainer},
    RuskyResult,
    *,
};

pub struct Rusky {
    pub client: Client,
    pub config: Config,
}

impl Rusky {
    pub async fn new(cfg_file_path: &str, ready_options: ReadyOptions) -> RuskyResult<Self> {
        let config = Config::load(cfg_file_path)?;
        let client = Client::builder(&config.discord.token)
            .event_handler(events::Handler)
            .application_id(config.discord.id)
            .await?;
        {
            let mut data = client.data.write().await;
            data.insert::<CommandManagerContainer>(Arc::clone(&Arc::new(Mutex::new(
                CommandManager::init(),
            ))));
            data.insert::<ReadyOptionsContainer>(Arc::new(ready_options));
        }
        Ok(Self { config, client })
    }

    pub async fn login(&mut self) -> RuskyResult<()> {
        if self.config.discord.auto_sharded {
            info!("Shards: Auto.");
            self.client.start_autosharded().await?;
        } else {
            info!("Shards: {}.", self.config.discord.shard_amount);

            self.client
                .start_shards(self.config.discord.shard_amount)
                .await?;
        }
        Ok(())
    }
}
