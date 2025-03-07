use serenity::all::{Context, Message};
use serenity::builder::CreateChannel;
use serenity::model::channel::ChannelType;
use serenity::model::id::GuildId;

pub async fn handle_message(_ctx: Context, msg: Message) {
    if msg.content.starts_with("!ping") {
        println!("Ping command");
        // Call ping command handler
    } else if msg.content.starts_with("!join") {
        let guild_id = msg.guild_id.unwrap();
        // Call application command handler
        let builder = CreateChannel::new("test")
            .kind(ChannelType::Text);
        let _channel = GuildId::new(u64::from(guild_id)).create_channel(&_ctx.http, builder).await;
    } else if msg.content.starts_with("!permission") {
        println!("Permission command");
        // Call permission command handler
    } else if msg.content.starts_with("/user") {
        // Call API command handler
        println!("User command");
    }
}