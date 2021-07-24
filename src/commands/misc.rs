use serenity::builder::CreateEmbed;

use crate::{apis::MeowApi, commands::SlashCommandContext, slash, RuskyResult};

pub async fn run_cat(context: &SlashCommandContext) -> RuskyResult<()> {
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
pub struct CatCommand;
slash!(CatCommand =>
    (@name: "gato")
    (@description: "mostra uma foto de um gato")
    (@execute: (c) => { run_cat(c).await?; } )
);
