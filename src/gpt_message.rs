use serde::{Deserialize, Serialize};


#[derive(Debug, Deserialize, Serialize)]
pub struct GptMessage {
    pub role:String,
    pub content:String
}

impl GptMessage {
    pub fn new(content: String) -> Self {
        Self {
            role: "app_user".to_string(),
            content
        }
    }
    
}
