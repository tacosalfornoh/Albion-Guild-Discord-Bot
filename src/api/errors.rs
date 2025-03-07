use std::fmt;

// Definisci un tipo di errore personalizzato
#[derive(Debug)]
pub enum MyError {
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error),
    NoData,
}

// Implementa Display per MyError
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MyError::ReqwestError(e) => write!(f, "Errore nella chiamata API: {}", e),
            MyError::SerdeError(e) => write!(f, "Errore nella deserializzazione del JSON: {}", e),
            MyError::NoData => write!(f, "Nessun dato trovato"),
        }
    }
}

// Implementa From per convertire reqwest::Error e serde_json::Error in MyError
impl From<reqwest::Error> for MyError {
    fn from(err: reqwest::Error) -> MyError {
        MyError::ReqwestError(err)
    }
}

impl From<serde_json::Error> for MyError {
    fn from(err: serde_json::Error) -> MyError {
        MyError::SerdeError(err)
    }
}