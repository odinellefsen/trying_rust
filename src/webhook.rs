use reqwest::Client;
use serde_json::json;

pub async fn send_webhook() -> Result<(), Box<dyn std::error::Error>> {
    // Your Flowcore webhook ingestion URL
    let webhook_url = "https://webhook.api.flowcore.io/event/odinellefsen/04938084-b536-4bae-ae19-b64f7eaa5444/random/random-1?key=6ff4a300-0b18-4193-b805-e372cacecf4e";

    // Create a JSON payload
    let payload = json!({
        "message": "Webhook test from Rust",
        "status": "success",
        "data": {
            "id": 123,
            "info": "More details"
        }
    });

    // Create an HTTP client
    let client = Client::new();

    // Send the webhook
    let response = client
        .post(webhook_url)  // HTTP POST request
        .json(&payload)     // Send the JSON payload
        .send()
        .await?;

    // Check if the webhook was successful
    if response.status().is_success() {
        println!("Webhook sent successfully!");
    } else {
        println!("Failed to send webhook. Status: {}", response.status());
    }

    Ok(())
}
