pub mod errors; // Dichiara il modulo errors (il file errors.rs)
pub mod fetch; // Aggiungi il modulo fetch
pub mod deserialize; // Aggiungi il modulo deserialize

pub use errors::MyError; // Riesporta MyError
pub use fetch::fetch_data; // Riesporta fetch_data
pub use deserialize::deserialize_json; // Riesporta deserialize_json

