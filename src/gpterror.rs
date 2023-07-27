use std::fmt;
use reqwest;
use serde_json;
use toml;

#[derive(Debug)]
pub enum GPTError {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    ConfigParse(toml::de::Error),
    Config(String, String),
    Other(String),
}

impl Into<String> for GPTError {
    fn into(self) -> String {
        format!("{}", self)
    }
}

impl fmt::Display for GPTError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GPTError::Reqwest(err) => write!(f, "Reqwest error: {}", err),
            GPTError::Serde(err) => write!(f, "Serde error: {}", err),
            GPTError::ConfigParse(err) => write!(f, "Config (toml parsing) error: {}", err),
            GPTError::Config(err, path) => write!(f, "Config error: {} [path: {}]", err, path),
            GPTError::Other(err) => write!(f, "Other error: {}", err),
        }
    }
}

impl From<reqwest::Error> for GPTError {
    fn from(err: reqwest::Error) -> Self {
        GPTError::Reqwest(err)
    }
}

impl From<toml::de::Error> for GPTError {
    fn from(err: toml::de::Error) -> Self {
        GPTError::ConfigParse(err)
    }
}

impl From<serde_json::Error> for GPTError {
    fn from(err: serde_json::Error) -> Self {
        GPTError::Serde(err)
    }
}