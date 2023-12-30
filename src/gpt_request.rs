use serde::Serialize;

use crate::gpt_message::GptMessage;

#[derive(Debug, Serialize)]
pub struct GptRequest {
    pub messages: Vec<GptMessage>,
    pub model: String,
}

impl GptRequest {
    pub fn new(message: String, model: String) -> Self {
        let message = GptMessage::new(message);
        let messages = vec![message];
        Self { messages, model }
    }
    
}