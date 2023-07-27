use reqwest::Client;
use serde::{ Serialize, Deserialize };
use serde_json::json;
use crate::gpterror;
use crate::config::Config;

#[derive(Debug, Clone)]
pub struct OpenAI {
    pub config:Config,
    pub uri:String,
    pub chatlog:Vec<(String, String)>,
    pub client:Client,
}

impl OpenAI {
    pub fn new(config:Config) -> OpenAI {
        OpenAI {
            config,
            uri: "https://api.openai.com/v1/chat/completions".into(),
            chatlog:Vec::<(String, String)>::new(),
            client: Client::new(),
        }
    }

    pub async fn immutable_get_response(&self, prompt:String, model_info:Option<ModelInfo>) -> Result<String, gpterror::GPTError>  {
        let prompt_msg = ModelInfoMessage { role: "user".into(), content: prompt.clone() };
        let message = match model_info {
            None => { 
                json!({
                        "model": "gpt-3.5-turbo",
                        "messages": [{"role": "user", "content": prompt.clone()}],
                        "temperature": 0.7,
                    })
                },
            Some(model_info) => {
                let mut full_msg = model_info.clone();
                full_msg.messages.push(prompt_msg);
                serde_json::to_value(full_msg)?
            }
        };

        let res = self.client.post(&self.uri)
            .header("Authorization", format!("Bearer {}", self.config.openai.api_key))
            .json(&message)
            .send()
            .await?
            .error_for_status()?;

        let response:ApiResponse = res.json().await?;
        let response_string = serde_json::to_string(&response)?;
    
        Ok(response_string)
    }

    pub async fn get_response(&mut self, prompt:String) -> Result<String, reqwest::Error>  {
        let message = json!({
            "model": "gpt-3.5-turbo",
            "messages": [{"role": "user", "content": prompt}],
            "temperature": 0.7,
        });

        let res = self.client.post(&self.uri)
            .header("Authorization", format!("Bearer {}", self.config.openai.api_key))
            .json(&message)
            .send()
            .await?;

        let response:ApiResponse = res.json().await?;

        let response_string = response.choices[0].message.content.clone();
        self.chatlog.push((prompt, response_string));

        Ok(response.choices[0].message.content.clone())

    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiResponse {
    pub choices: Vec<Choice>,
    pub created: u64,
    pub id: String,
    pub model: String,
    pub object: String,
    pub usage: Usage,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Choice {
    #[serde(rename = "finish_reason")]
    pub finish_reason: String,
    pub index: u64,
    pub message: Message,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub content: String,
    pub role: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Usage {
    #[serde(rename = "completion_tokens")]
    pub completion_tokens: u64,
    #[serde(rename = "prompt_tokens")]
    pub prompt_tokens: u64,
    #[serde(rename = "total_tokens")]
    pub total_tokens: u64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelInfo {
    pub model: String,
    pub messages: Vec<ModelInfoMessage>,
    pub temperature: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ModelInfoMessage {
    pub role: String,
    pub content: String,
}

impl ModelInfoMessage {
    pub fn new(role:String, content:String) -> ModelInfoMessage {
        ModelInfoMessage {
            role,
            content,
        }
    }
}