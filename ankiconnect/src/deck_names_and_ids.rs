use serde::{Deserialize, Serialize};

use super::AnkiRequest;

#[derive(Debug, Serialize)]
pub struct DeckNamesRequest;

#[derive(Debug, Deserialize)]
pub struct DeckNamesResponse(pub Vec<String>);

impl From<DeckNamesRequest> for AnkiRequest<DeckNamesRequest> {
    fn from(_: DeckNamesRequest) -> Self {
        AnkiRequest {
            action: "deckNames",
            version: 6,
            params: None,
        }
    }
}
