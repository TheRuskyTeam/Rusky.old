#[macro_export]
macro_rules! run {
    	($block:block catch $err:ident => $err_block:block) => {
        	let try_block = || -> rusky::RuskyResult<()> {
            	$block
            	Ok(())
        	};
        	if let Err($err) = try_block() {
            	$err_block
        	}
    	}
	}
#[macro_export]
macro_rules! async_run {
        ($block:block catch $err:ident => $err_block:block) => {
            async fn async_block() -> rusky::typings::RuskyResult<()> {
                $block
                Ok(())
            };
            if let Err($err) = async_block().await {
                $err_block
            }
        }
    }
#[macro_export]
macro_rules! setup {
    () => {
        $crate::run! {{
                 fern::Dispatch::new()
                    .format(|out, message, record| {
                        out.finish(format_args!(
                        "{}",
                rusky::utils::format_log_message(
                    record.level().to_string(),
                    record.target().to_string(),
                    chrono::Local::now()
                        .format("%H:%M:%S/%Y-%m-%d")
                        .to_string(),
                    message.to_string()
                )
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("hyper", log::LevelFilter::Info)
        .chain(std::io::stdout())
        // .chain(fern::log_file("rusky.log")?)
        .apply()?;
            } catch err => {
                panic!("{:?}", err);
            }}
    };
}
pub mod commands {
    #[macro_export]
    macro_rules! acmd {
        ($hash:ident <= = $command:ident) => {
            $hash.insert($command::information(&$command).name, Box::new($command))
        };
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
    #[macro_export]
    macro_rules! slash {
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
        };
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
        };

    }
    #[macro_export]
    macro_rules! get_arg {
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
        }};
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
        }};
    }
}

pub mod util {
    #[macro_export]
    macro_rules! nh {
        () => {
            std::collections::HashMap::new()
        };
    }
}
