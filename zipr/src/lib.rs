use std::error::Error;

pub mod zipr;


pub(crate) type CResult<T> = Result<T,Box<dyn Error>>; 