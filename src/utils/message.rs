use crate::{commands::SlashCommandContext, errors::NoneError, RuskyResult};
use futures::StreamExt;
use log::error;
use serenity::{
    builder::CreateEmbed,
    collector::ComponentInteractionCollectorBuilder,
    model::interactions::InteractionResponseType::*,
};
use std::{future::Future, time::Duration};

pub async fn yes_no_menu<YFut, NFut, Y, N>(
    context: &SlashCommandContext,
    embed: &CreateEmbed,
    on_yes: Y,
    on_no: N,
) -> RuskyResult<()>
where
    Y: Fn() -> YFut,
    N: Fn() -> NFut,
    YFut: Future<Output = RuskyResult<()>>,
    NFut: Future<Output = RuskyResult<()>>, {
    context.command.create_interaction_response(&context.client, |response| {
        response.kind(ChannelMessageWithSource)
            .interaction_response_data(|message| {
                message.add_embed(embed.to_owned()).components(|c| {
                    c.create_action_row(|a| {
                        a.create_button(|b| {
                            b.custom_id("button-yes")
                                .label("Sim")
                                .style(serenity::model::interactions::message_component::ButtonStyle::Danger)
                        }).create_button(|b| {
                            b.custom_id("button-no")
                                .label("Não")
                                .style(serenity::model::interactions::message_component::ButtonStyle::Primary)
                        })
                    })
                })
            })
    }).await?;
    let collector = ComponentInteractionCollectorBuilder::new(&context.client)
        .author_id(context.command.user.id.as_u64().to_owned())
        .channel_id(context.command.channel_id.as_u64().to_owned())
        .guild_id(
            context
                .command
                .guild_id
                .ok_or(NoneError)?
                .as_u64()
                .to_owned(),
        )
        .timeout(Duration::from_secs(60))
        .collect_limit(1)
        .await;
    let _: Vec<_> = collector
        .then(|int| async {
            let int = move || int;
            if int().data.custom_id == *"button-yes" {
                if let Err(err) = on_yes().await {
                    error!("{:?}", err);
                }
            } else {
                if let Err(err) = on_no().await {
                    error!("{:?}", err);
                }
            }
        })
        .collect()
        .await;

    Ok(())
}
