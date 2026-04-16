use chrono::{DateTime, Utc};
use dotenvy::var;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone)]
pub(crate) struct Config {
	pub(crate) token: String,
	pub(crate) guild_id: u64,
}

impl Config {
	pub(crate) fn load() -> Self {
		Self {
			token: var("TOKEN").expect("Token expected in the environnement"),
			guild_id: var("GUILD_ID")
				.expect("GuildID expected in the environnement")
				.parse::<u64>()
				.expect("Error while parsing GuildID"),
		}
	}
}

#[derive(Debug, Deserialize, Serialize)]
pub(crate) struct DataFile {
	pub(crate) guild_id: u64,
	pub(crate) guild_name: String,
	pub(crate) members: Vec<Member>,
	pub(crate) channels: Vec<Channel>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct Member {
	pub(crate) id: u64,
	pub(crate) username: String,
	pub(crate) time: f64,
	pub(crate) time_muted: f64,
	pub(crate) time_defeaned: f64,
	pub(crate) time_streaming: f64,
	pub(crate) time_video: f64,
	pub(crate) last_active: DateTime<Utc>,
	pub(crate) last_stream: DateTime<Utc>,
	pub(crate) last_video: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct Channel {
	pub(crate) id: u64,
	pub(crate) name: String,
	pub(crate) time: f64,
}

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Context<'a> = poise::Context<'a, Arc<RwLock<DataFile>>, Error>;
