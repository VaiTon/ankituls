use crate::{note::NoteInfo, AnkiRequestable, NoteId};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct NotesInfoRequest {
    pub notes: Vec<NoteId>,
}

#[derive(Debug, Deserialize)]

pub struct NotesInfoResponse(pub Vec<NoteInfo>);

impl AnkiRequestable for NotesInfoRequest {
    type Response = NotesInfoResponse;

    const ACTION: &'static str = "notesInfo";
    const VERSION: u16 = 6;
}
