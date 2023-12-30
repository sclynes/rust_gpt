use serde::Deserialize;
use crate::gpt_message::GptMessage;

#[derive(Debug, Deserialize)]
pub struct GptResponse {
    pub id: String,
    pub object: String,
    pub created: usize,
    pub model: String,
    pub choices: Vec<GptResponseChoice>,
}

#[derive(Debug, Deserialize)]
pub struct GptResponseChoice {
    pub index: usize,
    pub message: GptMessage,
    pub finish_reason: String,
}