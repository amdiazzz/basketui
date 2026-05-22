# basketui

`basketui` is a terminal user interface (TUI) for checking NBA scores from your shell. It uses [`ratatui`](https://ratatui.rs/) for rendering and the balldontlie API as the planned data source for game information.

## Features

- Full-screen terminal UI for today's NBA games.
- Home screen with a grid-style game overview.
- Game detail screen for the currently selected matchup.
- Keyboard navigation designed for quick terminal use.

> Note: the live game-fetching integration is still under active development. The current UI renders placeholder game/team data while the API response parsing is completed.

## Prerequisites

- Rust toolchain with Cargo installed. The easiest setup path is [`rustup`](https://rustup.rs/).
- A terminal that supports alternate-screen TUI applications.
- Internet access for live score requests once API parsing is enabled.

## Build from source

Clone the repository and build the release binary:

```bash
git clone https://github.com/amdiazzz/basketui.git
cd basketui
cargo build --release
```

The compiled binary will be available at:

```bash
./target/release/basketui
```

## Run

For local development, run directly with Cargo:

```bash
cargo run
```

Or run the release build:

```bash
./target/release/basketui
```

## Keybindings

| Key | Action |
| --- | --- |
| `Enter` | Open the game detail screen from the home screen |
| `Backspace` | Return from the game detail screen to the home screen |
| `q` | Quit the application |

## Configuration

There are currently no CLI flags or config files. The app uses the current local date to build the planned balldontlie games API request.

Future configuration options may include refresh interval, league selection, API credentials, and preferred teams.

## Development

Useful commands while contributing:

```bash
cargo fmt
cargo check
cargo test
```

If you add new UI or API behavior, please include focused tests where practical and update this README when user-facing commands or keybindings change.
