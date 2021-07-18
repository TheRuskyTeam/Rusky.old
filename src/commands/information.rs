use crate::slash;

pub struct PingCommand;
slash!(PingCommand =>
     (@name: "ping")
     (@description: "Pong!")
     (@execute: (context) => {
          context.reply("pong!").await?;
     })
);
