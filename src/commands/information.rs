use serenity::builder::CreateEmbed;

use crate::{
    commands::SlashCommandContext,
    constants::colors::BLUE,
    errors::NoneError,
    f,
    macros::commands::*,
    utils::{
        time::get_discord_relative_time,
        user::{get_user_banner_url, get_user_profile, random_discord_default_avatar},
    },
    RuskyResult,
};

pub struct PingCommand;
pub struct UserInfoCommand;
pub async fn userinfo(context: &SlashCommandContext) -> RuskyResult<()> {
    if context.command.guild_id.is_some() {
        let member_to_get_information = context.command.data.options.get(0);
        let (user, _) = get_arg!(member_to_get_information, User).ok_or(NoneError)?;
        let guild = context.command.guild_id.ok_or(NoneError)?;
        let member = guild.member(&context, user.id).await?;
        let user_name = &user.name;
        let user_profile = get_user_profile(*user.id.as_u64()).await?;
        let user_banner = get_user_banner_url(&user_profile).await;
        let user_joined_at =
            get_discord_relative_time(member.joined_at.ok_or(NoneError)?.timestamp());
        let user_created_at = get_discord_relative_time(user.created_at().timestamp());
        let user_tag = user.tag();
        let mut embed = CreateEmbed::default();
        let mut high_color: Option<_> = None;

        if let Some((id, _)) = member.highest_role_info(&context).await {
            if let Some(role) = context.client.cache.role(&guild, id).await {
                high_color = Some(role.colour);
            } else {
                let roles = guild.roles(&context).await?;
                if let Some(r) = roles.get(&id) {
                    high_color = Some(r.colour)
                }
            }
        }
        embed.title(format!("informações de {user_name}"));

        if let Ok(banner_url) = user_banner {
            embed.image(banner_url);
        }
        if let Some(accent_color) = user_profile.accent_color {
            embed.color(accent_color);
        } else if let Some(banner_color) = &user_profile.banner_color {
            let hex = hex::decode(banner_color.replace("#", ""))?
                .into_iter()
                .map(|i| f!("{i}"))
                .collect::<String>();
            if let Ok(hex) = hex.parse::<u64>() {
                embed.color(hex);
            }
        } else if let Some(c) = high_color {
            embed.color(c);
        } else {
            embed.color(BLUE);
        }
        embed.thumbnail(
            user.avatar_url()
                .unwrap_or_else(random_discord_default_avatar),
        );
        embed.description(format!(
            r#"
        Tag: {user_tag}
        Created at: {user_created_at}
        Joined at: {user_joined_at}
        "#
        ));
        context.reply(embed).await?;
    } else {
        context
            .reply("err? You need to run this command in a guild")
            .await?;
    }
    Ok(())
}

pub async fn ping(context: &SlashCommandContext) -> RuskyResult<()> {
    context.reply("Pong. *Comando para fazer.*").await?;
    Ok(())
}

slash!(UserInfoCommand =>
    (@name: "userinfo")
    (@description: "Pega as informações de um membro.")
    (@arg "membro", User: "membro para pegar as informações")
    (@execute: userinfo)
);
slash!(PingCommand =>
     (@name: "ping")
     (@description: "Pong!")
     (@execute: ping)
);
