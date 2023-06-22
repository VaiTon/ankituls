use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{AnkiRequestable, NoteId};

#[derive(Debug, Serialize)]
pub struct UpdateNoteFieldsRequest {
    pub note: UpdateNoteFields,
}

#[derive(Debug, Serialize)]
pub struct UpdateNoteFields {
    pub id: NoteId,
    pub fields: HashMap<String, String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateNoteFieldsResponse;

impl AnkiRequestable for UpdateNoteFieldsRequest {
    type Response = UpdateNoteFieldsResponse;
    const ACTION: &'static str = "updateNoteFields";
    const VERSION: u16 = 6;
}
