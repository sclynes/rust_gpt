
use std::{io::Cursor, fs};
use base64::{Engine as _, alphabet, engine::{self, general_purpose}};
use uuid::Uuid;
use reqwest::header::{HeaderValue, CONTENT_TYPE, CONTENT_LENGTH, HeaderMap, AUTHORIZATION};
use rodio::{OutputStream, source::Source};

mod gpt_message;
mod gpt_request;
mod gpt_response;
mod gpt;


//use car::Car;
//#[tokio::main]
fn main() {
    let gpt = gpt::Gpt::new(String::from("
    You are a storyteller with the story taking place in a post apocalyptic city overrun by zombies and robots,
    it is lashing rain and cold,
    respond in less than 100 words 
    "));
    let res = gpt.prompt(String::from("you hear somebody behind you?"));

    let clean = res.chars().filter(|c| !c.is_control()).collect::<String>();
    create_image(clean.clone());

    let endpoint: String = "https://api.openai.com/v1/audio/speech".to_string();
    let api_key: String = fs::read_to_string("api_key.conf").expect("Failed to read api_key.conf");

    let client = reqwest::blocking::Client::new();
    
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&api_key).unwrap());


    let json: serde_json::Value = serde_json::from_str(&format!(r#"{{
        "model": "tts-1",
        "input": "{}",
        "voice": "onyx"
    }}"#, clean)).unwrap();

    let mp3 = client.post(endpoint).headers(headers).json(&json).send().expect("Failed to send request").bytes().unwrap();
    let uuid = Uuid::new_v4();

    let txt_dir = "gen/text";
    let mp3_dir = "gen/audio";
    fs::create_dir_all(txt_dir).expect("Failed to create directory");
    fs::create_dir_all(mp3_dir).expect("Failed to create directory");

    let txt_filename = format!("{}/{}.txt", txt_dir, uuid);
    let mp3_filename = format!("{}/{}.mp3", mp3_dir, uuid);

    fs::write(txt_filename,  &res).expect("Unable to write txt file");
    fs::write(mp3_filename, &mp3).expect("Unable to write mp3 file");

    let cursor = Cursor::new(mp3);
    let source = rodio::Decoder::new(cursor).unwrap();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    stream_handle.play_raw(source.convert_samples());
    std::thread::sleep(std::time::Duration::from_secs(600));
}

fn create_image(mut prompt:String) {
    let endpoint: String = "https://api.openai.com/v1/images/generations".to_string();
    let api_key: String = fs::read_to_string("api_key.conf").expect("Failed to read api_key.conf");

    let client = reqwest::blocking::Client::new();
    
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));
    headers.insert(AUTHORIZATION, HeaderValue::from_str(&api_key).unwrap());

   // prompt.push_str(", in a pixel art style like flashback on the snes");
   // let clean = prompt.chars().filter(|c| !c.is_control()).collect::<String>();
   //prompt.push_str(" in a pixel art style like flashback on the snes");

    let json: serde_json::Value = serde_json::from_str(&format!(r#"{{
        "prompt": "{}",
        "size": "512x512",
        "response_format": "b64_json"
    }}"#, prompt)).unwrap();


    
    let json = client.post(endpoint).headers(headers).json(&json).send().expect("Failed to send request");
    
    //let image_data = Enginedecode(&image_data_base64);
    let bytes = general_purpose::STANDARD.decode(image_data_base64).unwrap();

    let image = image::load_from_memory(&bytes).unwrap();
    image.save("images/output.png").unwrap();
}