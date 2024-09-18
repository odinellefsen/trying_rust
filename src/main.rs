mod webhook; // Include the webhook.rs module

#[tokio::main] // This makes the main function asynchronous
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Sending webhook...");
    webhook::send_webhook().await?; // Await the result of the async function
    Ok(())
}
