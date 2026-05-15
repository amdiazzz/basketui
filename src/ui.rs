use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Paragraph, Widget},
};
use crate::app::{App, Screen};

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self.screen {
            Screen::Home => self.render_home(area, buf),
            Screen::Game => self.render_game(area, buf),
        }
    }
}

impl App {
    fn render_home(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Basketui ").bold();
        let hint = Line::from(" Enter: select game  q: quit ".dark_gray());

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(hint.centered())
            .border_set(border::THICK);

        let inner = block.inner(area);
        block.render(area, buf);

        Paragraph::new("No games yet")
            .alignment(Alignment::Center)
            .render(inner, buf);
    }

    fn render_game(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Basketui ").bold();
        let hint = Line::from(" Backspace: back  q: quit ".dark_gray());

        let home_name = if self.home_team.is_empty() { "---" } else { &self.home_team };
        let away_name = if self.away_team.is_empty() { "---" } else { &self.away_team };

        let teams = Line::from(vec![
            " Home ".blue().bold(),
            " Away ".red().bold(),
        ]);

        let team_names = Line::from(vec![
            format!(" {} ", home_name).blue().into(),
            format!(" {} ", away_name).red().into(),
        ]);

        let team_scores = Line::from(vec![
            format!(" {} ", self.home_score).blue().into(),
            format!(" {} ", self.away_score).red().into(),
        ]);

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(hint.centered())
            .border_set(border::THICK);

        let inner = block.inner(area);
        block.render(area, buf);

        let [_, content_area, _] = Layout::vertical([
            Constraint::Fill(1),
            Constraint::Length(3),
            Constraint::Fill(1),
        ])
        .areas(inner);

        Paragraph::new(Text::from(vec![teams, team_names, team_scores]))
            .alignment(Alignment::Center)
            .render(content_area, buf);
    }
}
