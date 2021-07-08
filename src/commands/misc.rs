use crate::{apis::MeowApi, slash_command};
use serenity::builder::CreateEmbed;
pub struct CatCommand;
slash_command! {
     for: CatCommand,
     name: "gato",
     description: "Mostra uma foto de um gato",
     options: None,
     execute: (context) => {
        let cat = MeowApi::fetch().await?.file;
        context.reply_embed(CreateEmbed::default()
                            .title("Meow!")
                            .description(format!("[Clique aqui]({}) caso a imagem não apareça.", cat))
                            .image(cat)).await?;
     }
}
