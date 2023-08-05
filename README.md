# Twitch Bot Project

## Overview

This is a Twitch bot written in Rust that connects to the Twitch IRC server and allows users to interact with the bot via chat messages. The bot can join multiple Twitch channels and respond to commands.

## Features

- Connects to the Twitch IRC server using an OAuth token for authentication.
- Allows the bot to join multiple Twitch channels specified in the configuration file.
- Listens to chat messages from the channels and responds to specific commands.
- Sends messages to the chat in response to user input.
- Gracefully handles CTRL+C signal to terminate the bot.

## Getting Started

### Prerequisites

- Rust programming language is installed on your system.

### Installation

1. Clone the repository to your local machine.
```bash
git clone https://github.com/yourusername/twitch-bot.git
```

2. Change into the project directory.
```bash
cd twitch-bot
```

### Configuration

Before running the bot, make sure to create a `config.json` file in the project directory with the following content:

```json
{
  "token": "YOUR_TWITCH_OAUTH_TOKEN",
  "nickname": "YOUR_BOT_NICKNAME",
  "channels": ["channel1", "channel2", "channel3"]
}
```
Replace YOUR_TWITCH_OAUTH_TOKEN with your Twitch OAuth token, and YOUR_BOT_NICKNAME with your desired bot nickname. Add the Twitch channels you want the bot to join in the "channels" array.

### Usage

1. Build and run the bot.

```bash
cargo run
```

1. The bot will connect to the Twitch IRC server and join the specified channels.
2. It will start listening to chat messages and respond to commands from viewers.
3. To send a message to the chat, type your message in the console and press Enter.

### Tags
* v1.0.0: Initial release of the Twitch bot.
* beta: Beta version of the bot with additional features.

### Contribution
Contributions are welcome! If you find a bug or want to add new features, feel free to create a pull request.

## Special Thanks

This project was inspired by the Twitch channel of [Prof. Andrea Pollini](https://www.twitch.tv/profandreapollini) and the supportive Twitch community. Thanks to their encouragement and feedback!



## License

This project is licensed under the MIT License - see the [LICENSE](https://www.mit.edu/~amini/LICENSE.md) for details.