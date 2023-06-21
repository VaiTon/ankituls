use crate::{note::NoteInfo, AnkiRequest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct NotesInfoRequest {
    pub notes: Vec<u64>,
}

#[derive(Debug, Deserialize)]

pub struct NotesInfoResponse(pub Vec<NoteInfo>);

impl From<NotesInfoRequest> for AnkiRequest<NotesInfoRequest> {
    fn from(value: NotesInfoRequest) -> Self {
        AnkiRequest {
            action: "notesInfo",
            version: 6,
            params: Some(value),
        }
    }
}
