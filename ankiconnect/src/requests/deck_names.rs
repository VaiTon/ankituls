use serde::{Deserialize, Serialize};

use crate::AnkiRequestable;

#[derive(Debug, Serialize)]
pub struct DeckNamesRequest;

#[derive(Debug, Deserialize)]
pub struct DeckNamesResponse(pub Vec<String>);

impl AnkiRequestable for DeckNamesRequest {
    type Response = DeckNamesResponse;

    const ACTION: &'static str = "deckNames";
    const VERSION: u16 = 6;

    fn params(self) -> Option<Self>
    where
        Self: Sized,
    {
        None
    }
}
