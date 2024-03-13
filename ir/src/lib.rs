use std::error::Error;

pub mod ir_core;
pub mod ir_args;
pub mod chrono_wrap;
pub mod spinner;
pub mod util;
pub(crate) type CResult<T> = Result<T,Box<dyn Error>>;