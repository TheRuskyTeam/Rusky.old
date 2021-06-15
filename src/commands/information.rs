use serenity::prelude::*;
use serenity::model::prelude::*;
use serenity::framework::standard::{
    Args, CommandResult,
    macros::command,
};

#[command]
pub async fn ping(ctx: &Context, message: &Message, _args: Args) -> CommandResult {
    message.reply(ctx, "Pong").await?;
    Ok(())
}