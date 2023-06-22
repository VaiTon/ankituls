use std::{error::Error, path::{Path, PathBuf}};

use ankiconnect::AnkiClient;

use crate::{Export, ExportFormat};

mod toml;

pub fn import(
    format: ExportFormat,
    client: AnkiClient,
    export: Export,
) -> Result<(), Box<dyn Error>> {
    match format {
        ExportFormat::Toml => toml::import(&client, export),
    }
}

pub fn read(format: ExportFormat, path: &Path) -> Result<Export, Box<dyn Error>> {
    match format {
        ExportFormat::Toml => toml::read(path),
    }
}

pub fn export(
    format: ExportFormat,
    client: &AnkiClient,
    deck_name: &str,
    export_file_path: &Path,
) -> Result<PathBuf, Box<dyn Error>> {
    match format {
        ExportFormat::Toml => toml::export_toml(client, export_file_path, deck_name),
    }
}
