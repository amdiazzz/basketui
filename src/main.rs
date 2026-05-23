use std::io;
use basketui::app::App;

#[tokio::main]
async fn main() -> io::Result<()> {
    let mut interval = 30;
    let mut start_league = String::from("NBA");
    let mut api_key_opt: Option<String> = None;
    
    let args: Vec<String> = std::env::args().collect();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--interval" | "-i" => {
                if i + 1 < args.len() {
                    if let Ok(val) = args[i + 1].parse::<u64>() {
                        interval = val;
                    }
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--league" | "-l" => {
                if i + 1 < args.len() {
                    start_league = args[i + 1].to_uppercase();
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--api-key" | "-k" => {
                if i + 1 < args.len() {
                    api_key_opt = Some(args[i + 1].clone());
                    i += 2;
                } else {
                    i += 1;
                }
            }
            "--help" | "-h" => {
                println!("basketui - Terminal-based NBA/NCAA/WNBA game viewer");
                println!();
                println!("Usage: basketui [OPTIONS]");
                println!();
                println!("Options:");
                println!("  -i, --interval <secs>  Set refresh interval in seconds (default: 30)");
                println!("  -l, --league <league>  Set initial league (NBA, NCAA, WNBA) (default: NBA)");
                println!("  -k, --api-key <key>    Set API key (or use BASKETUI_API_KEY environment variable)");
                println!("  -h, --help             Show this help message");
                return Ok(());
            }
            _ => i += 1,
        }
    }

    crossterm::execute!(io::stdout(), crossterm::event::EnableMouseCapture)?;
    crossterm::terminal::enable_raw_mode()?;
    
    let mut terminal = ratatui::init();
    let mut app = App::default();
    
    app.refresh_interval_secs = interval;
    app.league = start_league;
    if let Some(key) = api_key_opt {
        app.api_key = key;
    }

    let result = app.run(&mut terminal).await;

    ratatui::restore();
    crossterm::execute!(io::stdout(), crossterm::event::DisableMouseCapture)?;
    crossterm::terminal::disable_raw_mode()?;
    
    result
}
