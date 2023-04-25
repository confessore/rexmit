use serenity::{prelude::Context, model::prelude::PartialGuild};

pub async fn context_get_guild(ctx: &Context, guild_id: u64) -> Option<PartialGuild> {
    let partial_guild_result = ctx.http.get_guild(guild_id).await;
    if partial_guild_result.is_ok() {
        let partial_guild = partial_guild_result.unwrap();
        return Some(partial_guild)
    }
    println!("context guild not found");
    return None;
}