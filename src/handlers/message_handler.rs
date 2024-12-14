use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_message(_ctx: Context, msg: Message) {
    if msg.content.starts_with("!ping") {
        println!("Ping command");
        // Call ping command handler
    } else if msg.content.starts_with("!application") {
        println!("Application command");
        // Call application command handler
    } else if msg.content.starts_with("!permission") {
        println!("Permission command");
        // Call permission command handler
    } else if msg.content.starts_with("/user") {
        // Call API command handler
        println!("User command");
    }
}