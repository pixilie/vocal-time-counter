use crate::models::{Context, Error};
use chrono::Utc;

/// Test the bot response time
#[poise::command(slash_command, ephemeral)]
pub async fn ping(ctx: Context<'_>) -> Result<(), Error> {
	let ping = (Utc::now() - *ctx.created_at()).num_milliseconds();
	let response = format!("{} ms", ping);

	ctx.say(response).await?;
	Ok(())
}
