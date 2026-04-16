use handler::Handler;
use models::{Config, DataFile};

use serenity::prelude::*;
use tokio::sync::RwLock;

use std::{
	fs::{self},
	path::Path,
	sync::Arc,
};

mod commands;
mod handler;
mod models;

const DB_FILE: &str = "data.json";

fn load_database(file_path: &str) -> DataFile {
	let path = Path::new(file_path);

	if !path.exists() {
		println!(
			"[WARN] File {} doesn't exist. Creating new one...",
			file_path
		);

		let default_db = DataFile {
			guild_id: 0,
			guild_name: String::from("No server name"),
			members: Vec::new(),
			channels: Vec::new(),
		};

		let default_json =
			serde_json::to_string_pretty(&default_db).expect("[ERROR] Can't serialize default db");

		fs::write(file_path, default_json).expect("[ERROR] Can't create databse file");

		return default_db;
	}

	let content =
		fs::read_to_string(file_path).expect("[ERROR] An error occured while reading the file");

	let database = serde_json::from_str::<DataFile>(&content)
		.expect("[ERROR] JSON file corrupted or not fitting DataFile type");

	database
}

fn write_database(database: &DataFile) {
	let database_json = serde_json::to_string(&database)
		.expect("[ERROR] An error occurred while parsing Datafile into json");
	fs::write(DB_FILE, database_json)
		.expect("[ERROR] An error occurred while writing into the json file");
}

#[tokio::main]
async fn main() {
	let config = Config::load();
	let intents = GatewayIntents::GUILDS
		| GatewayIntents::GUILD_MESSAGES
		| GatewayIntents::GUILD_VOICE_STATES;

	let guild_id = config.guild_id;
	let token = config.token;

	let database = Arc::new(RwLock::new(load_database(DB_FILE)));
	let poise_database = database.clone();
	let handler_database = database.clone();

	let framework = poise::Framework::builder()
		.options(poise::FrameworkOptions {
			commands: vec![
				commands::ping(),
				commands::time(),
				commands::server(),
				commands::leaderboard(),
			],
			..Default::default()
		})
		.setup(move |ctx, _ready, framework| {
			Box::pin(async move {
				let empty_commands: &[poise::Command<
					crate::models::DataFile,
					crate::models::Error,
				>] = &[];

				// Clear commands
				poise::builtins::register_globally(ctx, empty_commands).await?;
				poise::builtins::register_in_guild(
					ctx,
					empty_commands,
					poise::serenity_prelude::GuildId::new(guild_id),
				)
				.await?;

				// Register commands
				// poise::builtins::register_globally(ctx, &framework.options().commands).await?;
				poise::builtins::register_in_guild(
					ctx,
					&framework.options().commands,
					poise::serenity_prelude::GuildId::new(guild_id),
				)
				.await?;
				Ok(poise_database)
			})
		})
		.build();

	let mut client = Client::builder(token, intents)
		.event_handler(Handler {
			db: handler_database,
		})
		.framework(framework)
		.await
		.expect("Error while creating client");

	if let Err(why) = client.start().await {
		println!("Client error: {why:?}");
	}
}
