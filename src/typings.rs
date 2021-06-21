use std::error::Error;
pub type RuskyError = Box<dyn Error + Sync + Send + 'static>;
pub type RuskyResult<T> = Result<T, RuskyError>;
