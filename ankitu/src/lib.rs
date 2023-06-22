use std::collections::HashMap;

use ankiconnect::{requests::CreateNote, CardId, NoteInfo};

use clap::ValueEnum;
use serde::{Deserialize, Serialize};

pub mod eximp;

#[derive(Serialize, Deserialize, Debug)]
pub struct Export {
    pub deck_name: String,
    pub notes: Vec<NoteInfo>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExportCardMod {
    pub card_id: CardId,
    pub time: u32,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ExportFormat {
    /// TOML representation of the deck, for collaboration and consumption by other tools
    Toml,
}
impl ToString for ExportFormat {
    fn to_string(&self) -> String {
        match self {
            ExportFormat::Toml => "toml".to_owned(),
        }
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

                CreateNote {
                    deck_name: value.deck_name.clone(),
                    model_name: info.model_name,
                    fields: fields_values,
                    tags: info.tags,
                    options: None,
                }
            })
            .collect()
    }
}
