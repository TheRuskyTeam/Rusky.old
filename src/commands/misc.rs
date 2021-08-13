use crate::{
    apis::MeowApi,
    commands::SlashCommandContext,
    macros::commands::*,
    utils::user::random_discord_default_avatar,
    RuskyResult,
};
use serenity::{builder::CreateEmbed, model::id::ChannelId};

pub struct CatCommand;
pub struct SayCommand;

pub async fn cat(context: &SlashCommandContext) -> RuskyResult<()> {
    let cat = MeowApi::fetch().await?.file;
    context
        .reply(
            CreateEmbed::default()
                .title("Meow!")
                .description(format!("[Click here]({}) to view raw image.", cat))
                .image(cat),
        )
        .await
}
pub async fn say(context: &SlashCommandContext) -> RuskyResult<()> {
    let guild = context.command.guild_id;
    if guild.is_some() {
        if let Some(guild) = guild {
            let me = guild
                .member(&context, &context.client.cache.current_user_id().await)
                .await?;
            let content_to_get = context.command.data.options.get(0);
            let channel_to_get = context.command.data.options.get(1);
            let content: String = get_arg!(content_to_get, Text ?? "** **");
            let channel: ChannelId = if let Some(ch) = get_arg!(channel_to_get, Channel) {
                ch.id
            } else {
                context.command.channel_id
            };
            if me.permissions(&context).await?.manage_webhooks() {
                let webhook = channel
                    .create_webhook(&context, &context.command.user.name)
                    .await?;

                webhook
                    .execute(context, false, |r| {
                        r.avatar_url(
                            context
                                .command
                                .user
                                .avatar_url()
                                .as_ref()
                                .unwrap_or(&random_discord_default_avatar())
                                .as_str(),
                        )
                        .content(content)
                    })
                    .await?;
                webhook.delete(context).await?;
                context.replyin("Message sent!").await?;
            } else {
                channel
                    .send_message(context, |r| {
                        r.content(format!(
                            "{content} *message sent by <@{}>*",
                            context.command.user.id
                        ))
                    })
                    .await?;
                context.replyin("Message sent!").await?;
            }
        }
    } else {
        context
            .reply("err? You need to run this command in a guild.")
            .await?;
    }
    Ok(())
}
slash!(SayCommand =>
    (@name: "say")
    (@description: "say")
    (@arg "content", Text: "content to say")
    (@arg "channel", OptionChannel: "channel to say")
    (@execute: say)
);
slash!(CatCommand =>
    (@name: "cat")
    (@description: "show a cat image")
    (@execute: cat)
);
