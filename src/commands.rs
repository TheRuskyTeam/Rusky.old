use std::{collections::HashMap, fmt::Display};

use log::error;
use serenity::{
    async_trait,
    builder::{CreateApplicationCommandOption, CreateEmbed},
    client::Context,
    model::{
        interactions::{
            Interaction,
            InteractionApplicationCommandCallbackDataFlags,
            InteractionResponseType,
        },
        prelude::application_command::ApplicationCommandInteraction,
    },
};

use information::*;
use misc::*;
use moderation::*;

use crate::{acmd, constants::colors::*, nh};

pub mod information;
pub mod misc;
pub mod moderation;

pub struct ReadyOptions {
    pub update_commands: bool,
}

pub struct SlashCommandMetaData {
    pub name: String,
    pub description: String,
    pub options: Option<Vec<CreateApplicationCommandOption>>,
}

pub struct SlashCommandContext {
    pub client: Context,
    pub interaction: Interaction,
    pub command: ApplicationCommandInteraction,
}

impl SlashCommandContext {
    async fn reply(&self, content: impl Display) -> crate::RuskyResult<()> {
        self.reply_embed(CreateEmbed::default().color(BLUE).description(content))
            .await?;
        Ok(())
    }

    async fn update_embed(&self, embed: CreateEmbed) -> crate::RuskyResult<()> {
        self.command
            .edit_original_interaction_response(&self.client, |response| {
                response
                    .set_embeds(vec![embed])
                    .components(|c| c.set_action_rows(vec![]))
            })
            .await?;
        Ok(())
    }

    async fn reply_error(&self, content: impl Display) -> crate::RuskyResult<()> {
        self.command
            .create_interaction_response(&self.client, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message
                            .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                            .add_embed(
                                CreateEmbed::default()
                                    .color(MATERIAL_RED)
                                    .description(content)
                                    .to_owned(),
                            )
                    })
            })
            .await?;
        Ok(())
    }

    async fn reply_embed(&self, embed: &mut CreateEmbed) -> crate::RuskyResult<()> {
        self.command
            .create_interaction_response(&self.client, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| message.add_embed(embed.to_owned()))
            })
            .await?;
        Ok(())
    }
}

#[async_trait]
pub trait SlashCommand {
    fn information(&self) -> SlashCommandMetaData;
    async fn execute(&self, context: &SlashCommandContext) -> crate::RuskyResult<()>;
}

pub struct CommandManager {
    pub commands: HashMap<String, Box<dyn SlashCommand + Sync + Send>>,
}

impl CommandManager {
    pub fn init() -> Self {
        let mut commands: HashMap<String, Box<dyn SlashCommand + Sync + Send>> = nh!();

        acmd!(commands <== PingCommand);
        acmd!(commands <== CatCommand);
        acmd!(commands <== BanCommand);
        acmd!(commands <== UserInfoCommand);
        Self { commands }
    }

    pub async fn run_command(&self, query: &str, context: &SlashCommandContext) {
        if let Some(command) = self.commands.get(query) {
            if let Err(err) = command.execute(context).await {
                error!("{:?}", err);
            }
        }
    }
}

/*
* WARNING: UNSAFE AREA!
*/
unsafe impl Sync for CommandManager {}

unsafe impl Send for CommandManager {}
