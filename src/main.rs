use rusky::commands::*;
use log::*;
use rusky::config;
use rusky::util;
use serenity::http::Http;
use serenity::{framework::StandardFramework, Client};
use std::collections::HashSet;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
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
        Err(why) => panic!("Could not access application info: {:?}", why),
    };


    let framework = StandardFramework::new()
        .configure(|c| c.owners(owners).prefix(&cfg.discord.prefix))
        .group(&INFORMATION_GROUP);
    let mut client = Client::builder(cfg.discord.token)
        .framework(framework)
        .application_id(cfg.discord.id)
        .await?;

    if let Err(err) = client.start().await {
        error!("Client error: {:?}", err);
    }
    Ok(())
}
