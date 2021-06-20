use crate::constants::colors::DISCORD_BLUE;
use crate::constants::emotes::*;
use crate::util::{
    discord_time::get_relative_time_string,
    discord_user::{format_client_status, get_client_status},
    image::random_default_avatar,
};
use serenity::framework::standard::{macros::command, Args, CommandResult};
use serenity::model::prelude::*;
use serenity::prelude::*;

#[command]
#[description("informações sobre um usuário")]
#[aliases("whois")]
#[only_in(guilds)]
#[example("@yxqsnz")]
pub async fn userinfo(context: &Context, message: &Message, _args: Args) -> CommandResult {
    if let Some(user) = message.mentions.first() {
        let statuses = format_client_status(
            &get_client_status(&message.guild(context).await.unwrap(), &user.id).await,
        );
        message
            .channel_id
            .send_message(context, |builder| {
                builder.reference_message(message).embed(|builder| {
                    builder
                        .title(&format!("Informações de {}", user.tag()))
                        .thumbnail(user.avatar_url().unwrap_or_else(random_default_avatar))
                        .description(format!(
                            "{} **·** Tag: `{}`\n{} **·** Conta criada: {}\n{} **·** Dispositivo: `{}`",
                            DETECTIVE_EMOTE,
                            user.tag(),
                            DATE_EMOTE,
                            get_relative_time_string(user.created_at().timestamp()),
                            COMPUTER_EMOTE,
                            statuses,

                        ))
                        .color(DISCORD_BLUE)
                })
            })
            .await?;
    } else {
        message
            .reply(
                context,
                &format!("{} **·** Por favor mencione um usuário!", ERROR_EMOTE),
            )
            .await?;
    }
    Ok(())
}
