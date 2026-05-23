use std::io;
use std::process::Command;
use json::JsonValue;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use ratatui::layout::Rect;
use crate::api;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    #[default]
    Home,
    Game,
}

#[derive(Debug)]
pub struct App {
    pub screen: Screen,
    pub username: String,
    pub games: Vec<JsonValue>,
    pub selected_game_stats: Vec<JsonValue>,
    pub stats_loading: bool,
    pub home_team: String,
    pub away_team: String,
    pub home_score: u8,
    pub away_score: u8,
    pub exit: bool,
    
    // TUI Navigation State
    pub selected_game_index: usize,
    pub page_offset: usize,
    pub league: String,
    pub refresh_interval_secs: u64,
    pub api_key: String,
    pub error_msg: Option<String>,
    pub last_area: std::cell::Cell<Rect>,
    pub favorites: Vec<String>,
    pub filter_favorites: bool,
    
    pub last_fetch_time: std::time::Instant,
}

impl Default for App {
    fn default() -> Self {
        let api_key = std::env::var("BASKETUI_API_KEY").unwrap_or_default();
        let favorites = Self::load_favorites_config();
        
        Self {
            screen: Screen::Home,
            username: String::new(),
            games: Vec::new(),
            selected_game_stats: Vec::new(),
            stats_loading: false,
            home_team: String::new(),
            away_team: String::new(),
            home_score: 0,
            away_score: 0,
            exit: false,
            selected_game_index: 0,
            page_offset: 0,
            league: "NBA".to_string(),
            refresh_interval_secs: 30,
            api_key,
            error_msg: None,
            last_area: std::cell::Cell::new(Rect::default()),
            favorites,
            filter_favorites: false,
            last_fetch_time: std::time::Instant::now() - std::time::Duration::from_secs(3600),
        }
    }
}

impl App {
    pub async fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        self.get_username();
        self.fetch_games().await;
        
