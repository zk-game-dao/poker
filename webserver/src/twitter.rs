use twitter_v2::{authorization::Oauth1aToken, Error as TwitterError, TwitterApi};

#[derive(Debug)]
pub enum TwitterClientError {
    ApiError(TwitterError),
}

impl From<TwitterError> for TwitterClientError {
    fn from(error: TwitterError) -> Self {
        TwitterClientError::ApiError(error)
    }
}

#[derive(Debug)]
pub struct TwitterClient {
    api: TwitterApi<Oauth1aToken>,
}

impl TwitterClient {
    pub fn new(
        consumer_key: String,
        consumer_secret: String,
        access_token: String,
        access_token_secret: String,
    ) -> Self {
        let auth = Oauth1aToken::new(
            consumer_key,
            consumer_secret,
            access_token,
            access_token_secret,
        );
        let api = TwitterApi::new(auth);
        Self { api }
    }

    pub async fn send_tweet(&self, message: &str) -> Result<(), TwitterClientError> {
        let tweet = self
            .api
            .post_tweet()
            .text(message.to_string())
            .send()
            .await?;

        println!("Tweet sent successfully with id: {:?}", tweet.data);
        Ok(())
    }

    pub async fn send_table_update(
        &self,
        table_id: &str,
        old_count: usize,
        new_count: usize,
    ) -> Result<(), TwitterClientError> {
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

        self.send_tweet(&message).await
    }
}

// Helper function for the notification server
pub async fn send_tweet(
    twitter_client: &TwitterClient,
    table_id: &str,
    old_count: usize,
    new_count: usize,
) {
    if let Err(e) = twitter_client
        .send_table_update(table_id, old_count, new_count)
        .await
    {
        eprintln!("Failed to send Twitter update: {:?}", e);
    }
}
