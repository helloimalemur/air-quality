use std::process;
use reqwest::ClientBuilder;
use reqwest::header::{CONTENT_TYPE};



pub async fn send_discord(discord: String, email: String, max_aqi: String, current_aqi: String) {
    println!("{} {} {}", discord, email, max_aqi);
    let message = format!("OVER AQI {} > {}", current_aqi, max_aqi);
    if discord.contains("https://discord.com/api/webhooks/") {
        println!("alert fired for {}", email);
        send(discord.as_str(), "AQ", message).await;
    } else {
        println!("invalid discord settings for {}", email);
    }

}

pub async fn send(api_url: &str, username: &str, message: String) {

    let json_message = match jsonify(username, message) {
        Ok(j) => j,
        Err(_e) => process::exit(3)
    };
    push_message(api_url, json_message).await;

}

async fn push_message(api_url: &str, json_message: Value) {
    let client = ClientBuilder::new()
        .danger_accept_invalid_certs(true)
        .no_gzip()
        .build();

    let response = match client {
        Ok(r) => r
            .post(api_url)
            .header(CONTENT_TYPE, "application/json")
            .json(&json_message)
            .send().await,
        Err(_e) => process::exit(3)
    };
    let result_text = match response {
        Ok(r) => r.text().await,
        Err(_e) => process::exit(3)
    };
    println!("{:?}", result_text)
}


// https://docs.rs/serde_json/latest/serde_json/
use serde_json::{json, Result, Value};



pub fn jsonify(username: &str, message: String) -> Result<Value> {
    let data = json!({
    "username": username,
    "content": message,
    });


    println!("{}", data.to_string());

    Ok(data)
}
