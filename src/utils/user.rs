use crate::{errors::NoneError, RuskyResult};
use rand::Rng;
use serde::Deserialize;
use std::env::var as get_env_var;
#[derive(Deserialize, Debug)]
pub struct UserProfile {
    pub banner: Option<String>,
    pub id: String,
    pub banner_color: Option<String>,
    pub accent_color: Option<u64>,
}
pub async fn get_user_profile(discord_user_id: u64) -> RuskyResult<UserProfile> {
    let token = get_env_var("DISCORD_TOKEN")?;
    let request_url = format!("https://discord.com/api/v9/users/{discord_user_id}");
    let requests = reqwest::Client::new();
    let res = requests
        .get(request_url)
        .header("Authorization", format!("Bot {token}"))
        .send()
        .await?;
    let raw_json = res.text().await?;
    Ok(serde_json::from_str(&raw_json)?)
}
pub async fn get_user_banner_url(profile: &UserProfile) -> RuskyResult<String> {
    if profile.banner.is_some() {
        let banner_url = profile.banner.as_ref().unwrap();
        let id = &profile.id;
        let request_urls = vec![
            format!("https://cdn.discordapp.com/banners/{id}/{banner_url}.gif?size=512"),
            format!("https://cdn.discordapp.com/banners/{id}/{banner_url}.png?size=512"),
        ];
        for url in request_urls {
            if reqwest::get(&url).await?.status().is_success() {
                return Ok(url);
            }
        }
        return Err(Box::new(NoneError));
    } else {
        Err(Box::new(NoneError))
    }
}
pub fn random_discord_default_avatar() -> String {
    let mut random = rand::thread_rng();
    format!(
        "https://cdn.discordapp.com/embed/avatars/{}.png",
        random.gen_range(1..5)
    )
}
#[cfg(test)]
mod tests {
    use super::get_user_profile;
    use std::io::{stdout, Write};
    use tokio_test::block_on;
    #[test]
    fn try_get_user_profile() {
        let profile = block_on(get_user_profile(567853754825572352));
        assert!(profile.is_ok());
        assert!(stdout().write(format!("{:#?}", profile).as_bytes()).is_ok());
    }
}
