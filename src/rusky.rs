use crate::commands::information::HELP;
use crate::commands::*;
use crate::config::RuskyConfig;
use crate::containers::RuskyConfigContainer;
use crate::containers::ShardManagerContainer;
use crate::events::Handler;
use crate::hooks;
use crate::typings::RuskyResult;
use serenity::client::bridge::gateway::GatewayIntents;
use serenity::http::Http;
use serenity::{framework::standard::StandardFramework, Client};
use std::collections::HashSet;
use std::sync::Arc;

pub struct Rusky {
    client: Client,
}
impl Rusky {
    pub async fn login(&mut self, shard_amount: u64) -> RuskyResult<()> {
        self.client.start_shards(shard_amount).await?;
        Ok(())
    }
    pub async fn new(config: &RuskyConfig, handler: Handler) -> RuskyResult<Self> {
        let http = Http::new_with_token(&config.discord.token);
        let (owners, bot_id) = match http.get_current_application_info().await {
            Ok(info) => {
                let mut owners = HashSet::new();
                if let Some(team) = info.team {
                    owners.insert(team.owner_user_id);
                } else {
                    owners.insert(info.owner.id);
                }
                match http.get_current_user().await {
                    Ok(bot_id) => (owners, bot_id.id),
                    Err(err) => {
                        panic!("{:?}", err)
                    }
                }
            }
            Err(err) => panic!("{:?}", err),
        };
        let framework = StandardFramework::new()
            .configure(|c| {
                c.with_whitespace(true)
                    .on_mention(Some(bot_id))
                    .prefix(&config.discord.prefix)
                    .owners(owners)
            })
            .help(&HELP)
            .after(hooks::after)
            .group(&INFORMATION_GROUP)
            .group(&UTILS_GROUP);
        let client = Client::builder(&config.discord.token)
            .event_handler(handler)
            .application_id(config.discord.id)
            .framework(framework)
            .intents(GatewayIntents::all())
            .await?;
        {
            let mut data = client.data.write().await;
            data.insert::<RuskyConfigContainer>(Arc::clone(&Arc::new(config.clone())));
            data.insert::<ShardManagerContainer>(Arc::clone(&client.shard_manager));
        }
        Ok(Rusky { client })
    }
}
