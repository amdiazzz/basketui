use std::{io};
use std::process::Command;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::DefaultTerminal;
use crate::api::{self, Game};

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
    pub games: Vec<Game>, // this will be the results from our api call
    pub home_team: String,
    pub away_team: String,
    pub home_score: u8,
    pub away_score: u8,
    pub exit: bool,
}

impl App {
    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        // get result from tokio runtime on api::get_games()
        self.games = tokio::runtime::Runtime::new()
            .expect("tokio runtime")
            .block_on(api::get_games())
            .unwrap_or_default();

        self.get_username();
        while !self.exit {
            terminal.draw(|frame| {
                frame.render_widget(&*self, frame.area());
            })?;
            self.handle_events()?;
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
