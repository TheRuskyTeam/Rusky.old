pub mod util {
    #[macro_export]
    macro_rules! run {
    	($block:block catch $err:ident => $err_block:block) => {
        	let try_block = || -> rusky::typings::RuskyResult<()> {
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
}
pub mod bot {
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
    #[macro_export]
    macro_rules! async_main {
        ($block:block) => {
            #[tokio::main]
            async fn main() {
                $block
            }
        };
    }
}
