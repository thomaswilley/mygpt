use crate::gpterror::GPTError;
use serde::{ Serialize, Deserialize };
use std::fs;
use toml;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub general: Option<GeneralConfig>,
    pub openai: OpenAiConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeneralConfig {
    pub username: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OpenAiConfig {
    pub api_key: String,
}

impl Config {

    pub fn new(config_path:Option<&str>) -> Result<Config, GPTError> {
        let mut path:&str= "./mygpt.conf";
        if config_path.is_some() { path = config_path.unwrap() }
        let contents = fs::read_to_string(path).unwrap_or("".to_string());

        let config: Result<Config, toml::de::Error> = toml::from_str(&contents);
        match config {
            Err(e)=> Err(GPTError::ConfigDeserialize(e)),
            Ok(cfg) => Ok(cfg),
        }
    }

    pub fn save(&self, config_path:Option<&str>) -> Result<String, GPTError> {
        let mut path:&str= "./mygpt.conf";
        if config_path.is_some() { path = config_path.unwrap() }

        let config_str: Result<String, toml::ser::Error> = toml::to_string(&self);

        match config_str {
            Err(e)=> Err(GPTError::ConfigSerialize(e)),
            Ok(cfg_str) => {
                fs::write(path, cfg_str).map_err(GPTError::IOError)?;
                let absolute_path = fs::canonicalize(path).map_err(GPTError::IOError)?;
                Ok(absolute_path.display().to_string())
            },
        }
    }

}

fn is_file(path: String) -> Result<(), String> {
    match fs::metadata(&path) {
        Ok(metadata) => {
            if metadata.is_file() {
                Ok(())
            } else {
                Err(format!("{} is not a file", path))
            }
        },
        Err(_) => Err(format!("Could not read file at {}", path)),
    }
}