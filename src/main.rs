use log::*;
use rusky::async_run;
use rusky::commands::*;
use rusky::config;
use rusky::util;
use serenity::http::Http;
use serenity::{framework::StandardFramework, Client};
use std::collections::HashSet;

#[tokio::main]
async fn main() {
    async_run! {{
    util::setup()?;
       info!("Rusky a simple bot for Discord!");
       let cfg = config::load("Rusky.toml")?;

       let http = Http::new_with_token(&cfg.discord.token);

       let (owners, _bot_id) = match http.get_current_application_info().await {
           Ok(info) => {
               let mut owners = HashSet::new();
               owners.insert(info.owner.id);

               (owners, info.id)
           },
           Err(why) => {return Err(Box::new(why));},
       };


       let framework = StandardFramework::new()
           .configure(|c| c.owners(owners).prefix(&cfg.discord.prefix))
           .group(&INFORMATION_GROUP);
       let mut client = Client::builder(cfg.discord.token)
           .framework(framework)
           .application_id(cfg.discord.id)
           .await?;

       client.start().await?;
       } catch err => {
           error!("{:?}", err);
       }};
}
