use std::{error::Error, fmt};

#[derive(Debug, Clone)]
pub struct NoneError;

impl fmt::Display for NoneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result { write!(f, "variable is None") }
}

impl Error for NoneError {}

unsafe impl Sync for NoneError {}

unsafe impl Send for NoneError {}
