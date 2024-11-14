use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::*;

pub async fn ready(ctx: Context, ready: Ready) {
    println!("{} is connected!", ready.user.name);

    // TODO: Registering slash commands for a guild (discord)
    let guild_id = GuildId::new(
        "940226887463100416" // * 940226887463100416 is the ID of the guild I'm in
            .parse()
            .expect("Failed to parse guild ID"),
    );

    let _commands = guild_id
        .set_commands(&ctx.http, vec![ping::register()])
        .await;

    println!("Registered guild commands");

    // TODO: Registering global slash commands
    let _guild_command = Command::set_global_commands(
        &ctx.http,
        vec![
            ping::register(),
            balance::register(),
        ],
    )
    .await;
    println!("Registered global commands");
}
