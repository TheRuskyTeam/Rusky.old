pub mod commands {
    pub macro acmd($hash:ident <= = $command:ident) {
        $hash.insert($command::information(&$command).name, Box::new($command))
    }
    #[macro_export]
    macro_rules! __slash_command_option {
        ($option_name:expr, User, $option_description:expr) => {{
            serenity::builder::CreateApplicationCommandOption::default()
                .name($option_name)
                .description($option_description)
                .required(true)
                .kind(serenity::model::interactions::application_command::ApplicationCommandOptionType::User)
        }};
        ($option_name:expr, Text, $option_description:expr) => {{
            serenity::builder::CreateApplicationCommandOption::default()
            .name($option_name)
            .description($option_description)
            .required(true)
            .kind(serenity::model::interactions::application_command::ApplicationCommandOptionType::String)
        }};
        ($option_name:expr, OptionChannel, $option_description:expr) => {{
            serenity::builder::CreateApplicationCommandOption::default()
                .name($option_name)
                .description($option_description)
                .kind(serenity::model::interactions::application_command::ApplicationCommandOptionType::Channel)
        }};
        ($option_name:expr, Channel, $option_description:expr) => {{
            serenity::builder::CreateApplicationCommandOption::default()
                .name($option_name)
                .description($option_description)
                .required(true)
                .kind(serenity::model::interactions::application_command::ApplicationCommandOptionType::Channel)
        }};
        ($option_name:expr, OptionUser, $option_description:expr) => {{
            sserenity::builder::CreateApplicationCommandOption::default()
                .name($option_name)
                .description($option_description)
                .kind(serenity::model::interactions::application_command::ApplicationCommandOptionType::User)
        }};
        ($option_name:expr, OptionText, $option_description:expr) => {{
            serenity::builder::CreateApplicationCommandOption::default()
            .name($option_name)
            .description($option_description)
            .kind(serenity::model::interactions::application_command::ApplicationCommandOptionType::String)
        }};

    }
    pub macro slash {
        (
            $to_impl:ident =>
            (@name: $slash_command_name:expr)
            (@description: $slash_command_description:expr)
            (@execute: $func:ident)
        ) => {
            #[serenity::async_trait]
            impl crate::commands::SlashCommand for $to_impl {
                 fn information(&self) -> crate::commands::SlashCommandMetaData {
                    crate::commands::SlashCommandMetaData {
                        name: $slash_command_name.into(),
                        description: $slash_command_description.into(),
                        options: None
                    }
                }
                async fn execute(&self, c: &crate::commands::SlashCommandContext) -> crate::RuskyResult<()> {
                    $func(c).await
                }
            }
        },
        (
            $to_impl:ident =>
            (@name: $slash_command_name:expr)
            (@description: $slash_command_description:expr)
            $(
              (@arg $arg_name:expr, $arg_type:ident: $arg_description:expr)
            )*
            (@execute: $func:ident)
        ) => {
             #[serenity::async_trait]
            impl crate::commands::SlashCommand for $to_impl {
                 fn information(&self) ->  crate::commands::SlashCommandMetaData {
                    crate::commands::SlashCommandMetaData {
                        name: $slash_command_name.into(),
                        description: $slash_command_description.into(),
                        options: Some({
                            let mut opts = vec![];
                            $(
                                opts.push(crate::__slash_command_option!($arg_name, $arg_type, $arg_description).to_owned());
                            )*
                            opts
                        })
                    }
                }
                async fn execute(&self, c: &crate::commands::SlashCommandContext) -> crate::RuskyResult<()> {
                    $func(c).await
                }
            }
        },

    }
    pub macro get_arg {
        ($variable:ident, Text ?? $default_text_value:expr) => {{
            if let Some(to_get) = $variable {
                if let Some(serenity::model::interactions::application_command
                    ::ApplicationCommandInteractionDataOptionValue::String(s)) = &to_get.resolved {
                    s.to_string()
                } else {
                    $default_text_value.to_string()
                }
            } else {
                $default_text_value.to_string()
            }
        }},
        ($variable:ident, Channel) => {{
            if let Some(to_get) = $variable {
                    if let Some(serenity::model::interactions::application_command
                    ::ApplicationCommandInteractionDataOptionValue
                    ::Channel(ch)) = &to_get.resolved {
                       Some(ch)
                    } else {
                        None
                    }
            } else {
                None
            }
        }},
        ($variable:ident, User) => {{
            if let Some(to_get) = $variable {
                    if let Some(serenity::model::interactions::application_command
                    ::ApplicationCommandInteractionDataOptionValue
                    ::User(user, member)) = &to_get.resolved {
                       Some((user, member))
                    } else {
                        None
                    }
            } else {
                None
            }
        }},
    }
}

pub mod util {}
