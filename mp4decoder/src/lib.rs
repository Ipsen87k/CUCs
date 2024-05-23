use std::error::Error;

pub mod mp4decoder_args;
pub mod mp4decoder_core;

pub(crate) type CResult<T>=Result<T,Box<dyn Error>>;