use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use ankiconnect::{requests::*, AnkiClient, NoteField, NoteId, NoteInfo};
use serde::{Deserialize, Serialize};

use crate::Export;

#[derive(Serialize, Debug, Deserialize)]
struct TomlExport {
    deck_name: String,
    notes: Vec<TomlExportNote>,
}

impl From<Export> for TomlExport {
    fn from(value: Export) -> Self {
        let mut notes: Vec<_> = value
            .notes
            .into_iter()
            .map(|p| {
                let mut fields: Vec<_> = p
                    .fields
                    .into_iter()
                    .map(|(nome, f)| TomlExportNoteField {
                        name: nome,
                        value: f.value,
                        order: f.order,
                    })
                    .collect();

                fields.sort_by_cached_key(|f| f.order);

                TomlExportNote {
                    note_id: p.note_id,
                    tags: p.tags,
                    fields,
                    model_name: p.model_name,
                }
            })
            .collect();

        notes.sort_by_cached_key(|f| f.note_id);

        Self {
            deck_name: value.deck_name,
            notes,
        }
    }
}

impl From<TomlExport> for Export {
    fn from(value: TomlExport) -> Self {
        let notes = value
            .notes
            .into_iter()
            .map(|p| {
                let fields = p
                    .fields
                    .into_iter()
                    .map(|f| {
                        (
                            f.name,
                            NoteField {
                                value: f.value,
                                order: f.order,
                            },
                        )
                    })
                    .collect::<HashMap<String, NoteField>>();

                NoteInfo {
                    note_id: p.note_id,
                    tags: p.tags,
                    fields,
                    model_name: p.model_name,
                }
            })
            .collect();

        Self {
            deck_name: value.deck_name,
            notes,
        }
    }
}

#[derive(Serialize, Debug, Deserialize)]
struct TomlExportNote {
    note_id: NoteId,
    tags: Vec<String>,
    fields: Vec<TomlExportNoteField>,
    model_name: String,
}

#[derive(Serialize, Debug, Deserialize)]
struct TomlExportNoteField {
    name: String,
    value: String,
    order: u64,
}

pub(super) fn export_toml(
    client: &AnkiClient,
    path: &Path,
    deck_name: &str,
) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let path = path.with_extension("toml");

    // Get all deck notes
    let FindNotesResponse(notes) = client.request(FindNotesRequest {
        query: format!("\"deck:{}\"", deck_name),
    })?;

    // Get all notes info
    let NotesInfoResponse(cards) = client.request(NotesInfoRequest { notes })?;

    let export: TomlExport = Export {
        deck_name: deck_name.to_string(),
        notes: cards,
    }
    .into();

    fs::write(&path, toml::to_string(&export)?)?;
    Ok(path)
}

pub(super) fn read(path: &Path) -> Result<Export, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path.with_extension("toml"))?;
    let export: TomlExport = toml::from_str(content.as_str())?;

    Ok(export.into())
}

pub(super) fn import(
    client: &AnkiClient,
    export: Export,
) -> Result<(), Box<dyn std::error::Error>> {
    // Delete the deck
    let _ = client.request(DeleteDecksRequest {
        decks: vec![export.deck_name.clone()],
        cards_too: true,
    });

    // Create the deck
    client.request(CreateDeckRequest {
        deck: export.deck_name.clone(),
    })?;

    // Create the notes
    let notes: Vec<CreateNote> = export.into();
    client.request(AddNotesRequest { notes })?;

    Ok(())
}
