use crate::models::{Context, Error};
use poise::CreateReply;
use poise::serenity_prelude as serenity;
use poise::serenity_prelude::{Color, CreateEmbed};
use serenity::Member;

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

/// Get the stats of any user in the server
#[poise::command(slash_command, ephemeral)]
pub async fn time(
	ctx: Context<'_>,
	#[description = "Discord user you want to see the stats"] user: Option<Member>,
) -> Result<(), Error> {
	let target = match &user {
		Some(member) => &member.user,
		None => ctx.author(),
	};

	let db = ctx.data().read().await;
	let db_user = db.members.iter().find(|m| m.id == target.id.get());

	let embed = match db_user {
		Some(stats) => CreateEmbed::new()
			.title(format!("📊 Voice channel stats of {}", target.name))
			.color(Color::BLURPLE)
			.thumbnail(target.face())
			.field("⏱️ Total time", format_duration(stats.time), false)
			.field("🙊 Mute", format_duration(stats.time_muted), true)
			.field("🎧 Deaf", format_duration(stats.time_defeaned), true)
			.field("\u{200B}", "\u{200B}", true)
			.field("📺 Streaming", format_duration(stats.time_streaming), true)
			.field("🎥 Video", format_duration(stats.time_video), true)
			.field("\u{200B}", "\u{200B}", true)
			.field(
				"📅 Last seen",
				format!("<t:{}:R>", stats.last_active.timestamp()),
				false,
			),
		None => CreateEmbed::new()
			.title("❌ User not found")
			.description(format!("No stats found for **{}**.", target.name))
			.color(Color::RED),
	};

	ctx.send(CreateReply::default().embed(embed)).await?;
	Ok(())
}
