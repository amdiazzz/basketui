use basketui::app::{App, Screen};
use json::object;

#[test]
fn test_app_default() {
    let app = App::default();
    assert_eq!(app.screen, Screen::Home);
    assert_eq!(app.league, "NBA");
    assert_eq!(app.refresh_interval_secs, 30);
}

#[test]
fn test_favorites_filtering() {
    let mut app = App::default();
    
    let game1 = object! {
        "home_team" => object! { "abbreviation" => "LAL", "full_name" => "Los Angeles Lakers" },
        "visitor_team" => object! { "abbreviation" => "BOS", "full_name" => "Boston Celtics" },
        "home_team_score" => 100,
        "visitor_team_score" => 98,
        "status" => "Final",
        "time" => "",
        "period" => 4,
        "id" => 123
    };
    let game2 = object! {
        "home_team" => object! { "abbreviation" => "GSW", "full_name" => "Golden State Warriors" },
        "visitor_team" => object! { "abbreviation" => "CHI", "full_name" => "Chicago Bulls" },
        "home_team_score" => 110,
        "visitor_team_score" => 105,
        "status" => "Final",
        "time" => "",
        "period" => 4,
        "id" => 456
    };
    
    app.games = vec![game1, game2];
    
    assert_eq!(app.filtered_games().len(), 2);
    
    app.favorites = vec!["LAL".to_string()];
    app.filter_favorites = true;
    
    let filtered = app.filtered_games();
    assert_eq!(filtered.len(), 1);
    assert_eq!(filtered[0]["home_team"]["abbreviation"], "LAL");
}
