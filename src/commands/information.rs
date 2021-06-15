use crate::util::{get_ram_usage, get_version};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;
use sysinfo::{ProcessExt, System, SystemExt};

#[command]
pub async fn ping(context: &Context, message: &Message, _args: Args) -> CommandResult {
    message.reply(context, "Pong").await?;
    Ok(())
}
#[command]
#[aliases("bot", "informacoesdobot", "rusky")]
pub async fn botinfo(context: &Context, message: &Message, _args: Args) -> CommandResult {
    let mut message = message.reply(context, "`...`").await?;
    let version = get_version();
    let system = System::new_all();
    let current_process = system.get_process_by_name("rusky")[0];
    let ram_usage = get_ram_usage().await;
    message
        .edit(context, |builder| {
            builder.content("").embed(|builder| {
                builder
                    .title("Minhas informações")
                    .field("Uso de ram", format!("`{}`", ram_usage), true)
                    .field(
                        "Uso de CPU",
                        format!("`{}%`", current_process.cpu_usage()),
                        true,
                    )
                    .field("Versão do Rust", format!("`v{}`", version), true)
                    .colour(0x5865F2)
            })
        })
        .await?;

    Ok(())
}
