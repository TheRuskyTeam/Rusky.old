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
                rusky::util::format_log_message(
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
        ($hash:ident <== $command:ident) => {
            $hash.insert($command::information(&$command).name, Box::new($command))
        };
    }
    #[macro_export]
    macro_rules! slash_command {
        (for: $struct:ident,
        name: $name:expr,
        description: $description:expr,
        options: $options:expr,
        execute: ($context:ident) => $command:block) => {
            #[serenity::async_trait]
            impl crate::commands::SlashCommand for $struct {
                fn information(&self) ->  crate::commands::SlashCommandMetaData
                {
                    crate::commands::SlashCommandMetaData
                    {
                        name: $name.into(),
                        description: $description.into(),
                        options: $options
                    }
                }
                async fn execute(&self, $context: &crate::commands::SlashCommandContext) -> crate::RuskyResult<()>
                {
                    $command
                    Ok(())
                }
            }
        };
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
