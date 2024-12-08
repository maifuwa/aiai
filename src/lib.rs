use reqwest::Error;
use serde_json::Value;

pub mod args;

pub mod constant;

pub async fn just_content(_: String) -> Result<String, Error> {
    let client = reqwest::Client::new();
    let body = r#"{
    "model": "llama3.2",
    "prompt": "String(2024-01-08 00:00) => date in java",
    "stream": false
}"#;
    let res = client.post(constant::OLLAMA_URL).body(body).send().await?;
    let res_text = res.text().await?;
    let res_text: Value = serde_json::from_str(&res_text).unwrap();
    Ok(res_text["response"].as_str().unwrap().to_string())
}

pub async fn println_response(response: String) {
    termimad::print_text(&response);
}
