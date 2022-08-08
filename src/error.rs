use std::{num, result};
use thiserror::Error;
pub type Result<T> = result::Result<T, ApplicationError>;
#[derive(Error, Debug)]
pub enum ApplicationError {
    #[error("Sqlite error: {0}")]
    Sqlite(#[from] rusqlite::Error),
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Walkdir error: {0}")]
    WalkDir(#[from] walkdir::Error),
    #[error("ParseInt error: {0}")]
    ParseInt(#[from] num::ParseIntError),
}
