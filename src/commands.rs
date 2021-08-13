use std::{collections::HashMap, convert::Into, fmt::Display};

use log::error;
use serde_json::Value;
use serenity::{
    async_trait,
    builder::{CreateApplicationCommandOption, CreateEmbed},
    client::{Cache, Context},
    http::{CacheHttp, Http},
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

use crate::{constants::colors::*, macros::commands::*};

pub mod information;
pub mod misc;
pub mod moderation;

pub struct ReadyOptions {
    pub update_commands: bool,
}

pub struct ReplyData {
    pub content: Option<String>,
    // emp? <content>
    pub is_emphemeral: bool,
    // emp+err? <content>
    pub is_error: bool,
    pub embed: Option<CreateEmbed>,
}
impl From<&mut CreateEmbed> for ReplyData {
    fn from(e: &mut CreateEmbed) -> Self {
        let mut reply = ReplyData {
            content: None,
            is_emphemeral: true,
            is_error: false,
            embed: Some(e.to_owned()),
        };

        if let Some(Value::String(s)) = e.0.clone().get("description") {
            if s.starts_with("emp?") {
                reply.is_emphemeral = true;
                e.description(s.replace("emp?", "").trim().to_string());
            } else if s.starts_with("emp+err?") {
                reply.is_emphemeral = true;
                reply.is_error = true;
                e.description(s.replace("emp+err?", "").trim().to_string());
            } else if s.starts_with("err?") {
                reply.is_error = true;
                e.description(s.replace("err?", "").trim().to_string());
            }
        }
        reply
    }
}
impl From<CreateEmbed> for ReplyData {
    fn from(mut e: CreateEmbed) -> Self {
        let mut reply = ReplyData {
            content: None,
            is_emphemeral: true,
            is_error: false,
            embed: Some(e.to_owned()),
        };

        if let Some(Value::String(s)) = e.0.clone().get("description") {
            if s.starts_with("emp?") {
                reply.is_emphemeral = true;
                e.description(s.replace("emp?", "").trim().to_string());
            } else if s.starts_with("emp+err?") {
                reply.is_emphemeral = true;
                reply.is_error = true;
                e.description(s.replace("emp+err?", "").trim().to_string());
            } else if s.starts_with("err?") {
                reply.is_error = true;
                e.description(s.replace("err?", "").trim().to_string());
            }
        }
        reply
    }
}
impl From<&str> for ReplyData {
    fn from(s: &str) -> Self {
        let s = s.to_string();
        let mut reply = ReplyData {
            content: Some(s.clone()),
            is_emphemeral: true,
            is_error: false,
            embed: None,
        };
        if s.starts_with("emp?") {
            reply.is_emphemeral = true;
            reply.content = Some(s.replace("emp?", "").trim().to_string());
        } else if s.starts_with("emp+err?") {
            reply.is_emphemeral = true;
            reply.is_error = true;
            reply.content = Some(s.replace("emp+err?", "").trim().to_string());
        } else if s.starts_with("err?") {
            reply.is_error = true;
            reply.content = Some(s.replace("err?", "").trim().to_string());
        }
        reply
    }
}
impl From<String> for ReplyData {
    fn from(s: String) -> Self {
        let mut reply = ReplyData {
            content: Some(s.clone()),
            is_emphemeral: true,
            is_error: false,
            embed: None,
        };
        if s.starts_with("emp?") {
            reply.is_emphemeral = true;
            reply.content = Some(s.replace("emp?", "").trim().to_string());
        } else if s.starts_with("emp+err?") {
            reply.is_emphemeral = true;
            reply.is_error = true;
            reply.content = Some(s.replace("emp+err?", "").trim().to_string());
        } else if s.starts_with("err?") {
            reply.is_error = true;
            reply.content = Some(s.replace("err?", "").trim().to_string());
        }
        reply
    }
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

impl AsRef<Http> for SlashCommandContext {
    fn as_ref(&self) -> &Http { &self.client.http }
}

impl AsRef<Cache> for SlashCommandContext {
    fn as_ref(&self) -> &Cache { &self.client.cache }
}

impl CacheHttp for SlashCommandContext {
    fn http(&self) -> &Http { &self.client.http }

    fn cache(&self) -> Option<&std::sync::Arc<Cache>> { Some(&self.client.cache) }
}

impl SlashCommandContext {
    //async fn reply(&self, content: impl Display) -> crate::RuskyResult<()> {
    //    self.reply_embed(CreateEmbed::default().color(BLUE).description(content))
    //        .await?;
    //    Ok(())
    //}

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

    async fn reply<T>(&self, reply: T) -> crate::RuskyResult<()>
    where
        T: Into<ReplyData>, {
        let reply = reply.into();
        self.command
            .create_interaction_response(self, |r| {
                r.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        if reply.is_emphemeral {
                            message
                                .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL);
                        }
                        if reply.is_error {
                            if let Some(content) = reply.content {
                                message.add_embed(
                                    CreateEmbed::default()
                                        .color(MATERIAL_RED)
                                        .description(content)
                                        .to_owned(),
                                );
                            }
                        } else if let Some(content) = reply.content {
                            message.add_embed(
                                CreateEmbed::default()
                                    .color(BLUE)
                                    .description(content)
                                    .to_owned(),
                            );
                        } else if let Some(embed) = reply.embed {
                            message.add_embed(embed);
                        }
                        message
                    })
            })
            .await?;
        Ok(())
    }

    async fn replyin(&self, content: impl Display) -> crate::RuskyResult<()> {
        self.command
            .create_interaction_response(&self.client, |response| {
                response
                    .kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|message| {
                        message
                            .flags(InteractionApplicationCommandCallbackDataFlags::EPHEMERAL)
                            .add_embed(
                                CreateEmbed::default()
                                    .color(BLUE)
                                    .description(content)
                                    .to_owned(),
                            )
                    })
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
        use crate::commands::SlashCommand as _;
        let mut commands: HashMap<String, Box<dyn SlashCommand + Sync + Send>> = HashMap::new();

        acmd!(commands <== PingCommand);
        acmd!(commands <== CatCommand);
        acmd!(commands <== BanCommand);
        acmd!(commands <== UserInfoCommand);
        acmd!(commands <== SayCommand);
        Self { commands }
    }

    pub async fn run_command(&self, query: &str, context: &SlashCommandContext) {
        if let Some(command) = self.commands.get(query) {
            if let Err(err) = command.execute(context).await {
                let _ = context.reply(format!("err? Error: {err}")).await;
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
