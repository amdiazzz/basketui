# Basketui

A small terminal UI for browsing NBA games from the `balldontlie` API.

## Requirements

- Rust toolchain
- A `TOKEN` env var for the API bearer token

## Build

```bash
cargo build --release
```

## Run

```bash
TOKEN=your_api_token cargo run
```

## Controls

- `Enter` - open the selected game
- `Backspace` - return to the home screen
- `q` - quit

## Notes

- The app currently shows today&apos;s games.
- The API key is read from `TOKEN` at runtime.
