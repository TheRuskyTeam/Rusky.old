use crate::slash_command;
pub struct PingCommand;
slash_command! {
     for: PingCommand,
     name: "ping",
     description: "Veja o ping do bot",
     options: None,
     execute: (context) => {
        context.reply("Hello world").await?;
     }
}
