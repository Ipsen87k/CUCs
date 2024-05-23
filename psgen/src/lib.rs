use std::error::Error;

pub mod psgen_core;
pub mod psgen_args;

pub(crate) type CResult<T> = Result<T,Box<dyn Error>>;