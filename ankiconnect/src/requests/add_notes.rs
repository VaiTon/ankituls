use serde::{Deserialize, Serialize};

use super::CreateNote;
use crate::{AnkiRequestable, NoteId};

#[derive(Debug, Serialize)]
pub struct AddNotesRequest {
    pub notes: Vec<CreateNote>,
}

#[derive(Debug, Deserialize)]
pub struct AddNotesResponse(pub Vec<Option<NoteId>>);

impl AnkiRequestable for AddNotesRequest {
    type Response = AddNotesResponse;

    const ACTION: &'static str = "addNotes";
    const VERSION: u16 = 6;
}
