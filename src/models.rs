use chrono::{DateTime, Utc};
use dotenvy::var;
use serde::{Deserialize, Serialize};

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
	pub(crate) version: u64,
	pub(crate) members: Vec<(i64, Member)>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(crate) struct Member {
	pub(crate) id: u64,
	pub(crate) username: String,
	pub(crate) time: u64,
	pub(crate) time_streaming: u64,
	pub(crate) time_muted: u64,
	pub(crate) time_defeaned: u64,
	pub(crate) last_seen: DateTime<Utc>,
}

pub(crate) type Error = Box<dyn std::error::Error + Send + Sync>;
pub(crate) type Context<'a> = poise::Context<'a, Data, Error>;
pub(crate) struct Data {}
