#[allow(dead_code)]
use crate::gpt_message::GptMessage;
use crate::gpt_response::GptResponse;
use crate::gpt_request::GptRequest;

use std::fs;
use serde::{Serialize, Deserialize};
use reqwest::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, AUTHORIZATION, HeaderMap};

#[derive(Serialize, Deserialize)]
pub struct Gpt {
    pub system_prompt: String,
  
}

#[allow(dead_code)]
impl Gpt {
    pub fn new(system_prompt: String) -> Self {
        Gpt {
            system_prompt
        }
    }

    pub fn default() -> Self {
        Gpt {
        system_prompt: String::from("
            You are a D&G storyteller with the story taking place in a post apocalyptic city overrun by zombies and robots,
            Car make and models don't have to be real world")
        }
    }

    pub fn prompt(&self, text:String)-> String {
        let endpoint: String = "https://api.openai.com/v1/chat/completions".to_string();
        let api_key: String = fs::read_to_string("api_key.conf").expect("Failed to read api_key.conf");


        let mut messages = Vec::new();
        messages.push(GptMessage{role: "assistant".to_string(), content:self.system_prompt.clone()});
        messages.push(GptMessage { role: "user".to_string(), content:text});
        let request = GptRequest { messages: messages , model: "gpt-3.5-turbo-1106".to_string()};
        let client = reqwest::blocking::Client::new();
        
        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));
        headers.insert(AUTHORIZATION, HeaderValue::from_str(&api_key).unwrap());
    
        let res = client.post(endpoint).headers(headers).json(&request).send().expect("Failed to send request")
        .json::<GptResponse>();
        
        match res {
            Ok(res) => {
                let choice = res.choices.get(0);
                match choice {
                    Some(c) => c.message.content.clone(),
                    None => String::from("No choice found"),
                }
            }
            Err(e) => format!("Error {}", e),
        }
    }
}
                            