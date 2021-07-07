use crate::{commands::SlashCommandContext, containers::CommandManagerContainer};
use log::{info, warn};
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        interactions::{application_command::ApplicationCommand, Interaction},
    },
    prelude::*,
};
pub struct Handler;
#[async_trait]
impl EventHandler for Handler
{
    async fn interaction_create(&self, context: Context, interaction: Interaction)
    {
        if let Interaction::ApplicationCommand(command) = interaction.clone()
        {
            let c_context = context.clone();
            let data = &c_context.data.read().await;
            if let Some(manager_arc) = data.get::<CommandManagerContainer>()
            {
                {
                    let manager = &manager_arc.lock().await;
                    if manager.commands.contains_key(&command.data.name)
                    {
                        manager
                            .run_command(&command.data.name, &SlashCommandContext {
                                client: context,
                                interaction,
                            })
                            .await;
                    }
                    else
                    {
                        warn!("Missing {} in slash commands.", command.data.name);
                    }
                }
            }
        }
    }

    async fn ready(&self, context: Context, bot: Ready)
    {
        let data = &context.data.read().await;
        let command_manager = data
            .get::<CommandManagerContainer>()
            .expect("Failed to get command manager");
        {
            let lock = &command_manager.lock().await;

            for (name, command) in &lock.commands
            {
                let info = &command.information();
                ApplicationCommand::create_global_application_command(&context, |c| {
                    c.name(name).description(&info.description);
                    if let Some(options) = &info.options
                    {
                        for option in options
                        {
                            c.add_option(option.to_owned());
                        }
                    };
                    c
                })
                .await
                .expect("Failed to create command");
            }
        }

        info!("Ready as {}", bot.user.tag());
    }
}
