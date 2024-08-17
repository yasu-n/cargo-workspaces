use std::io;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("already exists crate: {0}")]
    AlreadyExistsCrate(String),

    #[error("{0}")]
    CargoError(#[from] anyhow::Error),

    #[error("{0}")]
    Io(#[from] io::Error),

    #[error("{0}")]
    Toml(#[from] toml_edit::TomlError),

    #[error("{0}")]
    Git(#[from] git2::Error),
}
