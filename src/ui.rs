use chrono::Local;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};
use crate::{api::{Game}, app::{App, Screen}};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.screen {
            Screen::Home => self.render_home(area, buf),
            Screen::Game => self.render_game(area, buf),
        }
    }
}

impl App {
    // #4: Home Layout
    fn render_home(&self, area: Rect, buf: &mut Buffer) {
        // get date
        let now = Local::now();
        let date = now.format("%a %b %-d, %Y");

        let title = Line::from(" Basketui ").bold();
        let greeting = Line::from(format!(" NBA Scores  |  {date} "));
        let hint = Line::from(" Enter: select game  q: quit ".dark_gray());

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(hint.centered())
            .border_set(border::THICK);

        let inner = block.inner(area);
        block.render(area, buf);

        let separator = Line::from("─".repeat(inner.width as usize));
        
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([Constraint::Length(1), Constraint::Length(1), Constraint::Min(0)])
            .split(inner);

        // displays current day
        Paragraph::new(greeting)
            .alignment(Alignment::Center)
            .render(chunks[0], buf);

        Paragraph::new(separator)
            .render(chunks[1], buf);

        // Display game grid on home screen
        self.render_game_grid(chunks[2], buf);
    }

    fn render_game(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Basketui ").bold();
        let hint = Line::from(" Backspace: back  q: quit ".dark_gray());

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(hint.centered())
            .border_set(border::THICK);

        let inner = block.inner(area);
        block.render(area, buf);

        // Display single selected game details
        let home_name = if self.home_team.is_empty() { "---" } else { &self.home_team };
        let away_name = if self.away_team.is_empty() { "---" } else { &self.away_team };

        let content = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Length(2),
                Constraint::Length(2),
                Constraint::Length(2),
            ])
            .split(inner);

        Paragraph::new(format!("Home: {}", home_name)).render(content[0], buf);
        Paragraph::new(format!("Away: {}", away_name)).render(content[1], buf);
        Paragraph::new(format!("Score: {} - {}", self.home_score, self.away_score))
            .alignment(Alignment::Center)
            .bold()
            .render(content[2], buf);
    }

    fn render_game_grid(&self, area: Rect, buf: &mut Buffer) {

        // get games from api
        let games = &self.games;
        let len = games.len();

        let cols = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(33); len])
            .split(area);

        for row_start in (0..len).step_by(3) {
            if row_start == 0 {
                for col in 0..3 {
                    if row_start + col < len {
                        self.render_game_box(cols[col], buf, &games[row_start + col]);
                    }
                }
            }
            break;
        }
    }

    fn render_game_box(&self, area: Rect, buf: &mut Buffer, game: &Game) {
        // let box_title = format!("Game {}", game_num + 1);
        let home = format!("HOME: {}", game.home_team.abbreviation);
        let away = format!("HOME: {}", game.visitor_team.abbreviation);
        let score = format!("{} - {}", game.home_team_score, game.visitor_team_score);

        let block = Block::bordered()
            .title("Game")
            .border_set(border::ROUNDED);

        let inner = block.inner(area);
        block.render(area, buf);

        let content = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(1),
                Constraint::Length(1),
            ])
            .split(inner);

        Paragraph::new(home).render(content[0], buf);
        Paragraph::new(away).render(content[1], buf);
        Paragraph::new(score).alignment(Alignment::Center).bold().render(content[2], buf);
    }
}
