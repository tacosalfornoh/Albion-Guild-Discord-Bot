use serenity::builder::CreateCommand;
use serenity::model::application::ResolvedOption;
use crate::api::MyError;
use crate::api::{fetch_data, deserialize_json};
use crate::api::fetch::FetchType;
use crate::models::discord::Discord;

// Funzione per registrare il comando "balance".
pub fn register() -> CreateCommand {
    CreateCommand::new("balance").description("Replies with Balance!!")
}

// Funzione principale del comando "balance".
pub async fn run<'a>(_options: &'a [ResolvedOption<'a>]) -> String {
    // Esegue la chiamata API e la deserializzazione in modo asincrono.
    let result: Result<Discord, MyError> = async {
        let response = fetch_data(FetchType::PUT, "http://127.0.0.1:8000/discord/join", None).await?;
        let data = deserialize_json(&response)?;
        Ok(data)
    }
    .await;

    match result {
        Ok(discord_data) => {
            format!(
                "ID: {:?}, Nome: {:?}, Data di adesione: {:?}, Saldo: {:?}",
                discord_data.discord_id,
                discord_data.discord_name,
                discord_data.joined_at,
                discord_data.balance
            )
        },
        Err(err) => {
            format!("Errore durante la deserializzazione dei dati: {:?}", err)
        }
    }
}