use serenity::builder::CreateEmbed;

use crate::{apis::MeowApi, slash};

pub struct CatCommand;
slash!(CatCommand =>
    (@name: "gato")
    (@description: "mostra uma foto de um gato")
    (@execute: (context) => {
        let cat = MeowApi::fetch().await?.file;
        context.reply_embed(CreateEmbed::default()
                            .title("Meow!")
                            .description(format!("[Clique aqui]({}) caso a imagem não apareça.", cat))
                            .image(cat)).await?;
     })
);
