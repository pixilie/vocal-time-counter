use crate::models::{Context, Error};
use poise::CreateReply;
use poise::serenity_prelude::{Color, CreateEmbed};

fn format_duration(seconds: f64) -> String {
	let total_seconds = seconds.max(0.0) as u64;
	let hours = total_seconds / 3600;
	let minutes = (total_seconds % 3600) / 60;
	let secs = total_seconds % 60;

	if hours > 0 {
		format!("{}h {:02}m {:02}s", hours, minutes, secs)
	} else if minutes > 0 {
		format!("{}m {:02}s", minutes, secs)
	} else {
		format!("{}s", secs)
	}
}

/// Display ranking of most active users
#[poise::command(slash_command, ephemeral)]
pub async fn leaderboard(
	ctx: Context<'_>,
	#[description = "Number of user to display (default: 10, max: 25)"] limit: Option<usize>,
) -> Result<(), Error> {
	let db = ctx.data().read().await;
	let mut members: Vec<_> = db.members.iter().collect();
	members.sort_by(|a, b| {
		b.time
			.partial_cmp(&a.time)
			.unwrap_or(std::cmp::Ordering::Equal)
	});

	let limit = limit.unwrap_or(10).clamp(1, 25);
	let top_members = members.into_iter().take(limit);

	let mut embed = CreateEmbed::new()
		.title("🏆 Voice channel activity ranking")
		.color(Color::GOLD);

	let mut description = String::new();

	for (index, user) in top_members.enumerate() {
		let rank_emoji = match index {
			0 => "🥇",
			1 => "🥈",
			2 => "🥉",
			_ => "🏅",
		};

		description.push_str(&format!(
			"{} **{}** - {}\n",
			rank_emoji,
			user.username,
			format_duration(user.time)
		));
	}

	if description.is_empty() {
		description = "The server has been very quiet... No data recorded !".to_string();
	}

	embed = embed.description(description);
	ctx.send(CreateReply::default().embed(embed)).await?;

	Ok(())
}
