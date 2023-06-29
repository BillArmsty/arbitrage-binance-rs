use std::collections::HashMap;

//Alerts sent to discord via webhook

pub async fn alert(msg: &str) {
    let max_length = (2000).min(msg.len());
    let message = &msg[..max_length].to_string();
    let mut map = HashMap::new();
    map.insert("content", message.to_string());

    let webhook_url = std::env::var("DISCORD_WEBHOOK_URL").expect("DISCORD_WEBHOOK_URL not set");
    let client = reqwest::Client::new();
    let res = client.post(&webhook_url).json(&map).send().await;

    match res {
        Ok(_) => println!("Alert sent to discord"),
        Err(e) => println!("Error sending alert to discord: {}", e),
    }
}
