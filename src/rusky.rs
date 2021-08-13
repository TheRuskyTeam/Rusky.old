use crate::{
    commands::{CommandManager, ReadyOptions},
    config::Config,
    containers::{CommandManagerContainer, ReadyOptionsContainer},
    RuskyResult,
    *,
};
use log::info;
use serenity::Client;
use std::{path::Path, sync::Arc};
use tokio::sync::Mutex;

pub struct Rusky {
    pub client: Client,
    pub config: Config,
}

impl Rusky {
    pub async fn new(cfg_file_path: &str, ready_options: ReadyOptions) -> RuskyResult<Self> {
        Self::check(cfg_file_path)?;
        let config = Config::load(cfg_file_path)?;
        let client = Client::builder(&config.discord.token)
            .event_handler(events::Handler)
            .application_id(config.discord.id)
            .await?;
        {
            let mut data = client.data.write().await;
            data.insert::<CommandManagerContainer>(Arc::new(Mutex::new(CommandManager::init())));
            data.insert::<ReadyOptionsContainer>(Arc::new(ready_options));
        }
        Ok(Self { config, client })
    }

    pub fn check(cfg_file_path: &str) -> Result<(), String> {
        if !Path::new(cfg_file_path).exists() {
            Err(String::from("Missing configuration file"))
        } else if let Err(_err) = Config::load(cfg_file_path) {
            Err(format!("{_err}"))
        } else {
            Ok(())
        }
    }

    pub async fn login(&mut self) -> RuskyResult<()> {
        if self.config.discord.auto_sharded {
            info!("Shards: Automatic.");
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
