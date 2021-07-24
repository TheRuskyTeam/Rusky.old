use crate::{apis::MeowApi, commands::SlashCommandContext, slash, RuskyResult};
use serenity::builder::CreateEmbed;

pub struct CatCommand;

pub async fn cat(context: &SlashCommandContext) -> RuskyResult<()> {
    let cat = MeowApi::fetch().await?.file;
    context
        .reply_embed(
            CreateEmbed::default()
                .title("Meow!")
                .description(format!("[Clique aqui]({}) caso a imagem não apareça.", cat))
                .image(cat),
        )
        .await
}

slash!(CatCommand =>
    (@name: "gato")
    (@description: "mostra uma foto de um gato")
    (@execute: cat)
);
