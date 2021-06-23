use crate::{constants::urls::COVID19_BRAZIL_API_URL, typings::RuskyResult};
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
pub struct Covid19BrazilApiResponse {
    pub data: Vec<CityStatus>,
}
pub async fn fetch_data() -> RuskyResult<Covid19BrazilApiResponse> {
    let mut res = isahc::get_async(COVID19_BRAZIL_API_URL).await?;
    Ok(serde_json::from_str(&res.text().await?)?)
}
pub mod countries {
    use crate::{constants::urls::COVID19_BRAZIL_COUNTRIES_API_URL, typings::RuskyResult};
    use isahc::prelude::*;
    use serde::Deserialize;
    #[derive(Deserialize)]
    pub struct CountryStatus {}
    #[derive(Deserialize)]
    pub struct Covid19BrazilCountryApiResponse {
        pub data: Vec<CountryStatus>,
    }
    pub async fn fetch_data() -> RuskyResult<Covid19BrazilCountryApiResponse> {
        let mut res = isahc::get_async(COVID19_BRAZIL_COUNTRIES_API_URL).await?;

        Ok(serde_json::from_str(&res.text().await?)?)
    }
}
