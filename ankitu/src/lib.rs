use std::{
    collections::HashMap,
    error::Error,
    fs::{self, DirEntry},
    path::PathBuf,
};

use ankiconnect::{CreateNote, CreateNoteOptions, NoteInfo};
use clap::ValueEnum;
use serde::{Deserialize, Serialize};

mod export;
mod import;

pub use export::*;
pub use import::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Format {
    /// Anki package
    Apkg,

    /// TOML representation of the deck, for collaboration and consumption by other tools
    Toml,
}
impl ToString for Format {
    fn to_string(&self) -> String {
        match self {
            Format::Toml => "toml",
            Format::Apkg => "apkg",
        }
        .to_string()
    }
}

#[derive(Serialize, Deserialize)]
pub struct Export {
    pub deck_name: String,
    pub notes: Vec<NoteInfo>,
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

pub fn get_filtered_files(
    dir_path: PathBuf,
    format: Format,
) -> Result<Vec<DirEntry>, Box<dyn Error>> {
    let read_dir = fs::read_dir(dir_path)?;

    let mut files = read_dir
        .filter_map(Result::ok)
        .filter(|f: &fs::DirEntry| {
            let file_type = f.file_type();
            let file_name = f.file_name().into_string();

            let is_file = file_type.map(|t| t.is_file()).unwrap_or(false);
            let is_ext = file_name
                .map(|n| {
                    n.ends_with(match format {
                        Format::Apkg => ".apkg",
                        Format::Toml => ".toml",
                    })
                })
                .unwrap_or(false);

            is_file && is_ext
        })
        .collect::<Vec<_>>();

    files.sort_by(|a, b| {
        let a = a.file_name().to_ascii_lowercase();
        let b = b.file_name().to_ascii_lowercase();
        a.cmp(&b)
    });

    Ok(files)
}
