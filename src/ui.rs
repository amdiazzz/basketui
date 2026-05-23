use chrono::Local;
use json::JsonValue;
use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Layout, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Span},
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
        self.last_area.set(area);
        
        let now = Local::now();
        let date = now.format("%a %b %-d, %Y");

        let filter_status = if self.filter_favorites { " (Favorites Only)" } else { "" };
        let title_text = format!(" Basketui - {} Scores{} ", self.league, filter_status);
        let title = Line::from(title_text).bold().cyan();
        
        let hint_text = " Tab/l: cycle league | f: fav team | F: filter favs | Enter: view game | q: quit ";
        let hint = Line::from(hint_text).dark_gray();

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(hint.centered())
            .border_set(border::THICK)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray));

        let inner = block.inner(area);
        block.render(area, buf);

        let separator = Line::from("─".repeat(inner.width as usize)).dark_gray();
        
        let chunks = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Length(1), 
                Constraint::Length(1), 
                Constraint::Min(0),
                Constraint::Length(1)
            ])
            .split(inner);

        let greeting = Line::from(vec![
            "Active League: ".into(),
            self.league.as_str().bold().yellow(),
            "  |  ".into(),
            date.to_string().into(),
            "  |  User: ".into(),
            self.username.as_str().green()
        ]);
        Paragraph::new(greeting)
            .alignment(Alignment::Center)
            .render(chunks[0], buf);

        Paragraph::new(separator)
            .render(chunks[1], buf);

        let filtered = self.filtered_games();
        self.render_game_grid(chunks[2], buf, &filtered);

        if let Some(err) = &self.error_msg {
            let err_line = Line::from(format!(" ERROR: {} ", err)).bold().red();
            Paragraph::new(err_line).alignment(Alignment::Center).render(chunks[3], buf);
        } else {
            let status_text = format!(" Interval: {}s | Refreshing in {}s | Total Games: {}", 
                self.refresh_interval_secs,
                self.refresh_interval_secs.saturating_sub(self.last_fetch_time.elapsed().as_secs()),
                filtered.len()
            );
            Paragraph::new(Line::from(status_text).dark_gray()).alignment(Alignment::Center).render(chunks[3], buf);
        }
    }

    fn render_game(&self, area: Rect, buf: &mut Buffer) {
        let title = Line::from(" Basketui - Game Details ").bold().cyan();
        let hint = Line::from(" Backspace/Esc: back to list | q: quit ").dark_gray();

        let block = Block::bordered()
            .title(title.centered())
            .title_bottom(hint.centered())
            .border_set(border::THICK)
            .border_style(ratatui::style::Style::default().fg(ratatui::style::Color::DarkGray));

        let inner = block.inner(area);
        block.render(area, buf);

        let columns = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints([
                Constraint::Percentage(35),
                Constraint::Percentage(65),
            ])
            .split(inner);

        let overview_layout = Layout::default()
            .direction(ratatui::layout::Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Length(3),
                Constraint::Min(0)
            ])
            .split(columns[0]);

        Paragraph::new(Line::from("GAME SUMMARY").bold().underlined().yellow())
            .alignment(Alignment::Center)
            .render(overview_layout[0], buf);

        let home_block = Block::bordered().title("Home").border_set(border::ROUNDED);
        let home_inner = home_block.inner(overview_layout[1]);
        home_block.render(overview_layout[1], buf);
        Paragraph::new(Line::from(vec![
            self.home_team.as_str().bold(),
            " - ".into(),
            self.home_score.to_string().green().bold()
        ])).alignment(Alignment::Center).render(home_inner, buf);

        let away_block = Block::bordered().title("Away").border_set(border::ROUNDED);
        let away_inner = away_block.inner(overview_layout[2]);
        away_block.render(overview_layout[2], buf);
        Paragraph::new(Line::from(vec![
            self.away_team.as_str().bold(),
            " - ".into(),
            self.away_score.to_string().green().bold()
        ])).alignment(Alignment::Center).render(away_inner, buf);

        let status_block = Block::bordered().title("Status").border_set(border::ROUNDED);
        let status_inner = status_block.inner(overview_layout[3]);
        status_block.render(overview_layout[3], buf);
        
        let filtered = self.filtered_games();
        let status_text = if self.selected_game_index < filtered.len() {
            filtered[self.selected_game_index]["status"].as_str().unwrap_or("Scheduled").to_string()
        } else {
            "Unknown".to_string()
        };
        Paragraph::new(status_text).alignment(Alignment::Center).render(status_inner, buf);

        let stats_block = Block::bordered().title("Box Score").border_set(border::ROUNDED);
        let stats_inner = stats_block.inner(columns[1]);
        stats_block.render(columns[1], buf);

        if self.stats_loading {
            Paragraph::new("Loading box score stats...")
                .alignment(Alignment::Center)
                .render(stats_inner, buf);
        } else if self.selected_game_stats.is_empty() {
            Paragraph::new("No player statistics available for this game.")
                .alignment(Alignment::Center)
                .render(stats_inner, buf);
        } else {
            let header = format!("{:<22} | {:<4} | {:>3} | {:>3} | {:>3}", "Player", "Team", "PTS", "AST", "REB");
            let separator = "─".repeat(stats_inner.width as usize);
            
            let stats_layout = Layout::default()
                .direction(ratatui::layout::Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Length(1),
                    Constraint::Min(0)
                ])
                .split(stats_inner);

            Paragraph::new(Line::from(header).bold().cyan()).render(stats_layout[0], buf);
            Paragraph::new(separator).render(stats_layout[1], buf);

            let mut stat_lines = Vec::new();
            for stat in &self.selected_game_stats {
                let first = stat["player"]["first_name"].as_str().unwrap_or("");
                let last = stat["player"]["last_name"].as_str().unwrap_or("");
                let name = format!("{} {}", first, last);
                
                let team = stat["team"]["abbreviation"].as_str().unwrap_or("");
                let pts = stat["pts"].as_i64().unwrap_or(0);
                let ast = stat["ast"].as_i64().unwrap_or(0);
                let reb = stat["reb"].as_i64().unwrap_or(0);
                
                let display_name = if name.len() > 22 {
                    format!("{}...", &name[..19])
                } else {
                    format!("{:<22}", name)
                };

                let line = format!("{} | {:<4} | {:>3} | {:>3} | {:>3}", display_name, team, pts, ast, reb);
                stat_lines.push(Line::from(line));
            }
            
            Paragraph::new(stat_lines).render(stats_layout[2], buf);
        }
    }

    fn render_game_grid(&self, area: Rect, buf: &mut Buffer, filtered_games: &[&JsonValue]) {
        let grid_height = area.height;
        let visible_rows = (grid_height / 5) as usize;
        
        let cols = Layout::default()
            .direction(ratatui::layout::Direction::Horizontal)
            .constraints(vec![
                Constraint::Percentage(33),
                Constraint::Percentage(33),
                Constraint::Percentage(33),
            ])
            .split(area);

        let start_row = self.page_offset;
        let end_row = start_row + visible_rows;

        for r in start_row..end_row {
            let row_idx = r - start_row;
            let row_y = area.y + (row_idx * 5) as u16;
            
            if row_idx * 5 + 5 > grid_height as usize {
                break;
            }

            for col in 0..3 {
                let game_idx = r * 3 + col;
                if game_idx < filtered_games.len() {
                    let game_area = Rect::new(
                        cols[col].x,
                        row_y,
                        cols[col].width,
                        5
                    );
                    self.render_game_box(game_area, buf, game_idx, filtered_games[game_idx]);
                }
            }
        }
    }

    fn render_game_box(&self, area: Rect, buf: &mut Buffer, game_idx: usize, game: &JsonValue) {
        let is_selected = game_idx == self.selected_game_index;
        
        let home_abbr = game["home_team"]["abbreviation"].as_str().unwrap_or("").to_uppercase();
        let visitor_abbr = game["visitor_team"]["abbreviation"].as_str().unwrap_or("").to_uppercase();
        
        let is_home_fav = self.favorites.contains(&home_abbr);
        let is_visitor_fav = self.favorites.contains(&visitor_abbr);
        
        let fav_star = if is_home_fav || is_visitor_fav { "★ " } else { "" };
        let box_title = format!("{}Game {}", fav_star, game_idx + 1);
        
        let home_name = game["home_team"]["full_name"].as_str().unwrap_or("---");
        let visitor_name = game["visitor_team"]["full_name"].as_str().unwrap_or("---");
        
        let home_score = game["home_team_score"].as_i64().map(|s| s.to_string()).unwrap_or_else(|| "-".to_string());
        let visitor_score = game["visitor_team_score"].as_i64().map(|s| s.to_string()).unwrap_or_else(|| "-".to_string());
        
        let status = game["status"].as_str().unwrap_or("Scheduled");
        let time = game["time"].as_str().unwrap_or("");
        let period = game["period"].as_i64().unwrap_or(0);
        
        let status_display = if status.to_lowercase().contains("qtr") || period > 0 {
            format!("{} - {}", status, time)
        } else {
            status.to_string()
        };

        let border_color = if is_selected {
            ratatui::style::Color::Cyan
        } else if is_home_fav || is_visitor_fav {
            ratatui::style::Color::Yellow
        } else {
            ratatui::style::Color::DarkGray
        };

        let block = Block::bordered()
            .title(Line::from(box_title).bold().fg(if is_selected { ratatui::style::Color::Cyan } else { ratatui::style::Color::White }))
            .border_set(border::ROUNDED)
            .border_style(ratatui::style::Style::default().fg(border_color));

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

        let home_style = if is_home_fav { ratatui::style::Style::default().yellow().bold() } else { ratatui::style::Style::default() };
        Paragraph::new(Line::from(vec![
            " HOME: ".into(),
            Span::styled(home_name, home_style)
        ])).render(content[0], buf);

        let visitor_style = if is_visitor_fav { ratatui::style::Style::default().yellow().bold() } else { ratatui::style::Style::default() };
        Paragraph::new(Line::from(vec![
            " AWAY: ".into(),
            Span::styled(visitor_name, visitor_style)
        ])).render(content[1], buf);

        let score_text = format!("{} : {}  ({})", home_score, visitor_score, status_display);
        Paragraph::new(Line::from(score_text).bold().green())
            .alignment(Alignment::Center)
            .render(content[2], buf);
    }
}
