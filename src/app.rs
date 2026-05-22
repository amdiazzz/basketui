use std::{
    io,
    process::Command,
    time::{Duration, Instant},
};

use crate::api;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use json::*;
use ratatui::DefaultTerminal;

const API_RETRY_INTERVAL: Duration = Duration::from_secs(30);
const EVENT_POLL_INTERVAL: Duration = Duration::from_millis(250);

#[derive(Debug, Default)]
pub enum Screen {
    #[default]
    Home,
    Game,
}

#[derive(Debug, Default)]
pub struct App {
    pub screen: Screen,
    pub username: String,
    pub games: Vec<JsonValue>, // this will be the results from our api call
    pub api_error: Option<String>,
    last_api_fetch: Option<Instant>,
    pub home_team: String,
    pub away_team: String,
    pub home_score: u8,
    pub away_score: u8,
    pub exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .map_err(|error| io::Error::new(io::ErrorKind::Other, error))?;

        self.refresh_games(&runtime);
        self.get_username();
        while !self.exit {
            terminal.draw(|frame| {
                frame.render_widget(&*self, frame.area());
            })?;

            if event::poll(EVENT_POLL_INTERVAL)? {
                self.handle_events()?;
            } else if self.should_refresh_games() {
                self.refresh_games(&runtime);
            }
        }
        Ok(())
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            },

            _ => {}
        };
        Ok(())
    }

    fn should_refresh_games(&self) -> bool {
        self.last_api_fetch
            .map(|last_fetch| last_fetch.elapsed() >= API_RETRY_INTERVAL)
            .unwrap_or(true)
    }

    fn refresh_games(&mut self, runtime: &tokio::runtime::Runtime) {
        self.last_api_fetch = Some(Instant::now());
        match runtime.block_on(api::get_games()) {
            Ok(games) => {
                self.games = games;
                self.api_error = None;
            }
            Err(error) => {
                self.api_error = Some(format_api_error(&error));
            }
        }
    }

    // #3: Key Events
    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match (&self.screen, key_event.code) {
            (_, KeyCode::Char('q')) => self.exit(),
            (Screen::Home, KeyCode::Enter) => self.screen = Screen::Game,
            (Screen::Game, KeyCode::Backspace) => self.screen = Screen::Home,
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    fn get_username(&mut self) {
        let output = Command::new("sh")
            .arg("-c")
            .arg("whoami")
            .output()
            .expect("failed to execute process");
        self.username = String::from_utf8_lossy(&output.stdout).trim().to_string();
    }
}

fn format_api_error(error: &api::ApiError) -> String {
    if error.is_timeout() {
        return "Request timed out. Will retry automatically.".to_string();
    }

    if let Some(status) = error.status() {
        return format!("API returned {status}. Will retry automatically.");
    }

    format!("Unable to load games: {error}. Will retry automatically.")
}
