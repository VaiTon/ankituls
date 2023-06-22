use std::{
    fs,
    path::{Path, PathBuf},
};

use ankiconnect::{requests::*, AnkiClient};

use crate::Export;

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
    let NotesInfoResponse(mut cards) = client.request(NotesInfoRequest { notes })?;
    // Sort by card id, so that for version control systems, the diff is easier to read
    cards.sort_by_cached_key(|c| c.note_id.0);

    let export = Export {
        deck_name: deck_name.to_owned(),
        notes: cards,
    };

    fs::write(&path, toml::to_string(&export)?)?;
    Ok(path)
}

pub(super) fn read(path: &Path) -> Result<Export, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path.with_extension("toml"))?;
    let export: Export = toml::from_str(content.as_str())?;
    Ok(export)
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
