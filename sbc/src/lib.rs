use std::error::Error;

pub mod sc_args;
pub mod sc_core;

pub(crate) type  CResult<T> = Result<T,Box<dyn Error>>;