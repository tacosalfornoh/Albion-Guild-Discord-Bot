use serenity::all::{CreateInteractionResponseMessage, CreateInteractionResponse};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::*;

pub async fn handle_interaction(ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
        let content = match command.data.name.as_str() {
            "ping" => Some(ping::run(&command.data.options())),
            "balance" => Some(balance::run(&command.data.options())),
            _ => Some("not implemented :(".to_string()),
        };

        if let Some(content) = content {
            let data = CreateInteractionResponseMessage::new().content(content);
            let builder = CreateInteractionResponse::Message(data);
            if let Err(why) = command.create_response(&ctx.http, builder).await {
                println!("Cannot respond to slash command: {why}");
            }
        }
    }
}