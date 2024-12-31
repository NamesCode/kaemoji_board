# Kaemoji board
A simple emoticon board, inspired by the macOS emoji picker! Intended for use with Hotkey daemons. 

## Usage 

### Prerequisites
- Rust
 
### Installation
```bash
git clone https://github.com/NamesCode/kaemoji_board.git
cd kaemoji_board
cargo install --path .
```
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

## Support

For support, you can ask in [my discord server here!](https://discord.gg/yvZF6cX9BD)
