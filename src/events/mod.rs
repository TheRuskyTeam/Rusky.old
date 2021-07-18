use log::{info, warn};
use serenity::{
    async_trait,
    model::{
        gateway::Ready,
        interactions::{application_command::ApplicationCommand, Interaction},
    },
    prelude::*,
};

use crate::{
    commands::SlashCommandContext,
    containers::{CommandManagerContainer, ReadyOptionsContainer},
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, context: Context, bot: Ready) {
        let data = &context.data.read().await;
        let command_manager = data
            .get::<CommandManagerContainer>()
            .expect("Failed to get command manager");
        let ready_options = data
            .get::<ReadyOptionsContainer>()
            .expect("failed to get ReadyOptionsContainer");
        {
            let lock = &command_manager.lock().await;
            let commands = ApplicationCommand::get_global_application_commands(&context)
                .await
                .expect("Failed to get commands");

            context.idle().await;

            if ready_options.update_commands || commands.len() != lock.commands.len() {
                info!("updating slash commands...");
                for (name, command) in &lock.commands {
                    let info = &command.information();
                    ApplicationCommand::create_global_application_command(&context, |c| {
                        c.name(name).description(&info.description);
                        if let Some(options) = &info.options {
                            for option in options {
                                c.add_option(option.to_owned());
                            }
                        };
                        c
                    })
                    .await
                    .expect("failed to create command");
                }
            }
        }

        info!("shard {} is ready as {}", context.shard_id, bot.user.tag());
    }

    async fn interaction_create(&self, context: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction.clone() {
            let cloned_context = context.clone();
            let data = &cloned_context.data.read().await;
            if let Some(manager_arc) = data.get::<CommandManagerContainer>() {
                {
                    let manager = &manager_arc.lock().await;
                    let command_name = &command.clone().data.name;
                    if manager.commands.contains_key(command_name) {
                        manager
                            .run_command(command_name, &SlashCommandContext {
                                client: context,
                                interaction,
                                command,
                            })
                            .await;
                    } else {
                        warn!("missing {} in slash commands.", command_name);
                    }
                }
            }
        }
    }
}
