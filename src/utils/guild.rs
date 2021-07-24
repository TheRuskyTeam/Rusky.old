use serenity::{
    client::Context,
    model::id::{GuildId, UserId},
};

use crate::RuskyResult;

pub async fn get_guild_owner(context: &Context, guild: &GuildId) -> RuskyResult<UserId> {
    match context.cache.guild(guild).await {
        Some(g) => Ok(g.owner_id),
        None => Ok(context.http.get_guild(*guild.as_u64()).await?.owner_id),
    }
}
