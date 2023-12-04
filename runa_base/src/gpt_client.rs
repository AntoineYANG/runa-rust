use reqwest;
use std::env;
use std::error::Error;

pub async fn fetch_gpt_response(prompt: &str) -> Result<String, Box<dyn Error>> {
    let api_key = env::var("GPT_API_KEY").expect("GPT_API_KEY not set");
    let client = reqwest::Client::new();

    let response = client.post("https://api.openai.com/v1/engines/gpt-3.5-turbo/completions")
        .header("Authorization", format!("Bearer {}", api_key))
        .header("Content-Type", "application/json")
        .body(format!("{{\"prompt\":\"{}\", \"max_tokens\": 60}}", prompt))
        .send()
        .await?;

    let response_text = response.text().await?;
    Ok(response_text)
}
