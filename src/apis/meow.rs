use serde::Deserialize;

use crate::{constants::MEOW_API_URL, RuskyResult};

#[derive(Deserialize)]
pub struct MeowApi {
    pub file: String,
}

impl MeowApi {
    pub async fn fetch() -> RuskyResult<Self> {
        let res = reqwest::get(MEOW_API_URL).await?;
        let content = res.text().await?;
        Ok(serde_json::from_str(&content)?)
    }
}
