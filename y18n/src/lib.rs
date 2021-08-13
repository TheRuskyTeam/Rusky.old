use glob::glob;
use std::{
    error::Error,
    fmt::Display,
    fs::{self, File},
};
pub type AnyError = Box<dyn Error + Sync + Send + 'static>;
#[derive(Debug)]
pub struct CannotFindStringError;
impl Display for CannotFindStringError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "cannot find string")
    }
}

pub struct YamlTranslation {
    path: String,
}
impl YamlTranslation {
    pub fn new<T>(path: T) -> Self
    where
        T: ToString, {
        Self {
            path: path.to_string(),
        }
    }

    pub fn path<T>(&mut self, path: T)
    where
        T: ToString, {
        self.path = path.to_string();
    }

    pub fn load(&mut self) -> Result<(), AnyError> {
        for path in glob(&format!("{}/*.yml", self.path))?.filter_map(Result::ok) {
            let file = File::open(path)?;
        }
        Ok(())
    }

    // t("userinfo.embed.title", ["yxqsnz#0000"])
    pub fn t<T>(q: &str, vars: Vec<T>) -> Result<String, CannotFindStringError>
    where
        T: ToString, {
        Ok("xD".to_string())
    }
}
