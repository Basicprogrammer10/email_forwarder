use std::path::PathBuf;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Config {
    pub imap: ImapConfig,
    pub smtp: SmtpConfig,
    pub forward: ForwardConfig,
}

#[derive(Deserialize, Debug)]
pub struct ImapConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub certificate: Option<PathBuf>,
}

#[derive(Deserialize, Debug)]
pub struct SmtpConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub certificate: Option<PathBuf>,
}

#[derive(Deserialize, Debug)]
pub struct ForwardConfig {
    pub to: Vec<String>,
}
