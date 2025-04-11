#![allow(clippy::module_name_repetitions)]

use serde::Deserialize;
use std::{env, fs, io, path::Path};
use thiserror::Error;

#[derive(Deserialize)]
pub struct ServeConfig {
    pub rest_addr: Box<str>,
}

#[derive(Deserialize)]
pub struct LoggingConfig {
    pub dirs: Box<str>,
}

#[derive(Deserialize)]
pub struct IdentityProviderConfig {
    pub client_id: String,
    pub client_secret: String,
    pub issuer_url: String,
}

#[derive(Deserialize)]
pub struct Config {
    pub serve: ServeConfig,
    pub logging: LoggingConfig,
    pub identity_provider: IdentityProviderConfig,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error(transparent)]
    IO(#[from] io::Error),
    #[error(transparent)]
    Toml(#[from] toml::de::Error),
}

/// # Panics
/// Panics if `CONFIG_PATH` env variable not specified
#[must_use]
pub fn get_path() -> Box<str> {
    let Ok(path) = env::var("CONFIG_PATH") else {
        panic!("`CONFIG_PATH` env variable not specified!");
    };

    path.into_boxed_str()
}

#[allow(clippy::missing_errors_doc)]
pub fn parse_from_fs(path: impl AsRef<Path>) -> Result<Config, ParseError> {
    let raw: String = fs::read_to_string(path)?;

    toml::from_str(&raw).map_err(Into::into)
}
