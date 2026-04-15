use crate::models::{Context, Error};
use poise::serenity_prelude as serenity;

#[poise::command(slash_command)]
pub async fn leaderboard(
	ctx: Context<'_>,
	#[description = "Description of arg1 here"] arg1: serenity::Member,
	#[description = "Description of arg2 here"] arg2: Option<u32>,
) -> Result<(), Error> {
	// Command code here

	Ok(())
}
