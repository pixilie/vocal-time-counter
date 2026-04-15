use models::{Config, Data, DataFile, Member};

use serenity::async_trait;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

mod commands;
mod models;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn ready(&self, _: Context, ready: Ready) {
		println!("{} is connected!", ready.user.name);
	}
}

#[tokio::main]
async fn main() {
	let config = Config::load();
	let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::GUILD_VOICE_STATES;

	let framework = poise::Framework::builder()
		.options(poise::FrameworkOptions {
			commands: vec![commands::ping(), commands::time(), commands::leaderboard()],
			..Default::default()
		})
		.setup(|ctx, _ready, framework| {
			Box::pin(async move {
				poise::builtins::register_globally(ctx, &framework.options().commands).await?;
				Ok(Data {})
			})
		})
		.build();

	let mut client = Client::builder(&config.token, intents)
		.event_handler(Handler)
		.framework(framework)
		.await
		.expect("Error while creating client");

	if let Err(why) = client.start().await {
		println!("Client error: {why:?}");
	}
}