        while !self.exit {
            if self.last_fetch_time.elapsed() >= std::time::Duration::from_secs(self.refresh_interval_secs) {
                self.fetch_games().await;
            }

            terminal.draw(|frame| {
                frame.render_widget(&*self, frame.area());
            })?;
            
            if event::poll(std::time::Duration::from_millis(250))? {
                self.handle_events().await?;
            }
        }
        Ok(())
    }

    async fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event).await;
            },
            Event::Mouse(mouse_event) => {
                self.handle_mouse_event(mouse_event).await;
            }
            _ => {}
        };
        Ok(())
    }

    async fn handle_key_event(&mut self, key_event: KeyEvent) {
        match (&self.screen, key_event.code) {
            (_, KeyCode::Char('q')) => self.exit(),
            
            (Screen::Home, KeyCode::Enter) | (Screen::Home, KeyCode::Char(' ')) => {
                let game_info = {
                    let filtered = self.filtered_games();
                    if !filtered.is_empty() && self.selected_game_index < filtered.len() {
                        let game = filtered[self.selected_game_index];
                        Some((
                            game["home_team"]["full_name"].as_str().unwrap_or("").to_string(),
                            game["visitor_team"]["full_name"].as_str().unwrap_or("").to_string(),
                            game["home_team_score"].as_u64().unwrap_or(0) as u8,
                            game["visitor_team_score"].as_u64().unwrap_or(0) as u8,
                        ))
                    } else {
                        None
                    }
                };
                if let Some((home, visitor, h_score, a_score)) = game_info {
                    self.home_team = home;
                    self.away_team = visitor;
                    self.home_score = h_score;
                    self.away_score = a_score;
                    self.screen = Screen::Game;
                    self.selected_game_stats.clear();
                    self.fetch_game_stats().await;
                }
            }
            
            (Screen::Game, KeyCode::Backspace) | (Screen::Game, KeyCode::Esc) => {
                self.screen = Screen::Home;
            }
            
            (Screen::Home, KeyCode::Left) => {
                if self.selected_game_index > 0 {
                    self.selected_game_index -= 1;
                    self.adjust_page_offset();
                }
            }
            
            (Screen::Home, KeyCode::Right) => {
                let filtered = self.filtered_games();
                if !filtered.is_empty() && self.selected_game_index < filtered.len() - 1 {
                    self.selected_game_index += 1;
                    self.adjust_page_offset();
                }
            }
            
            (Screen::Home, KeyCode::Up) => {
                if self.selected_game_index >= 3 {
                    self.selected_game_index -= 3;
                    self.adjust_page_offset();
                }
            }
            
            (Screen::Home, KeyCode::Down) => {
                let filtered = self.filtered_games();
                if !filtered.is_empty() && self.selected_game_index + 3 < filtered.len() {
                    self.selected_game_index += 3;
                    self.adjust_page_offset();
                }
            }
            
            (Screen::Home, KeyCode::PageUp) => {
                let page_size = 6;
                if self.selected_game_index >= page_size {
                    self.selected_game_index -= page_size;
                } else {
                    self.selected_game_index = 0;
                }
                self.adjust_page_offset();
            }
            
            (Screen::Home, KeyCode::PageDown) => {
                let page_size = 6;
                let filtered = self.filtered_games();
                if !filtered.is_empty() {
                    if self.selected_game_index + page_size < filtered.len() {
                        self.selected_game_index += page_size;
                    } else {
                        self.selected_game_index = filtered.len() - 1;
                    }
                }
                self.adjust_page_offset();
            }
            
            (Screen::Home, KeyCode::Char('l')) | (Screen::Home, KeyCode::Tab) => {
                self.league = match self.league.as_str() {
                    "NBA" => "NCAA".to_string(),
                    "NCAA" => "WNBA".to_string(),
                    _ => "NBA".to_string(),
                };
                self.selected_game_index = 0;
                self.page_offset = 0;
                self.fetch_games().await;
            }
            
            (Screen::Home, KeyCode::Char('f')) => {
                let filtered = self.filtered_games();
                if !filtered.is_empty() && self.selected_game_index < filtered.len() {
                    let game = filtered[self.selected_game_index];
                    let home = game["home_team"]["abbreviation"].as_str().unwrap_or("").to_uppercase();
                    if self.favorites.contains(&home) {
                        self.favorites.retain(|x| x != &home);
                    } else {
                        self.favorites.push(home);
                    }
                    self.save_favorites_config();
                    
                    // Re-sort games to keep favorites on top
                    let favs = &self.favorites;
                    self.games.sort_by_key(|g| {
                        let h = g["home_team"]["abbreviation"].as_str().unwrap_or("").to_uppercase();
                        let v = g["visitor_team"]["abbreviation"].as_str().unwrap_or("").to_uppercase();
                        let is_fav = favs.contains(&h) || favs.contains(&v);
                        if is_fav { 0 } else { 1 }
                    });
                }
            }
            
            (Screen::Home, KeyCode::Char('F')) => {
                self.filter_favorites = !self.filter_favorites;
                self.selected_game_index = 0;
                self.page_offset = 0;
            }
            
            (Screen::Home, KeyCode::Char('r')) => {
                self.fetch_games().await;
            }
            
            _ => {}
        }
    }

    async fn handle_mouse_event(&mut self, mouse_event: event::MouseEvent) {
        let area = self.last_area.get();
        if area.width == 0 || area.height == 0 {
            return;
        }
        let grid_x = area.x + 1;
        let grid_y = area.y + 3;
        let grid_width = area.width - 2;
        let grid_height = area.height.saturating_sub(4);
        
        // Match columns mathematically
        let col_width = grid_width / 3;
        let c = if mouse_event.column >= grid_x && mouse_event.column < grid_x + grid_width {
            let relative_x = mouse_event.column - grid_x;
            (relative_x / col_width) as usize
        } else {
            return;
        };

        match mouse_event.kind {
            event::MouseEventKind::Down(event::MouseButton::Left) => {
                if matches!(self.screen, Screen::Home) {
                    if mouse_event.row >= grid_y {
                        let click_relative_y = mouse_event.row - grid_y;
                        let r = (click_relative_y / 5) as usize;
                        let clicked_index = (self.page_offset + r) * 3 + c;
                        let clicked_game_info = {
                            let filtered = self.filtered_games();
                            if clicked_index < filtered.len() {
                                let game = filtered[clicked_index];
                                Some((
                                    game["home_team"]["full_name"].as_str().unwrap_or("").to_string(),
                                    game["visitor_team"]["full_name"].as_str().unwrap_or("").to_string(),
                                    game["home_team_score"].as_u64().unwrap_or(0) as u8,
                                    game["visitor_team_score"].as_u64().unwrap_or(0) as u8,
                                ))
                            } else {
                                None
                            }
                        };
                        if let Some((home, visitor, h_score, a_score)) = clicked_game_info {
                            self.selected_game_index = clicked_index;
                            self.home_team = home;
                            self.away_team = visitor;
                            self.home_score = h_score;
                            self.away_score = a_score;
                            self.screen = Screen::Game;
                            self.selected_game_stats.clear();
                            self.fetch_game_stats().await;
                        }
                    }
                }
            }
            event::MouseEventKind::ScrollUp => {
                if matches!(self.screen, Screen::Home) {
                    if self.page_offset > 0 {
                        self.page_offset -= 1;
                    }
                }
            }
            event::MouseEventKind::ScrollDown => {
                if matches!(self.screen, Screen::Home) {
                    let len = self.filtered_games().len();
                    let max_rows = (len + 2) / 3;
                    let visible_rows = (grid_height / 5) as usize;
                    if self.page_offset + visible_rows < max_rows {
                        self.page_offset += 1;
                    }
                }
            }
            _ => {}
        }
    }

    pub fn filtered_games(&self) -> Vec<&JsonValue> {
        let mut list = Vec::new();
        for game in &self.games {
            let home = game["home_team"]["abbreviation"].as_str().unwrap_or("").to_uppercase();
            let visitor = game["visitor_team"]["abbreviation"].as_str().unwrap_or("").to_uppercase();
            
            if self.filter_favorites {
                if self.favorites.contains(&home) || self.favorites.contains(&visitor) {
                    list.push(game);
                }
            } else {
                list.push(game);
            }
        }
        list
    }

    pub async fn fetch_games(&mut self) {
        if self.api_key.is_empty() {
            self.error_msg = Some("API Key is missing! Set BASKETUI_API_KEY or use -k flag.".to_string());
            return;
        }
        
        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
        match api::get_games(&today, &self.league, &self.api_key).await {
            Ok(mut fetched_games) => {
                self.error_msg = None;
                
                // Sort favorites to top
                let favs = &self.favorites;
                fetched_games.sort_by_key(|game| {
                    let home = game["home_team"]["abbreviation"].as_str().unwrap_or("").to_uppercase();
                    let visitor = game["visitor_team"]["abbreviation"].as_str().unwrap_or("").to_uppercase();
                    let is_fav = favs.contains(&home) || favs.contains(&visitor);
                    if is_fav { 0 } else { 1 }
                });
                
                self.games = fetched_games;
                if self.selected_game_index >= self.filtered_games().len() {
                    self.selected_game_index = 0;
                }
                self.last_fetch_time = std::time::Instant::now();
            }
            Err(err) => {
                self.error_msg = Some(format!("API Error: {}", err));
            }
        }
    }

    pub async fn fetch_game_stats(&mut self) {
        if self.api_key.is_empty() || self.games.is_empty() {
            return;
        }
        let filtered = self.filtered_games();
        if self.selected_game_index >= filtered.len() {
            return;
        }
        let game = filtered[self.selected_game_index];
        let game_id = game["id"].as_i64().unwrap_or(0);
        if game_id == 0 {
            return;
        }

        self.stats_loading = true;
        match api::get_game_stats(game_id, &self.league, &self.api_key).await {
            Ok(stats) => {
                self.selected_game_stats = stats;
                self.error_msg = None;
            }
            Err(err) => {
                self.error_msg = Some(format!("Stats Error: {}", err));
            }
        }
        self.stats_loading = false;
    }

    fn adjust_page_offset(&mut self) {
        let grid_height = self.last_area.get().height.saturating_sub(4);
        let visible_rows = (grid_height / 5) as usize;
        let selected_row = self.selected_game_index / 3;
        
        if selected_row < self.page_offset {
            self.page_offset = selected_row;
        } else if selected_row >= self.page_offset + visible_rows {
            if visible_rows > 0 {
                self.page_offset = selected_row - visible_rows + 1;
            }
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn get_username(&mut self) {
        self.username = std::env::var("USERNAME")
            .or_else(|_| std::env::var("USER"))
            .unwrap_or_else(|_| {
                let output = if cfg!(target_os = "windows") {
                    Command::new("cmd").args(["/C", "whoami"]).output()
                } else {
                    Command::new("whoami").output()
                };
                match output {
                    Ok(out) => String::from_utf8_lossy(&out.stdout).trim().to_string(),
                    Err(_) => "User".to_string(),
                }
            });
    }

    fn get_config_path() -> Option<std::path::PathBuf> {
        let home = std::env::var("USERPROFILE")
            .or_else(|_| std::env::var("HOME"))
            .ok()?;
        let path = std::path::PathBuf::from(home)
            .join(".config")
            .join("basketui")
            .join("config.toml");
        Some(path)
    }

    fn load_favorites_config() -> Vec<String> {
        if let Some(path) = Self::get_config_path() {
            if path.exists() {
                if let Ok(content) = std::fs::read_to_string(path) {
                    let mut favorites = Vec::new();
                    for line in content.lines() {
                        if line.trim().starts_with("favorites") {
                            if let Some(start) = line.find('[') {
                                if let Some(end) = line.find(']') {
                                    let list_str = &line[start + 1..end];
                                    for item in list_str.split(',') {
                                        let clean_item = item.trim().replace('"', "").replace('\'', "");
                                        if !clean_item.is_empty() {
                                            favorites.push(clean_item.to_uppercase());
                                        }
                                    }
                                }
                            }
                        }
                    }
                    return favorites;
                }
            }
        }
        Vec::new()
    }

    fn save_favorites_config(&self) {
        if let Some(path) = Self::get_config_path() {
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
            }
            let list_str = self.favorites
                .iter()
                .map(|f| format!("\"{}\"", f))
                .collect::<Vec<String>>()
                .join(", ");
            let content = format!("favorites = [{}]\n", list_str);
            let _ = std::fs::write(path, content);
        }
    }
}
