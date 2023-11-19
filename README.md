# Kaemoji board
A simple emoticon board, inspired by the macOS emoji picker! Intended for use with Hotkey daemons. 
## Installation
### Prerequisites
- Rust


```bash
git clone https://github.com/NamesCode/kaemoji_board.git
cd kaemoji_board
cargo install --path .
```

## Usage 
Once installed, run `kaemoji_board`.

### Config
Configuration is simple with a single JSON file found at `~/.config/kaemoji_board/KaemojiConfig.json` for macOS and Linux or your operating systems default configuration path.

Example config:
```json
{
  "kaemojis": {
    "Example kaemoji": [
      {
        "emoticon": "( ͡° ͜ʖ ͡°)",
        "tags": [
          "example"
        ]
      }
    ]
  }
}
```
