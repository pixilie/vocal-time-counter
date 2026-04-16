use crate::{
	models::{Channel, DataFile, Member},
	write_database,
};

use chrono::Utc;
use serenity::all::{ChannelId, UserId, VoiceState};
use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct Handler {
	pub db: Arc<RwLock<DataFile>>,
}

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, _: Context, ready: Ready) {
		println!("[INFO] {} is connected!", ready.user.name);
	}

	async fn voice_state_update(&self, ctx: Context, old: Option<VoiceState>, new: VoiceState) {
		let user_id = new.user_id;
		let username = new.member.as_ref().unwrap().user.name.clone();

		let old_channel = old.as_ref().and_then(|state| state.channel_id);
		let new_channel = new.channel_id;

		match (old_channel, new_channel) {
			(None, Some(id)) => {
				println!("[INFO] {} joined channel {}", username, id);
				self.user_joined(&new, user_id, &username).await;
			}
			(Some(id), None) => {
				println!("[INFO] {} left channel {}", username, id);
				self.user_left(&ctx, &old.as_ref().unwrap(), user_id, id)
					.await;
			}
			(Some(old_id), Some(new_id)) if old_id != new_id => {
				println!("[INFO] {} moved from {} to {}", username, old_id, new_id);
				self.user_left(&ctx, &old.as_ref().unwrap(), user_id, old_id)
					.await;
				self.user_joined(&new, user_id, &username).await;
			}
			_ => {}
		}

		let old_streaming = old
			.as_ref()
			.and_then(|state| state.self_stream)
			.unwrap_or(false);
		let new_streaming = new.self_stream.unwrap_or(false);

		match (old_streaming, new_streaming) {
			(false, true) => {
				println!("[INFO] {} started streaming", username);
				self.user_started_streaming(user_id).await;
			}
			(true, false) => {
				println!("[INFO] {} stopped streaming", username);
				self.user_stopped_streaming(user_id).await;
			}
			_ => {}
		}

		let old_mute = old.as_ref().map(|state| state.self_mute).unwrap_or(false);
		let new_mute = new.self_mute;

		match (old_mute, new_mute) {
			(false, true) => {
				println!("[INFO] {} muted himself", username);
				self.user_muted(user_id).await;
			}
			(true, false) => {
				println!("[INFO] {} unmuted himself", username);
				self.user_unmuted(user_id).await;
			}
			_ => {}
		}

		let old_deaf = old.as_ref().map(|state| state.self_deaf).unwrap_or(false);
		let new_deaf = new.self_deaf;

		match (old_deaf, new_deaf) {
			(false, true) => {
				println!("[INFO] {} deafen himself", username);
				self.user_deafen(user_id).await;
			}
			(true, false) => {
				println!("[INFO] {} undeafen himself", username);
				self.user_undeafen(user_id).await;
			}
			_ => {}
		}

		let old_video = old.as_ref().map(|state| state.self_video).unwrap_or(false);
		let new_video = new.self_video;

		match (old_video, new_video) {
			(false, true) => {
				println!("[INFO] {} started a video", username);
				self.user_started_video(user_id).await;
			}
			(true, false) => {
				println!("[INFO] {} stopped a video", username);
				self.user_stopped_video(user_id).await;
			}
			_ => {}
		}
	}
}

impl Handler {
	pub(crate) async fn user_joined(&self, state: &VoiceState, user_id: UserId, username: &str) {
		let mut db = self.db.write().await;
		let now = Utc::now();
		let user_exists = db.members.iter().any(|m| m.id == user_id.get());

		if !user_exists {
			db.members.push(Member {
				id: user_id.get(),
				username: username.to_string(),
				time: 0.0,
				time_muted: 0.0,
				time_defeaned: 0.0,
				time_streaming: 0.0,
				time_video: 0.0,
				last_active: now,
				last_mute: now,
				last_deafen: now,
				last_stream: now,
				last_video: now,
			});
		} else {
			let user = db
				.members
				.iter_mut()
				.find(|m| m.id == user_id.get())
				.unwrap();

			user.last_active = now;

			if state.mute || state.self_mute {
				user.last_mute = now;
			}
			if state.deaf || state.self_deaf {
				user.last_deafen = now;
			}
			if state.self_stream.unwrap_or(false) {
				user.last_stream = now;
			}
			if state.self_video {
				user.last_video = now;
			}
		}

		write_database(&db);
	}

