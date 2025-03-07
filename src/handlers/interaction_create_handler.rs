use serenity::all::{CreateInteractionResponseMessage, CreateInteractionResponse};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::commands::*;

pub async fn handle_interaction(ctx: Context, interaction: Interaction) {
    if let Interaction::Command(command) = interaction {
        // Extracted logic into a separate function
        let content = handle_command(&command).await;

        let data = CreateInteractionResponseMessage::new().content(content);
        let builder = CreateInteractionResponse::Message(data);

        // Log and handle response errors with extra context
        if let Err(why) = command.create_response(&ctx.http, builder).await {
            eprintln!("Error responding to slash command '{}': {why}", command.data.name);
        }
    }
}

// A helper function to handle commands logic
async fn handle_command(command: &CommandInteraction) -> String {
    match command.data.name.as_str() {
        "ping" => ping::run(&command.data.options()),
        "balance" => balance::run(&command.data.options()).await,
        _ => "not implemented :(".to_string(),
    }
}