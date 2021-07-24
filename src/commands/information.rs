use crate::{commands::SlashCommandContext, errors::NoneError, get_arg, slash, utils, RuskyResult};
use crate::constants::colors::BLUE;
use serenity::builder::CreateEmbed;
pub struct PingCommand;
pub struct UserInfoCommand;

pub async fn userinfo(context: &SlashCommandContext) -> RuskyResult<()> {
    if context.command.guild_id.is_some() {
        let member_to_get_information = context.command.data.options.get(0);
        let (user, member) = get_arg!(member_to_get_information, User).ok_or(NoneError)?;
        let member = member.as_ref().ok_or(NoneError)?;
        let user_name = &user.name;
        let user_profile = utils::user::get_user_profile(*user.id.as_u64()).await?;
        let user_banner = utils::user::get_user_banner_url(&user_profile).await;
        let user_joined_at =
            utils::time::get_discord_relative_time(member.joined_at.ok_or(NoneError)?.timestamp());
        let user_created_at = utils::time::get_discord_relative_time(user.created_at().timestamp());
        let user_tag = user.tag();
        let mut embed = CreateEmbed::default();
        embed.title(format!("informações de {user_name}"));
        if let Ok(banner_url) = user_banner {
            embed.image(banner_url);
        }
        if let Some( banner_color ) = &user_profile.banner_color {
            if let Ok(hex) = banner_color.parse::<u64>() {
                embed.color(hex);
            }
        } else if let Some( accent_color ) = &user_profile.accent_color {
            if let Ok(hex) = accent_color.parse::<u64>() {
                embed.color(hex);
            }
        } else {
            embed.color(BLUE);
        }
        embed.thumbnail(user.avatar_url().unwrap_or(utils::user::random_discord_default_avatar()));
        embed.description(format!(r#"
        Tag: {user_tag}
        Conta Criada: {user_created_at}
        Entrou: {user_joined_at}
        "#));
        context.reply_embed(&mut embed).await?;
    } else {
        context
            .reply_error("você só pode executar esse comando em uma guilda.")
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
