use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO ERROR: {0}")]
    IoError(#[from] std::io::Error),

    #[error("BITCOIN ENCODING ERROR: {0}")]
    Encode(#[from] bitcoin::consensus::encode::Error),

    #[error("ERROR: {0}")]
    CustomError(String),
}
