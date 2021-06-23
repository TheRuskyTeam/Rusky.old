use crate::constants::emotes::ERROR_EMOTE;
use log::{error, info};
use serenity::{
    client::Context,
    framework::standard::{macros::hook, CommandResult},
    model::channel::Message,
};
#[hook]
pub async fn after(
    context: &Context,
    message: &Message,
    command_name: &str,
    command_result: CommandResult,
) {
    match command_result {
        Ok(()) => {
            info!("{}: {}", message.author.id, command_name)
        }
        Err(err) => {
            error!("{:?}", err);
            let _ = message
                .reply(
                    context,
                    &format!(
                        "{} **·** Me desculpe! Ocorreu um erro fatal no comando `{}`, Que \
                         resultou nessa mensagem.: `{:?}` ",
                        ERROR_EMOTE, command_name, err
                    ),
                )
                .await;
        }
    }
}
