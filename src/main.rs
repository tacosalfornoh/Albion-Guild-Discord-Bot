mod commands;
mod api;
mod models;
mod handlers;
mod utils;

use dotenv::dotenv;
use serenity::prelude::*;
use std::env;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    println!("Bot is running...");


    let mut client = Client::builder(&token, GatewayIntents::all())
        .event_handler(handlers::Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}