use serenity::prelude::*;
use serenity::model::guild::Guild;
use crate::api::fetch::{fetch_data, FetchType};
use serde_json::json;
use crate::models::discord::Discord;
use chrono;

// Gestisce l'evento GuildCreate (quando il bot viene aggiunto a un server).
pub async fn handle_guild_create(ctx: Context, guild: Guild, _is_new: Option<bool>) {
    // Trova il canale predefinito (o un canale specifico).
    if let Some(channel_id) = guild.system_channel_id {
        // Invia il messaggio di saluto.
        let _ = channel_id.say(&ctx.http, "Ciao a tutti! Sono un bot.").await;
    }

    // Ottieni i dati reali del server
    let discord_data = Discord {
        discord_id: Some(guild.id.get() as i64), // ID del server
        discord_name: Some(guild.name), // Nome del server
        // joined_at: Some(chrono::Utc::now().to_rfc3339()), // Data e ora correnti
        joined_at: Some(guild.joined_at.map(|ts| ts.to_rfc3339()))
        balance: Some(0), // Imposta il saldo iniziale a 0 (o un altro valore predefinito)
    };
    
    // Construct the URL
    let discord_id = discord_data.discord_id.unwrap_or(0);
    let url = format!("http://127.0.0.1:8000/api/v1/discord/{}", discord_id);
    // Make the PUT request using fetch_data (synchronous)
    let result = fetch_data(FetchType::PUT, &url, Some(json!({
        "discord_id": discord_id,
        "discord_name": discord_data.discord_name.clone(),
        "joined_at": discord_data.joined_at.clone(),
        "balance": discord_data.balance.clone()
    })));

    match result.await {
        Ok(response_text) => {
            println!("API call successful. Response: {}", response_text);
        },
        Err(err) => {
            println!("API call failed: {:?}", err);
        }
    }
}