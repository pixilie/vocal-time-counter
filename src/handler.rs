use crate::{
	Handler,
	models::{Channel, Member},
	write_database,
};

use chrono::Utc;
use serenity::all::{ChannelId, UserId, VoiceState};
use serenity::prelude::*;

impl Handler {
	pub(crate) async fn user_joined(&self, user_id: UserId, username: &String) {
		let mut db = self.db.write().await;
		let user_exists = db.members.iter().any(|m| m.id == user_id.get());

		if !user_exists {
			println!(
				"[INFO] New user created, id: {}, name: {}",
				user_id.get(),
				username.clone()
			);

			db.members.push(Member {
				id: user_id.get(),
				username: username.clone(),
				time: 0 as f64,
				time_muted: 0 as f64,
				time_defeaned: 0 as f64,
				time_streaming: 0 as f64,
				time_video: 0 as f64,
				last_active: Utc::now(),
				last_stream: Utc::now(),
				last_video: Utc::now(),
			});
		} else {
			let user = db
				.members
				.iter_mut()
				.find(|m| m.id == user_id.get())
				.unwrap();

			user.last_active = Utc::now();
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
			Ok(_) => "Unknown channel".to_string(),
			Err(_) => "Unknown channel".to_string(),
		};

		let mut db = self.db.write().await;

		let user = db
			.members
			.iter_mut()
			.find(|m| m.id == user_id.get())
			.unwrap();

		let time_delta = (Utc::now() - user.last_active).as_seconds_f64();

		if state.deaf || state.self_deaf {
			user.time_defeaned += time_delta;
		} else if state.mute || state.self_mute {
			user.time_muted += time_delta;
		} else {
			user.time += time_delta;
		}

		user.last_active = Utc::now();

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

			if channel.name != channel_name && channel_name != "Unkown channel" {
				channel.name = channel_name;
			}
		}

		write_database(&db);
	}

	pub(crate) async fn user_moved(&self, ctx: &Context, user_id: UserId, channel_id: ChannelId) {
		let channel_name: String = match channel_id.to_channel(ctx).await {
			Ok(serenity::all::Channel::Guild(guild_channel)) => guild_channel.name.clone(),
			Ok(_) => "Unknown channel".to_string(),
			Err(_) => "Unknown channel".to_string(),
		};

		let mut db = self.db.write().await;

		let user = db
			.members
			.iter_mut()
			.find(|m| m.id == user_id.get())
			.unwrap();

		let time_delta = (Utc::now() - user.last_active).as_seconds_f64();

		user.time += time_delta;
		user.last_active = Utc::now();

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

		user.time += (Utc::now() - user.last_active).as_seconds_f64();
		user.last_active = Utc::now();

		write_database(&db);
	}

	pub(crate) async fn user_unmuted(&self, user_id: UserId) {
		let mut db = self.db.write().await;

		let user = db
			.members
			.iter_mut()
			.find(|m| m.id == user_id.get())
			.unwrap();

		user.time_muted += (Utc::now() - user.last_active).as_seconds_f64();
		user.last_active = Utc::now();

		write_database(&db);
	}

	pub(crate) async fn user_deafen(&self, user_id: UserId) {
		let mut db = self.db.write().await;

		let user = db
			.members
			.iter_mut()
			.find(|m| m.id == user_id.get())
			.unwrap();

		user.time += (Utc::now() - user.last_active).as_seconds_f64();
		user.last_active = Utc::now();

		write_database(&db);
	}

	pub(crate) async fn user_undeafen(&self, user_id: UserId) {
		let mut db = self.db.write().await;

		let user = db
			.members
			.iter_mut()
			.find(|m| m.id == user_id.get())
			.unwrap();

		user.time_defeaned += (Utc::now() - user.last_active).as_seconds_f64();
		user.last_active = Utc::now();

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
