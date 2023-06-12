use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NoteInfo {
    #[serde(rename = "noteId")]
    pub note_id: u64,
    pub tags: Vec<String>,
    pub fields: HashMap<String, NoteField>,
    #[serde(rename = "modelName")]
    pub model_name: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct NoteField {
    pub value: String,
    pub order: u64,
}
