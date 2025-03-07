use crate::api::errors::MyError;
use crate::models::discord::Discord;    

// Funzione per deserializzare il JSON
pub fn deserialize_json(response: &str) -> Result<Discord, MyError> {
    serde_json::from_str(response).map_err(MyError::SerdeError)
}