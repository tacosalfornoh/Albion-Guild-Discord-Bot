use serenity::model::prelude::*;
use serenity::prelude::*;

pub async fn handle_message(_ctx: Context, msg: Message) {
    if msg.content.starts_with("!ping") {
        // Call ping command handler
    } else if msg.content.starts_with("!application") {
        // Call application command handler
    } else if msg.content.starts_with("!permission") {
        // Call permission command handler
    } else if msg.content.starts_with("/user") {
        // Call API command handler
    }
}