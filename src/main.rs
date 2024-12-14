mod commands;

use sqlx::postgres::PgPoolOptions;
use sqlx::Row;

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

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://postgres:umpalumpa@192.168.1.214:5432/umpalumpa")
        .await?;

    let rows = sqlx::query("SELECT * FROM discord_guilds")
        .fetch_all(&pool)
        .await?;

    for row in rows {
        let id: i64 = row.get("id"); // Presupponi che ci sia una colonna `id`
        let name: String = row.get("name"); // Presupponi che ci sia una colonna `name`
        println!("Guild ID: {}, Name: {}", id, name);
    }


    let mut client = Client::builder(&token, GatewayIntents::all())
        .event_handler(handlers::Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }

    Ok(())
}