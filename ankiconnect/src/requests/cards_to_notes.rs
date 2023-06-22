use serde::{Deserialize, Serialize};

use crate::{AnkiRequestable, CardId, NoteId};

#[derive(Debug, Serialize)]
pub struct CardsToNotesRequest {
    #[serde(rename = "cards")]
    pub ids: Vec<CardId>,
}

#[derive(Debug, Deserialize)]
pub struct CardsToNotesResponse(pub Vec<NoteId>);

impl AnkiRequestable for CardsToNotesRequest {
    type Response = CardsToNotesResponse;

    const ACTION: &'static str = "cardsToNotes";
    const VERSION: u16 = 6;
}
