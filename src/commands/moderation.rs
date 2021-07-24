use crate::{
    commands::SlashCommandContext,
    constants::{
        colors::{BLUE, MATERIAL_RED},
        emotes::RUSKY_CHECK,
    },
    errors::NoneError,
    get_arg,
    slash,
    utils::{guild::get_guild_owner, message::yes_no_menu},
    RuskyResult,
};
use serenity::builder::CreateEmbed;
pub struct BanCommand;
pub async fn ban(context: &SlashCommandContext) -> RuskyResult<()> {
    if context.command.guild_id.is_some() {
        let member_to_get = &context.command.data.options.get(0);
        let reason_to_get = &context.command.data.options.get(1);
        let (user_to_ban, _) = get_arg!(member_to_get, User).ok_or(NoneError)?;
        let reason = get_arg!(reason_to_get, Text ?? "Motivo não especificado");
        let guild = context.command.guild_id.ok_or(NoneError)?;
        let me = &guild
            .member(
                &context.client,
                &context.client.cache.current_user_id().await,
            )
            .await?;
        let me_permissions = me.permissions(&context.client).await?;
        let member_to_ban = &guild.member(&context.client, user_to_ban).await?;
        let author_permissions = &context
            .command
            .member
            .as_ref()
            .ok_or(NoneError)?
            .permissions(&context.client)
            .await?;
        if !author_permissions.ban_members() {
            context
                .reply_error(
                    "Você precisa da permissão `Banir membros` para poder executar esse comando!",
                )
                .await?;
        } else if get_guild_owner(&context.client, &guild)
            .await?
            .to_user(&context.client)
            .await?
            .id ==
            user_to_ban.id
        {
            context
                .reply_error("Porque você está tentando banir o dono do servidor?")
                .await?;
        } else if member_to_ban
            .permissions(&context.client)
            .await?
            .administrator()
        {
            context
                .reply_error("Você não pode banir um administrador.")
                .await?;
        } else if !me_permissions.ban_members() {
            context
                .reply_error("Eu preciso da permissão `Banir membros` para executar esse comando.")
                .await?;
        } else {
            yes_no_menu(
                context,
                CreateEmbed::default()
                    .color(MATERIAL_RED)
                    .description(format!(
                        "Você deseja realmente banir <@{}>?",
                        user_to_ban.id.as_u64()
                    )),
                || async {
                    member_to_ban
                        .ban_with_reason(&context.client, 7, &reason)
                        .await?;
                    context
                        .update_embed(
                            CreateEmbed::default()
                                .description(format!(
                                    "{} **·** <@{}> foi banido.",
                                    RUSKY_CHECK,
                                    user_to_ban.id.as_u64()
                                ))
                                .color(BLUE)
                                .to_owned(),
                        )
                        .await?;
                    Ok(())
                },
                || async {
                    context
                        .update_embed(
                            CreateEmbed::default()
                                .description("Banimento cancelado.")
                                .color(MATERIAL_RED)
                                .to_owned(),
                        )
                        .await?;
                    Ok(())
                },
            )
            .await?;
        }
    } else {
        context
            .reply_error("Você só pode executar esse comando em uma guilda.")
            .await?;
    }
    Ok(())
}
slash!(BanCommand =>
    (@name: "ban")
    (@description: "bane um membro")
    (@arg "membro", User: "Membro para banir")
    (@arg "motivo", OptionText: "Motivo para banir")
    (@execute: ban)
);
