use error::Error;

pub mod error;
pub mod cplay_args;
pub mod cplay_core;

pub type CResult<T> = Result<T,Error>;