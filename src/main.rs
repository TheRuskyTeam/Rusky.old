use clap::clap_app;
use rusky::{rusky::Rusky, setup};
#[tokio::main]
async fn main() {
    setup!();
    let config_file_path: String;
    let file_exists = |path: String| {
        if std::fs::metadata(path).is_ok() {
            Ok(())
        } else {
            Err(String::from("File doesn't exist."))
        }
    };
    let matches = clap_app!(Rusky =>
        (@setting SubcommandRequiredElseHelp)
        (version: env!("CARGO_PKG_VERSION"))
        (@subcommand run =>
             (about: "Run bot")
             (@arg config: -c --config [file] {file_exists} "Sets a custom config file")
        )
        (@subcommand check => // config file and Token
             (about: "TODO")
        )
        (@subcommand update =>
             (about: "TODO") // update eg Slash Commands
        )
    )
    .get_matches();
    if let Some(sub) = matches.subcommand_matches("run") {
        config_file_path = sub.value_of("config").unwrap_or("./Rusky.toml").to_string();
        let mut rusky = Rusky::new(&config_file_path)
            .await
            .expect("Failed to create client");
        rusky.login().await.expect("Client error");
    }
}
