use reqwest::{Client, Error as ReqwestError};
use serde::Serialize;
use std::time::Duration;

#[derive(Debug)]
pub enum DiscordError {
    RequestFailed(ReqwestError),
    ApiError(String),
}

impl From<ReqwestError> for DiscordError {
    fn from(error: ReqwestError) -> Self {
        DiscordError::RequestFailed(error)
    }
}

#[derive(Debug, Serialize)]
pub struct DiscordWebhookMessage {
    content: Option<String>,
    embeds: Vec<DiscordEmbed>,
}

#[derive(Debug, Serialize)]
struct DiscordEmbed {
    title: String,
    description: String,
    url: Option<String>,
    color: u32, // Discord color integer
}

#[derive(Debug)]
pub struct DiscordClient {
    client: Client,
    webhook_url: String,
}

impl DiscordClient {
    pub fn new(webhook_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            webhook_url,
        }
    }

    pub async fn send_message(&self, message: DiscordWebhookMessage) -> Result<(), DiscordError> {
        let response = self
            .client
            .post(&self.webhook_url)
            .json(&message)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(DiscordError::ApiError(format!(
                "Discord API error: {}",
                response.status()
            )));
        }

        Ok(())
    }

    pub async fn send_table_update(
        &self,
        table_id: &str,
        old_count: usize,
        new_count: usize,
    ) -> Result<(), DiscordError> {
        let change_symbol = if new_count > old_count {
            "📈"
        } else {
            "📉"
        };
        let table_url = format!("https://zkpoker.app/tables/{}", table_id);

        let description = format!(
            "{} Player count changed: {} → {}\n\n{}",
            change_symbol,
            old_count,
            new_count,
            if new_count > old_count {
                "More players have joined! Come join the action!"
            } else {
                "Seats are opening up! Perfect time to join!"
            }
        );

        let embed = DiscordEmbed {
            title: "🎮 ZK Poker Table Update".to_string(),
            description,
            url: Some(table_url),
            color: 3447003, // Discord blue
        };

        let message = DiscordWebhookMessage {
            content: None,
            embeds: vec![embed],
        };

        self.send_message(message).await
    }
}

pub async fn send_discord_message(
    discord_client: &DiscordClient,
    table_id: &str,
    old_count: usize,
    new_count: usize,
) {
    match discord_client
        .send_table_update(table_id, old_count, new_count)
        .await
    {
        Ok(_) => println!("Successfully sent Discord update for table {}", table_id),
        Err(e) => eprintln!("Failed to send Discord update: {:?}", e),
    }
}
