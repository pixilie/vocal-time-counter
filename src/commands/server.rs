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

/// Display server statistics
#[poise::command(slash_command, ephemeral)]
pub async fn server(
	ctx: Context<'_>,
	#[description = "Number of channels to display (default: 10, max: 20)"] limit: Option<usize>,
) -> Result<(), Error> {
	let db = ctx.data().read().await;

	let mut total_time = 0.0;
	let mut total_stream = 0.0;
	let mut total_video = 0.0;
	let mut total_muted = 0.0;
	let mut total_deafened = 0.0;

	for member in &db.members {
		total_time += member.time;
		total_stream += member.time_streaming;
		total_video += member.time_video;
		total_muted += member.time_muted;
		total_deafened += member.time_defeaned;
	}

	let mut channels: Vec<_> = db.channels.iter().collect();
	channels.sort_by(|a, b| {
		b.time
			.partial_cmp(&a.time)
			.unwrap_or(std::cmp::Ordering::Equal)
	});

	let limit = limit.unwrap_or(10).clamp(1, 20);
	let top_channels = channels.into_iter().take(limit);

	let mut ranking_desc = String::from("**🏆 Voice channels ranking**\n");
	let mut has_channels = false;

	for (index, channel) in top_channels.enumerate() {
		has_channels = true;
		let rank_emoji = match index {
			0 => "🥇",
			1 => "🥈",
			2 => "🥉",
			_ => "🏅",
		};

		ranking_desc.push_str(&format!(
			"{} **{}** - {}\n",
			rank_emoji,
			channel.name,
			format_duration(channel.time)
		));
	}

	if !has_channels {
		ranking_desc.push_str("No voice channel has been used.\n");
	}

	ranking_desc.push_str("\n\u{200B}");

	let embed = CreateEmbed::new()
		.title("🌍 Server's statistics")
		.color(Color::BLURPLE)
		.description(ranking_desc)
		.field("⏱️ Total time", format_duration(total_time), true)
		.field(
			"📺 Total time streamed",
			format_duration(total_stream),
			true,
		)
		.field("🎥 Total time video", format_duration(total_video), true)
		.field("🙊 Total time muted", format_duration(total_muted), true)
		.field(
			"🎧 Total time deafen",
			format_duration(total_deafened),
			true,
		)
		.field("\u{200B}", "\u{200B}", true);

	ctx.send(CreateReply::default().embed(embed)).await?;

	Ok(())
}
