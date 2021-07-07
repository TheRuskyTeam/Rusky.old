use crate::slash_command;
pub struct CatCommand;
slash_command! {
     for: CatCommand,
     name: "image cat",
     description: "Mostra uma foto de um gato",
     options: None,
     execute: (context) => {
        context.reply("Hello world").await?;
     }
}
