use anyhow::Result;
use iggy::client::{Client, UserClient};
use iggy::clients::client::IggyClient;

use iggy::clients::builder::IggyClientBuilder;
mod send_structured_message;
use send_structured_message::send_structured_message;

#[tokio::main]
async fn main() -> Result<()> {
    // Connect to the local Iggy server
    //let client = IggyClient::default();

    let client = IggyClientBuilder::new()
        .with_tcp()
        .with_server_address("abjad.mayorana.ch:8090".to_string())
        .build()?;

    client.connect().await?;
    client.login_user("iggy", "iggy").await?;

    send_structured_message(
        &client,
        "gibro",
        "notification",
        "send_email",
        vec![
            "user@example.com".to_string(),
            "Welcome!".to_string(),
            "Hello, welcome to momo service".to_string(),
        ],
    )
    .await?;

    println!("Message sent!");
    Ok(())
}
