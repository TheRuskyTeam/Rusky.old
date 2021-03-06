use std::error::Error;

pub mod apis;
pub mod commands;
pub mod config;
pub mod constants;
pub mod containers;
pub mod errors;
pub mod events;
pub mod macros;
pub mod rusky;
pub mod utils;

pub type AnyError = Box<dyn Error + Sync + Send + 'static>;
pub type RuskyResult<T> = Result<T, AnyError>;
