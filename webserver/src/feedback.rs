use axum::response::IntoResponse;
use octocrab::Octocrab;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::env;
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct Feedback {
    text: String,
    image_url: Option<String>,
}

impl Feedback {
    pub async fn post_to_github(&self) -> Result<(), FeedbackError> {
        let token = env::var("GH_TOKEN").map_err(|_| FeedbackError::MissingGitHubToken)?;

        let octo = Octocrab::builder()
            .personal_token(token)
            .build()
            .map_err(FeedbackError::OctocrabInitError)?;

        let has_image = self.image_url.is_some();
        let body = format!(
            "{}\n\n{}",
            self.text,
            if has_image {
                format!("![Attached Image]({})", self.image_url.as_ref().unwrap())
            } else {
                "No image attached".to_string()
            }
        );

        let mut labels = Vec::new();
        labels.push("feedback".to_string());
        labels.push("user-submitted".to_string());
        labels.push(if has_image {
            "with-image".to_string()
        } else {
            "no-image".to_string()
        });

        octo.issues("zk-game-dao", "app")
            .create("User feedback")
            .body(&body)
            .labels(Some(labels))
            .send()
            .await
            .map_err(FeedbackError::GitHubError)?;

        Ok(())
    }
}

#[derive(Error, Debug)]
pub enum FeedbackError {
    #[error("Submission is empty (no text or image URL)")]
    EmptySubmission,

    #[error("GH_TOKEN environment variable not set")]
    MissingGitHubToken,

    #[error("Failed to initialize Octocrab client")]
    OctocrabInitError(octocrab::Error),

    #[error("Failed to post feedback to GitHub")]
    GitHubError(octocrab::Error),
}

impl IntoResponse for FeedbackError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match &self {
            FeedbackError::EmptySubmission => (
                StatusCode::BAD_REQUEST,
                "Submission is empty (no text or image URL)".to_string(),
            ),
            FeedbackError::MissingGitHubToken => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "GitHub token not configured".to_string(),
            ),
            FeedbackError::OctocrabInitError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to initialize GitHub client".to_string(),
            ),
            FeedbackError::GitHubError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to post feedback to GitHub".to_string(),
            ),
        };

        eprintln!("Error occurred: {:?}", &self); // Log the full error for debugging
        (status, error_message).into_response()
    }
}
