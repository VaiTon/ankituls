use ankiconnect::note::NoteInfo;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Export {
    pub deck_name: String,
    pub cards: Vec<NoteInfo>,
}
