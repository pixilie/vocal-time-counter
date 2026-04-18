# Vocal Time Counter

## 📊 Overview
**Vocal Time Counter** is a Discord bot written in **Rust** designed to accurately track the time members spend in voice channels.
It doesn't just count global time: it also tracks time spent while **muted**, **deafened**, **streaming**, or with a **camera** turned on.

## ✨ Features
- ⏱️ **Total time tracking** in voice channels.
- 🙊 **Specific state tracking**: Mute, Deafen, Streaming, and Video.
- 🏆 **Automatic leaderboards** for the most active members.
- 🌍 **Global server statistics** including top voice channels ranking.

## 🛠️ Available Commands
- `/time [user]`: Displays complete voice statistics for yourself or another member.
- `/leaderboard [limit]`: Displays the ranking of the most active members (default top 10).
- `/server [limit]`: Displays global server stats (total time, stream time, etc.) and the ranking of the most popular voice channels.
- `/ping`: Displays the bot's response time (latency).

## ⚙️ Configuration & Deployment (Recommended)

The simplest and most secure way to host this bot in production is using **Docker**.

### 1. Prerequisites
- [Docker](https://docs.docker.com/get-docker/) and [Docker Compose](https://docs.docker.com/compose/install/) installed on your server (e.g., Ubuntu).
- A Discord bot token (obtained from the [Discord Developer Portal](https://discord.com/developers/applications)).

### 2. Clone the Repository
```bash
git clone https://github.com/pixilie/vocal-time-counter/ # (or git@github.com:pixilie/vocal-time-counter.git)
cd vocal-time-counter
```

### 3. Environment Configuration

Edit the ``docker-compose.yml`` file with your information:

```yml
environment:
  - TOKEN=your_discord_token_here
  - GUILD_ID=your_discord_server_id
  - RUST_LOG=info
```

_Note: As of now, the bot registers its commands specifically to the server provided in GUILD_ID so they update instantly without delay._

### 4. Launch with Docker
The docker-compose.yml file is pre-configured to build the bot and create a persistent volume for your statistics (./data/data.json).

Run this command to build and start the bot in the background:
```bash
docker compose up -d --build
```
Important: Your data will survive restarts! It is saved persistently in the data/ folder automatically created at the root of your project.

### 5. Updating the Bot

If there is an update, you can deploy the new version without losing your statistics by simply running:
```bash
docker compose up -d --build
```

## 👨‍💻 Local Development (Without Docker)

To develop or test locally:
1. Ensure Rust is installed.
2. Copy the example file to create your configuration:
```bash
cp .env.exemple .env
```
3. Edit the ``.env`` file with your information:
```
TOKEN=your_discord_token_here
GUILD_ID=your_discord_server_id
```
4. Run the project with Cargo:
```bash
cargo run
```
_Note: As of now, the bot registers its commands specifically to the server provided in GUILD_ID so they update instantly without delay._

## Support & Contact
If you encounter bugs or have suggestions, feel free to contact me:
- Discord: ``@pixilie``
- Email: ``contact@pixilie.net``

## 📜 License
This project is licensed under the MIT License. See the ``LICENSE`` file for details.
