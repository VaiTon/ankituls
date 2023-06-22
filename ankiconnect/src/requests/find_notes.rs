use serde::{Deserialize, Serialize};

use crate::{AnkiRequestable, NoteId};

#[derive(Debug, Serialize)]
pub struct FindNotesRequest {
    pub query: String,
}

#[derive(Debug, Deserialize)]
pub struct FindNotesResponse(pub Vec<NoteId>);

impl AnkiRequestable for FindNotesRequest {
    type Response = FindNotesResponse;

    const ACTION: &'static str = "findNotes";
    const VERSION: u16 = 6;
}