	pub(crate) async fn user_left(
		&self,
		ctx: &Context,
		state: &VoiceState,
		user_id: UserId,
		channel_id: ChannelId,
	) {
		let channel_name: String = match channel_id.to_channel(ctx).await {
			Ok(serenity::all::Channel::Guild(guild_channel)) => guild_channel.name.clone(),
			_ => "Unknown channel".to_string(),
		};

		let mut db = self.db.write().await;
		let now = Utc::now();

		let user = db
			.members
			.iter_mut()
			.find(|m| m.id == user_id.get())
			.unwrap();
		let time_delta = (now - user.last_active).as_seconds_f64();

		user.time += time_delta;

		if state.mute || state.self_mute {
			user.time_muted += (now - user.last_mute).as_seconds_f64();
		}
		if state.deaf || state.self_deaf {
			user.time_defeaned += (now - user.last_deafen).as_seconds_f64();
		}
		if state.self_stream.unwrap_or(false) {
			user.time_streaming += (now - user.last_stream).as_seconds_f64();
		}
		if state.self_video {
			user.time_video += (now - user.last_video).as_seconds_f64();
		}

		let channel_exists = db.channels.iter().any(|c| c.id == channel_id.get());

		if !channel_exists {
			db.channels.push(Channel {
				id: channel_id.get(),
				name: channel_name,
				time: time_delta,
			});
		} else {
			let channel = db
				.channels
				.iter_mut()
				.find(|c| c.id == channel_id.get())
				.unwrap();

			channel.time += time_delta;

			if channel.name != channel_name && channel_name != "Unknown channel" {
				channel.name = channel_name;
			}
		}

		write_database(&db);
	}

	pub(crate) async fn user_started_streaming(&self, user_id: UserId) {
		let mut db = self.db.write().await;
		let user = db
			.members
			.iter_mut()
			.find(|m| m.id == user_id.get())
			.unwrap();
		user.last_stream = Utc::now();
		write_database(&db);
	}

	pub(crate) async fn user_stopped_streaming(&self, user_id: UserId) {
		let mut db = self.db.write().await;
		let user = db
			.members
			.iter_mut()
			.find(|m| m.id == user_id.get())
			.unwrap();
		user.time_streaming += (Utc::now() - user.last_stream).as_seconds_f64();
		write_database(&db);
	}

	pub(crate) async fn user_muted(&self, user_id: UserId) {
		let mut db = self.db.write().await;
		let user = db
			.members
			.iter_mut()
			.find(|m| m.id == user_id.get())
			.unwrap();
		user.last_mute = Utc::now();
		write_database(&db);
	}

	pub(crate) async fn user_unmuted(&self, user_id: UserId) {
		let mut db = self.db.write().await;
		let user = db
			.members
			.iter_mut()
			.find(|m| m.id == user_id.get())
			.unwrap();
		user.time_muted += (Utc::now() - user.last_mute).as_seconds_f64();
		write_database(&db);
	}

	pub(crate) async fn user_deafen(&self, user_id: UserId) {
		let mut db = self.db.write().await;
		let user = db
			.members
			.iter_mut()
			.find(|m| m.id == user_id.get())
			.unwrap();
		user.last_deafen = Utc::now();
		write_database(&db);
	}

	pub(crate) async fn user_undeafen(&self, user_id: UserId) {
		let mut db = self.db.write().await;
		let user = db
			.members
			.iter_mut()
			.find(|m| m.id == user_id.get())
			.unwrap();
		user.time_defeaned += (Utc::now() - user.last_deafen).as_seconds_f64();
		write_database(&db);
	}

	pub(crate) async fn user_started_video(&self, user_id: UserId) {
		let mut db = self.db.write().await;
		let user = db
			.members
			.iter_mut()
			.find(|m| m.id == user_id.get())
			.unwrap();
		user.last_video = Utc::now();
		write_database(&db);
	}

	pub(crate) async fn user_stopped_video(&self, user_id: UserId) {
		let mut db = self.db.write().await;
		let user = db
			.members
			.iter_mut()
			.find(|m| m.id == user_id.get())
			.unwrap();
		user.time_video += (Utc::now() - user.last_video).as_seconds_f64();
		write_database(&db);
	}
}
