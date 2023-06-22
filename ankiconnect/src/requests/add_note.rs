use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::AnkiRequestable;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNote {
    #[serde(rename = "deckName")]
    pub deck_name: String,
    #[serde(rename = "modelName")]
    pub model_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<CreateNoteOptions>,
    pub fields: HashMap<String, String>,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNoteOptions {
    #[serde(rename = "allowDuplicate")]
    pub allow_duplicate: bool,
    #[serde(rename = "duplicateScope")]
    pub duplicate_scope: String,
}

#[derive(Debug, Serialize)]
pub struct AddNoteRequest {
    pub note: CreateNote,
}

#[derive(Debug, Deserialize)]
pub struct AddNoteResponse(pub Option<u32>);

impl AnkiRequestable for AddNoteRequest {
    type Response = AddNoteResponse;
    const ACTION: &'static str = "addNote";
    const VERSION: u16 = 6;
}
