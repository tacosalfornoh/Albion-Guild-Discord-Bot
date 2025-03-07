use crate::api::errors::MyError;
use reqwest::Client;

// Allow dead code since we may add more functionality later
#[allow(dead_code)]
pub enum FetchType {
    GET,
    POST,
    PUT,
    DELETE,
    // Tolgo gli altri perché non so come usarli per ora -> breve spiegazione:
    // PATCH, Modifica parziale di una risorsa
    // HEAD, Ottiene solo l'intestazione della risorsa
    // OPTIONS, Ottiene le opzioni di comunicazione per una risorsa
    // TRACE, Esegue il tracciamento di una richiesta
    // CONNECT, Prepara una connessione tunnel
}

// Funzione per effettuare la chiamata API (asincrona)
pub async fn fetch_data(fetch_type: FetchType, url: &str, body: Option<serde_json::Value>) -> Result<String, MyError> {
    let client = Client::new();
    let response = match fetch_type {
        FetchType::GET => client.get(url),
        FetchType::POST => client.post(url),
        FetchType::PUT => client.put(url),
        FetchType::DELETE => client.delete(url),
        // FetchType::PATCH => blocking::Client::new().patch(url).send()?,
        // FetchType::HEAD => blocking::Client::new().head(url).send()?,
        // FetchType::OPTIONS => blocking::Client::new().request(reqwest::Method::OPTIONS, url).send()?,
        // FetchType::TRACE => blocking::Client::new().request(reqwest::Method::TRACE, url).send()?,
        // FetchType::CONNECT => blocking::Client::new().request(reqwest::Method::CONNECT, url).send()?,
    };
    
    let response = if let Some(body_data) = body {
        response
            .header("Content-Type", "application/json")
            .json(&body_data)
    } else {
        response
    }.send().await?;
    
    let text = response.text().await?;
    Ok(text)
}
