use crate::constants::urls::COVID19_BRAZIL_API_URL;
use crate::typings::RuskyResult;
use isahc::prelude::*;
use serde::Deserialize;
#[derive(Deserialize)]
pub struct CityStatus {
    pub state: String,
    pub cases: u32,
    pub deaths: u32,
    pub suspects: u32,
    pub refuses: u32,
}
#[derive(Deserialize)]
pub struct Data {
    pub states: Vec<CityStatus>,
}
#[derive(Deserialize)]
pub struct Covid19BrazilApi {
    pub data: Vec<CityStatus>,
}
pub async fn fetch_data() -> RuskyResult<Covid19BrazilApi> {
    let mut res = isahc::get_async(COVID19_BRAZIL_API_URL).await?;
    Ok(serde_json::from_str(&res.text().await?)?)
}
