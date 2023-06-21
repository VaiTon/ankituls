use std::{error::Error, fs, path::Path};

use ankiconnect::{
    AddNotesRequest, AddNotesResponse, AnkiClient, AnkiResponse, CreateNote, ImportPackageRequest,
    ImportPackageResponse,
};

use crate::Export;

pub fn import_apkg(client: &AnkiClient, path: &Path) -> Result<(), Box<dyn Error>> {
    let path = path.to_str().ok_or("invalid path")?.to_owned();

    let response = client.request(ImportPackageRequest { path })?;

    match response {
        ImportPackageResponse(true) => Ok(()),
        ImportPackageResponse(false) => Err("could not import file")?,
    }
}

pub struct ImportResult {
    pub imported_notes: usize,
    pub total_notes: usize,
}

pub fn import_toml(client: &AnkiClient, file_path: &Path) -> Result<ImportResult, Box<dyn Error>> {
    let notes = fs::read_to_string(file_path)?;
    let export: Export = toml::from_str(&notes)?;

    let notes: Vec<CreateNote> = export.into();
    let notes_len = notes.len();

    let response: AnkiResponse<AddNotesResponse> = client.request(AddNotesRequest { notes })?;

    match response.result {
        Some(AddNotesResponse(imported_notes)) => {
            let imports_ok = imported_notes.iter().filter_map(|n| n.as_ref()).count();

            Ok(ImportResult {
                imported_notes: imports_ok,
                total_notes: notes_len,
            })
        }
        None => Err(format!(
            "could not import file: {}",
            response.error.unwrap_or("unknown error".to_string())
        ))?,
    }
}
