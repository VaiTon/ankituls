use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Deserialize, Serialize)]
pub struct CreateNote {
    #[serde(rename = "deckName")]
    pub deck_name: String,
    #[serde(rename = "modelName")]
    pub model_name: String,
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
pub struct AddNotesRequest {
    pub notes: Vec<CreateNote>,
}

#[derive(Debug, Deserialize)]
pub struct AddNotesResponse(pub Vec<Option<Value>>);

impl From<AddNotesRequest> for super::AnkiRequest<AddNotesRequest> {
    fn from(value: AddNotesRequest) -> Self {
        super::AnkiRequest {
            action: "addNotes",
            version: 6,
            params: Some(value),
        }
    }
}
