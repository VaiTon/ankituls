use serde::{Deserialize, Serialize};

use crate::AnkiRequestable;

#[derive(Serialize, Debug)]
pub struct DeleteDecksRequest {
    pub decks: Vec<String>,
    #[serde(rename = "cardsToo")]
    pub cards_too: bool,
}

#[derive(Debug, Deserialize)]
pub struct DeleteDecksResponse;

impl AnkiRequestable for DeleteDecksRequest {
    type Response = DeleteDecksResponse;
    const ACTION: &'static str = "deleteDecks";
    const VERSION: u16 = 6;
}
