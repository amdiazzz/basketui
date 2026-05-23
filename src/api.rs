use json::JsonValue;

pub async fn get_games(date: &str, league: &str, api_key: &str) -> Result<Vec<JsonValue>, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = match league {
        "WNBA" => format!("https://api.balldontlie.io/wnba/v1/games?dates[]={}", date),
        "NCAA" => format!("https://api.balldontlie.io/ncaab/v1/games?dates[]={}", date),
        _ => format!("https://api.balldontlie.io/v1/games?dates[]={}", date),
    };

    let res = client.get(&url)
        .header("Authorization", api_key)
        .send()
        .await?;

    let text = res.text().await?;
    let parsed = json::parse(&text).unwrap_or(JsonValue::Null);
    let mut games = Vec::new();
    if let JsonValue::Array(arr) = &parsed["data"] {
        for item in arr {
            games.push(item.clone());
        }
    }
    Ok(games)
}

pub async fn get_game_stats(game_id: i64, league: &str, api_key: &str) -> Result<Vec<JsonValue>, reqwest::Error> {
    let client = reqwest::Client::new();
    let url = match league {
        "WNBA" => format!("https://api.balldontlie.io/wnba/v1/stats?game_ids[]={}", game_id),
        "NCAA" => format!("https://api.balldontlie.io/ncaab/v1/stats?game_ids[]={}", game_id),
        _ => format!("https://api.balldontlie.io/v1/stats?game_ids[]={}", game_id),
    };

    let res = client.get(&url)
        .header("Authorization", api_key)
        .send()
        .await?;

    let text = res.text().await?;
    let parsed = json::parse(&text).unwrap_or(JsonValue::Null);
    let mut stats = Vec::new();
    if let JsonValue::Array(arr) = &parsed["data"] {
        for item in arr {
            stats.push(item.clone());
        }
    }
    Ok(stats)
}