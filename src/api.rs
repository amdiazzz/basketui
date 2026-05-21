// https://api.balldontlie.io/v1/games?dates[]=<date>
// Authorization: Bearer <API_KEY>

use chrono::Local;

pub async fn get_games() -> Result<(), reqwest::Error> {
    let local_now = Local::now();
    let url = format!("https://api.balldontlie.io/v1/games?dates[]={}", local_now.format("%Y-%m-%d"));
    let _res = reqwest::get(url).await?;

    // #1: API Creation
    // TODO: Deserialize and return games
    // let games = res.json().await?;

    Ok(())
}