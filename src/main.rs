use models::{Config, DataFile};

use serenity::all::VoiceState;
use serenity::async_trait;
use serenity::model::gateway::Ready;
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

struct Handler {
	db: Arc<RwLock<DataFile>>,
}

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, _: Context, ready: Ready) {
		println!("[INFO] {} is connected!", ready.user.name);
	}

	async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
		let user_id = new.user_id;
		let username = new.member.unwrap().user.name;

		// Join, Left & Move
		let old_channel = old.as_ref().and_then(|state| state.channel_id);
		let new_channel = new.channel_id;

		match (old_channel, new_channel) {
			(None, Some(id)) => {
				println!("[INFO] {}[{}] joined channel {}", username, user_id, id);
				self.user_joined(user_id, &username).await;
			}
			(Some(id), None) => {
				println!("[INFO] {}[{}] left channel {}", username, user_id, id);
				self.user_left(&ctx, old.as_ref().unwrap(), user_id, id)
					.await;
			}
			(Some(old_id), Some(new_id)) => {
				if old_id != new_id {
					println!(
						"[INFO] {}[{}] moved from channel {} to {}",
						username, user_id, old_id, new_id
					);
					self.user_moved(&ctx, user_id, old_id).await;
				}
			}
			_ => {}
		}

		// Stream & Stop stream
		let old_streaming = old
			.as_ref()
			.and_then(|state| state.self_stream)
			.unwrap_or(false);
		let new_streaming = new.self_stream.unwrap_or(false);

		match (old_streaming, new_streaming) {
			(false, true) => {
				println!("[INFO] {}[{}] started streaming", username, user_id);
				self.user_started_streaming(user_id).await;
			}
			(true, false) => {
				println!("[INFO] {}[{}] stopped streaming", username, user_id);
				self.user_stopped_streaming(user_id).await;
			}
			_ => {}
		}

		// Mute & Unmute
		let old_mute = old.as_ref().map(|state| state.self_mute).unwrap_or(false)
			|| old.as_ref().map(|state| state.mute).unwrap_or(false);
		let new_mute = new.self_mute || new.mute;

		match (old_mute, new_mute) {
			(false, true) => {
				println!(
					"[INFO] {}[{}] muted himself or has been muted",
					username, user_id
				);
				self.user_muted(user_id).await;
			}
			(true, false) => {
				println!(
					"[INFO] {}[{}] unmuted himself or has been unmuted",
					username, user_id
				);
				self.user_unmuted(user_id).await;
			}
			_ => {}
		}

		// Deaf & Undeaf
		let old_deaf = old.as_ref().map(|state| state.self_deaf).unwrap_or(false)
			|| old.as_ref().map(|state| state.deaf).unwrap_or(false);
		let new_deaf = new.self_deaf || new.deaf;

		match (old_deaf, new_deaf) {
			(false, true) => {
				println!(
					"[INFO] {}[{}] deafen himself or has been deafen",
					username, user_id
				);
				self.user_deafen(user_id).await;
			}
			(true, false) => {
				println!(
					"[INFO] {}[{}] undeafen himself or has been undeafen",
					username, user_id
				);
				self.user_undeafen(user_id).await;
			}
			_ => {}
		}

		// On video && Not on video
		let old_video = old.as_ref().map(|state| state.self_video).unwrap_or(false);
		let new_video = new.self_video;

		match (old_video, new_video) {
			(false, true) => {
				println!("[INFO] {}[{}] started a video", username, user_id);
				self.user_started_video(user_id).await;
			}
			(true, false) => {
				println!("[INFO] {}[{}] stopped a video", username, user_id);
				self.user_stopped_video(user_id).await;
			}
			_ => {}
		}
	}
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
				//poise::builtins::register_globally(ctx, &framework.options().commands).await?;
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
