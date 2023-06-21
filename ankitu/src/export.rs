use std::{error::Error, fs, path::Path};

use ankiconnect::{
    AnkiClient, ExportPackageRequest, ExportPackageResponse, FindNotesRequest, FindNotesResponse,
    NotesInfoRequest, NotesInfoResponse,
};

use crate::Export;

pub fn export_apkg(
    client: &AnkiClient,
    deck_name: &str,
    path: &Path,
) -> Result<(), Box<dyn Error>> {
    let path = path.to_str().ok_or("invalid path")?.to_owned();

    let response = client.request(ExportPackageRequest {
        deck: deck_name.to_owned(),
        include_scheduling: false,
        path,
    })?;

    match response {
        Some(ExportPackageResponse(true)) => Ok(()),
        Some(ExportPackageResponse(false)) | None => Err("could not export file")?,
    }
}

pub fn export_toml(
    client: &AnkiClient,
    deck_name: &str,
    path: &Path,
) -> Result<(), Box<dyn Error>> {
    let FindNotesResponse(ids) = client.request(FindNotesRequest {
        query: format!("\"deck:{}\"", deck_name),
    })?;

    let NotesInfoResponse(mut cards) = client.request(NotesInfoRequest { notes: ids })?;

    // Sort by card id, so that for version control systems, the diff is easier to read
    cards.sort_by_cached_key(|c| c.note_id);

    let export = Export {
        deck_name: deck_name.to_owned(),
        notes: cards,
    };
    fs::write(path, toml::to_string(&export)?)?;
    Ok(())
}
