use std::collections::HashMap;

use ankiconnect::{
    requests::{CreateNote, CreateNoteOptions},
    NoteInfo,
};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

mod export;
mod import;

pub use export::*;
pub use import::*;

#[derive(Serialize, Deserialize)]
pub struct Export {
    pub deck_name: String,
    pub notes: Vec<NoteInfo>,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ExportFormat {
    /// Anki package
    Apkg,

    /// TOML representation of the deck, for collaboration and consumption by other tools
    Toml,
}

impl ToString for ExportFormat {
    fn to_string(&self) -> String {
        match self {
            ExportFormat::Toml => "toml",
            ExportFormat::Apkg => "apkg",
        }
        .to_string()
    }
}

impl From<Export> for Vec<CreateNote> {
    fn from(value: Export) -> Self {
        value
            .notes
            .into_iter()
            .map(|info| {
                let fields_values = info
                    .fields
                    .into_iter()
                    .map(|(key, value)| (key, value.value))
                    .collect::<HashMap<String, String>>();

                let options = CreateNoteOptions {
                    allow_duplicate: true,
                    duplicate_scope: "deck".to_string(),
                };

                CreateNote {
                    deck_name: value.deck_name.clone(),
                    model_name: info.model_name,
                    fields: fields_values,
                    tags: info.tags,
                    options: Some(options),
                }
            })
            .collect()
    }
}
