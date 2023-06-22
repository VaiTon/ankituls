use serde::{Deserialize, Serialize};

use crate::{AnkiRequest, AnkiRequestable, DeckId};

#[derive(Serialize, Debug)]
pub struct CreateDeckRequest {
    #[serde(rename = "deck")]
    pub deck: String,
}

#[derive(Deserialize, Debug)]
pub struct CreateDeckResponse(DeckId);

impl From<CreateDeckRequest> for AnkiRequest<CreateDeckRequest> {
    fn from(value: CreateDeckRequest) -> Self {
        AnkiRequest {
            action: "createDeck",
            version: 6,
            params: Some(value),
        }
    }
}

impl AnkiRequestable for CreateDeckRequest {
    type Response = CreateDeckResponse;

    const ACTION: &'static str = "createDeck";
    const VERSION: u16 = 6;
}
