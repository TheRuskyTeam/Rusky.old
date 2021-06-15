use crate::models::RuskyConfig;
use std::fs::File;
use std::io::Read;
pub fn load(path: &str) -> Result<RuskyConfig, Box<dyn std::error::Error>> {
    let mut buf = String::new();
    let mut file = File::open(path)?;
    file.read_to_string(&mut buf)?;
    let config: RuskyConfig = toml::from_str(&buf)?;

    Ok(config)
}
