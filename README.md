# basketui

`basketui` is a fast, lightweight, and modern terminal-based game scores and statistics viewer for basketball fans. It retrieves live and historical scores and detailed player box scores for **NBA**, **NCAA**, and **WNBA** games directly in your terminal, built with **Ratatui** and **Tokio**.

## Features

- **Multi-League Support**: Cycle between NBA, NCAA, and WNBA scores.
- **Interactive TUI**: Navigate games using a responsive grid layout.
- **Keyboard & Mouse Events**: Support full navigation using keyboard arrow keys, Page Up/Page Down, and mouse clicking or scrolling.
- **Detailed Game Views**: Drill down into any active or finished game to view detailed team stats and player box scores (PTS, AST, REB).
- **Pin Favorites**: Persist and highlight your favorite teams so they always show up at the top of the grid.
- **Filtering**: Toggle visibility to only show games involving your favorite teams.
- **Configurable Auto-Refresh**: Specify how often scores are updated in seconds.
- **Error Handling**: Graceful error alert bar for network/rate limit issues without app crashes.

## Prerequisites

- **Rust toolchain** (Rust 1.70+ recommended)
- **API Key**: Register at [app.balldontlie.io](https://app.balldontlie.io/) to get a free personal API key.

## Installation

### Build from source

1. Clone this repository:
   ```bash
   git clone https://github.com/amdiazzz/basketui.git
   cd basketui
   ```

2. Build the project in release mode:
   ```bash
   cargo build --release
   ```

3. The compiled binary will be located at `target/release/basketui`. You can install it locally:
   ```bash
   cargo install --path .
   ```

## Configuration & Usage

Set your API key using the `BASKETUI_API_KEY` environment variable:

```bash
# On Unix-like systems (Linux / macOS)
export BASKETUI_API_KEY="your_api_key_here"

# On Windows (PowerShell)
$env:BASKETUI_API_KEY="your_api_key_here"
```

Then run `basketui`:

```bash
basketui [OPTIONS]
```

### Options

- `-i, --interval <secs>`: Set auto-refresh interval in seconds (default: `30` seconds).
- `-l, --league <league>`: Set initial league (`NBA`, `NCAA`, `WNBA`) (default: `NBA`).
- `-k, --api-key <key>`: Set API key directly via command line.
- `-h, --help`: Show help and usage instructions.

### Keybindings

| Key | Action |
| --- | --- |
| `q` | Exit the application |
| `Arrow Keys` / Mouse Click | Navigate the game grid / Select a game |
| `Enter` / `Space` | Drill into selected game details (Box Score) |
| `Backspace` / `Esc` | Back to home scores grid |
| `Tab` / `l` | Cycle active league (NBA -> NCAA -> WNBA -> NBA) |
| `f` | Toggle favorite status of the home team in selected game |
| `F` | Toggle favorites filter (only show games involving favorite teams) |
| `r` | Trigger manual scores refresh |
| `Page Up` / `Page Down` / Mouse Scroll | Scroll/page through game lists |

### Local Configuration File

Your pinned favorite teams are persisted locally in:
- Unix: `~/.config/basketui/config.toml`
- Windows: `%USERPROFILE%\.config\basketui\config.toml`

License: MIT
