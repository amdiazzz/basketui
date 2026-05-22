// https://api.balldontlie.io/v1/games?dates[]=<date>
// Authorization: Bearer <API_KEY>

use chrono::Local;
use json::JsonValue;
use std::{fmt, time::Duration};

const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug)]
pub enum ApiError {
    Request(reqwest::Error),
    InvalidJson(json::Error),
}

impl ApiError {
    pub fn is_timeout(&self) -> bool {
        matches!(self, ApiError::Request(error) if error.is_timeout())
    }

    pub fn status(&self) -> Option<reqwest::StatusCode> {
        match self {
            ApiError::Request(error) => error.status(),
            ApiError::InvalidJson(_) => None,
        }
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApiError::Request(error) => write!(formatter, "{error}"),
            ApiError::InvalidJson(error) => write!(formatter, "invalid JSON response: {error}"),
        }
    }
}

impl std::error::Error for ApiError {}

impl From<reqwest::Error> for ApiError {
    fn from(error: reqwest::Error) -> Self {
        ApiError::Request(error)
    }
}

pub async fn get_games() -> Result<Vec<JsonValue>, ApiError> {
    let local_now = Local::now();
    let url = format!(
        "https://api.balldontlie.io/v1/games?dates[]={}",
        local_now.format("%Y-%m-%d")
    );
    let body = reqwest::Client::builder()
        .timeout(REQUEST_TIMEOUT)
        .build()?
        .get(url)
        .send()
        .await?
        .error_for_status()?
        .text()
        .await?;

    // #1: API Creation
    // TODO: Deserialize and return games
    let parsed_body = json::parse(&body).map_err(ApiError::InvalidJson)?;
    let games = parsed_body["data"].members().cloned().collect();

    Ok(games)
}
