use yansi::Paint;
pub fn format_log_message(level: String, target: String, date: String, message: String) -> String {
    let level: String = match level.as_str() {
        "DEBUG" => Paint::new("debug").bold().to_string(),
        "INFO" => Paint::cyan("information").bold().to_string(),
        "ERROR" => Paint::red("error").bold().to_string(),
        "WARN" => Paint::red("warn").underline().to_string(),
        _ => level,
    };
    let message = Paint::new(message).bold();
    let log = format!(
        "{}@{} on {}: {}",
        level,
        Paint::yellow(target).bold(),
        Paint::green(date).bold(),
        message
    );
    log
}
pub mod misc {
    use rustc_version::version as ver;
    pub fn get_rust_version() -> String {
        let mut version = String::from("unknown");
        if let Ok(rust_version) = ver() {
            version = format!(
                "{}.{}.{}",
                rust_version.major, rust_version.minor, rust_version.patch
            );
        }
        version
    }
}
pub mod image {
    use rand::Rng;

    pub fn random_default_avatar() -> String {
        let mut random = rand::thread_rng();
        format!(
            "https://cdn.discordapp.com/embed/avatars/{}.png",
            random.gen_range(1..5)
        )
    }
}
pub mod discord_time {
    pub fn get_relative_time_string(time: i64) -> String {
        format!("<t:{}:R>", time)
    }
}
pub mod discord_user {
    use crate::models::RuskyResult;
    use serenity::client::Context;
    use serenity::model::guild::Guild;
    use serenity::model::prelude::UserId;
    use serenity::model::user::User;

    pub async fn get_user_by_id(context: &Context, id: u64) -> RuskyResult<User> {
        match context.cache.user(id).await {
            Some(user) => Ok(user),
            None => Ok(context.http.get_user(id).await?),
        }
    }

    pub fn format_client_status(status: &[String]) -> String {
        if status.is_empty() {
            return String::from("Nenhum");
        }
        let mut message = String::new();
        for device in status {
            let device = match device.as_str() {
                "web" => "Navegador",
                "desktop" => "PC",
                "mobile" => "Celular",
                _ => "Desconhecido",
            };
            if message.is_empty() {
                message += device;
            } else {
                message += &format!(", {}", device);
            }
        }
        message
    }
    pub async fn get_client_status(guild: &Guild, user_id: &UserId) -> Vec<String> {
        let mut status: Vec<_> = vec![];
        if let Some(presence) = guild.presences.get(user_id) {
            if let Some(client_status) = &presence.client_status {
                if client_status.web.is_some() {
                    status.push("web".into());
                }
                if client_status.mobile.is_some() {
                    status.push("mobile".into());
                }
                if client_status.desktop.is_some() {
                    status.push("desktop".into())
                }
            }
        }

        status
    }
}
