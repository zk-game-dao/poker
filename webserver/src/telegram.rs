use reqwest::{Client, Error as ReqwestError};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const TELEGRAM_API_BASE: &str = "https://api.telegram.org";

#[derive(Debug)]
pub enum TelegramError {
    RequestFailed(ReqwestError),
    ApiError(String),
}

impl From<ReqwestError> for TelegramError {
    fn from(error: ReqwestError) -> Self {
        TelegramError::RequestFailed(error)
    }
}

#[derive(Debug, Serialize)]
struct SendMessageRequest {
    chat_id: String,
    text: String,
    parse_mode: Option<String>,
    disable_web_page_preview: Option<bool>,
}

#[derive(Debug, Deserialize)]
struct TelegramResponse {
    ok: bool,
    #[serde(default)]
    description: Option<String>,
}

#[derive(Debug)]
pub struct TelegramClient {
    client: Client,
    bot_token: String,
}

impl TelegramClient {
    pub fn new(bot_token: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .expect("Failed to create HTTP client");

        Self { client, bot_token }
    }

    pub async fn send_message(
        &self,
        channel_id: &str,
        message: &str,
        use_markdown: bool,
    ) -> Result<(), TelegramError> {
        let url = format!("{}/bot{}/sendMessage", TELEGRAM_API_BASE, self.bot_token);

        let request = SendMessageRequest {
            chat_id: channel_id.to_string(),
            text: message.to_string(),
            parse_mode: if use_markdown {
                Some("MarkdownV2".to_string())
            } else {
                None
            },
            disable_web_page_preview: Some(true),
        };

        let response = self
            .client
            .post(&url)
            .json(&request)
            .send()
            .await?
            .json::<TelegramResponse>()
            .await?;

        if !response.ok {
            return Err(TelegramError::ApiError(
                response
                    .description
                    .unwrap_or_else(|| "Unknown API error".to_string()),
            ));
        }

        Ok(())
    }

    pub async fn send_formatted_game_update(
        &self,
        channel_id: &str,
        table_id: &str,
        old_count: usize,
        new_count: usize,
    ) -> Result<(), TelegramError> {
        let table_url = format!("https://zkpoker.app/tables/{}", table_id);

        let change_symbol = if new_count > old_count {
            "📈"
        } else {
            "📉"
        };
        let message = format!(
            "🎮 *Poker Table Update* {}\n\n\
            [Join Table]({})\n\
            Players: {} → {}\n\n\
            {}",
            change_symbol,
            table_url,
            old_count,
            new_count,
            if new_count > old_count {
                "More players have joined\\! Come join the action\\!"
            } else {
                "Seats are opening up\\! Perfect time to join\\!"
            }
        );

        self.send_message(channel_id, &message, true).await
    }
}

pub async fn send_telegram_message(
    telegram_client: &TelegramClient,
    channel_id: &str,
    table_id: &str,
    old_count: usize,
    new_count: usize,
) {
    match telegram_client
        .send_formatted_game_update(channel_id, table_id, old_count, new_count)
        .await
    {
        Ok(_) => println!("Successfully sent Telegram update for table {}", table_id),
        Err(e) => eprintln!("Failed to send Telegram update: {:?}", e),
    }
}
