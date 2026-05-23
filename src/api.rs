// https://api.balldontlie.io/v1/games?dates[]=<date>
// Authorization: Bearer <API_KEY>

use std::{env};
use reqwest::Client;
use chrono::{DateTime, Local};
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Deserialize)]
pub struct Team {
    pub id: i64,
    pub abbreviation: String,
    pub city: String,
    pub conference: String,
    pub division: String,
    pub full_name: String,
    pub name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Game {
    pub id: i64,
    // pub date: DateTime<chrono::Utc>,
    pub season: i32,
    pub period: i32,
    pub status: String,
    pub time: Option<String>,
    pub postseason: bool,
    pub home_team: Team,
    pub visitor_team: Team,
    pub home_team_score: i32,
    pub visitor_team_score: i32,
}

pub async fn get_games() -> Result<Vec<Game>, reqwest::Error> {
    let local_now = Local::now();
    let url = format!("https://api.balldontlie.io/v1/games?dates[]={}", local_now.format("%Y-%m-%d"));
    let client = Client::new();
    let token = std::env::var("TOKEN").expect("TOKEN env var not set");

    let res = client
                .get(url)
                .bearer_auth(token)
                .send()
                .await?;

    #[derive(Debug, Deserialize)]
    struct GamesResponse {
        data: Vec<Game>,
    }

    let games_resp = res.json::<GamesResponse>().await?;

    Ok(games_resp.data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_get_games() {
        env::set_var("TOKEN", "insert_token_here");
        let games = get_games().await.expect("get_games failed");
        println!("Got {} games", games.len());
        println!("Games:");
        for game in &games {
            println!("  {:#?}", game);
        }
    }
}