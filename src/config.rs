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
            Err(e)=> Err(GPTError::ConfigParse(e)),
            Ok(cfg) => Ok(cfg),
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