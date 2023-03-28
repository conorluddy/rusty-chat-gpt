use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::env;

async fn get_chatgpt(_message: String) -> Result<ChatGPTCompletionResponse, reqwest::Error> {
    let url = "https://api.openai.com/v1/chat/completions";
    let client = reqwest::Client::new();
    let api_key = env::var("CHATGPT_API_KEY").unwrap();

    let request = CompletionRequest {
        model: "gpt-3.5-turbo".to_string(),
        messages: vec![
            Message { role: "user".to_string(), content: "haiku from a summary of current weather forecast data: coords [53.343, -6.263],Air pressure is 1006.6 hPa. Air temperature is 7.7 degrees Celsius. Cloud area fraction is 96.1 percent. relative_humidity: 88.7, wind_from_direction: 41.7, wind_speed: 5.8,".to_string() }
        ],
        temperature: 0.2,
    };

    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", format!("Bearer {}", api_key).to_string())
        .json(&request)
        .send()
        .await?;

    let res = response.json::<ChatGPTCompletionResponse>().await?;

    Ok(res)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();
    let res = get_chatgpt("test".to_string()).await?;
    let obj = json!(&res);

    println!("{}", serde_json::to_string_pretty(&obj).unwrap());
    // println!("{}", summary);

    Ok(())
}

// Request

#[derive(Serialize)]
struct CompletionRequest {
    model: String,
    messages: Vec<Message>,
    temperature: f32,
}

// Response

#[derive(Debug, Serialize, Deserialize)]
struct ChatGPTCompletionResponse {
    id: String,
    object: String,
    created: u64,
    model: String,
    usage: Usage,
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Usage {
    prompt_tokens: u64,
    completion_tokens: u64,
    total_tokens: u64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    message: Message,
    finish_reason: String,
    index: u64,
}

// Both

#[derive(Debug, Serialize, Deserialize)]
struct Message {
    role: String,
    content: String,
}
