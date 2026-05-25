mod api;
mod app;
mod ui;

use std::io;
use app::App;

fn main() -> io::Result<()> {
    dotenvy::dotenv().ok();
    ratatui::run(|terminal| App::default().run(terminal));
    Ok(())
}
