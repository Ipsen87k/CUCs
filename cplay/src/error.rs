use std::io;

use rodio::{decoder::DecoderError, PlayError, StreamError};

#[derive(Debug)]
pub enum Error {
    IoError(io::Error),
    StreamError(StreamError),
    PlayError(PlayError),
    DecoderError(DecoderError),
}