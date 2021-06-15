use humansize::{file_size_opts, FileSize};
use rustc_version::version as ver;
use simple_process_stats::ProcessStats;
use yansi::Paint;

pub fn get_logger_message(level: String, target: String, date: String, message: String) -> String {
    let level: String = match level.as_str() {
        "DEBUG" => Paint::new("debug").bold().to_string(),
        "INFO" => Paint::cyan("information").bold().to_string(),
        "ERROR" => Paint::red("error").bold().to_string(),
        "WARN" => Paint::red("warn").underline().to_string(),
        _ => level,
    }
    .into();
    let message = Paint::new(message).bold();
    let log = format!("{}@{} on {}: {}", level, Paint::yellow(target).bold(), Paint::green(date).bold(), message);
    log
}

pub fn setup() -> Result<(), Box<dyn std::error::Error>> {
    fern::Dispatch::new()
        .format(|out, message, record| {
            out.finish(format_args!(
                "{}",
                get_logger_message(
                    record.level().to_string(),
                    record.target().to_string(),
                    chrono::Local::now()
                        .format("[%Y-%m-%d][%H:%M:%S]")
                        .to_string(),
                    message.to_string()
                )
            ))
        })
        .level(log::LevelFilter::Debug)
        .level_for("hyper", log::LevelFilter::Info)
        .chain(std::io::stdout())
        .chain(fern::log_file("rusky.log")?)
        .apply()?;

    Ok(())
}
#[macro_export]
macro_rules! run {
    ($block:block catch $err:ident => $err_block:block) => {
        let try_block = || -> Result<(), Box<dyn std::error::Error>> {
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
        async fn async_block() -> Result<(), Box<dyn std::error::Error>> {
            $block
            Ok(())
        };
        if let Err($err) = async_block().await {
            $err_block
        }
    }
}
pub fn get_version() -> String {
    let mut version = String::from("unknown");
    if let Ok(rust_version) = ver() {
        version = format!(
            "{}.{}.{}",
            rust_version.major, rust_version.minor, rust_version.patch
        );
    }
    version
}
pub async fn get_ram_usage() -> String {
    let mut ram_usage = String::from("unknown");
    if let Ok(stats) = ProcessStats::get().await {
        let bytes = stats.memory_usage_bytes;
        ram_usage = format!(
            "{}",
            bytes
                .file_size(file_size_opts::BINARY)
                .unwrap_or(String::from("unknown"))
        );
    }

    ram_usage
}
