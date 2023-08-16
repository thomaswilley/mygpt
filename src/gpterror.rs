use std::fmt;
use reqwest;
use serde_json;
use toml;

#[derive(Debug)]
pub enum GPTError {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
    ConfigDeserialize(toml::de::Error),
    ConfigSerialize(toml::ser::Error),
    Config(String, String),
    Other(String),
    IOError(std::io::Error),
}

/* 
impl Into<String> for GPTError {
    fn into(self) -> String {
        format!("{}", self)
    }
}
*/

impl fmt::Display for GPTError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GPTError::Reqwest(err) => write!(f, "Reqwest error: {}", err),
            GPTError::Serde(err) => write!(f, "Serde error: {}", err),
            GPTError::ConfigDeserialize(err) => write!(f, "Unable to deserailize config: {}", err),
            GPTError::ConfigSerialize(err) => write!(f, "Unable to serialize config: {}", err),
            GPTError::Config(err, path) => write!(f, "Config error: {} [path: {}]", err, path),
            GPTError::Other(err) => write!(f, "Other error: {}", err),
            GPTError::IOError(err) => write!(f, "IO Error: {}", err),
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
        GPTError::ConfigDeserialize(err)
    }
}

impl From<toml::ser::Error> for GPTError {
    fn from(err: toml::ser::Error) -> Self {
        GPTError::ConfigSerialize(err)
    }
}

impl From<serde_json::Error> for GPTError {
    fn from(err: serde_json::Error) -> Self {
        GPTError::Serde(err)
    }
}

impl From<std::io::Error> for GPTError {
    fn from(err: std::io::Error) -> Self {
        GPTError::IOError(err)
    }
}

impl From<GPTError> for String {
    fn from(err: GPTError) -> String {
        format!("GPTError: {}", err)
    }
}