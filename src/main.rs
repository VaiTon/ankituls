use std::{
    collections::HashMap,
    error::Error,
    ffi::OsString,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
    process::exit,
};

use ankiconnect::*;
use clap::{Parser, Subcommand, ValueEnum};
use dialoguer::{theme::ColorfulTheme, Select};
use owo_colors::OwoColorize;

use crate::export::Export;

mod export;

fn main() {
    let args = Args::parse();

    let result = match args.cmd {
        Command::Import { path, format } => import_file(path, format),
        Command::Export {
            dir: file,
            deck,
            format,
        } => export_file(file, deck, format),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "error:".bold().red(), e);
        exit(1);
    }
}

fn import_file(path: String, format: Format) -> Result<(), Box<dyn Error>> {
    let client = AnkiClient::new();

    let path = PathBuf::from(&path);

    let file_path = if path.is_file() {
        path
    } else {
        let read_dir = fs::read_dir(path.clone()).or(Err(format!(
            "directory not found: {}",
            path.display().bold()
        )))?;

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

        let file_names = files
            .iter()
            .map(DirEntry::file_name)
            .map(OsString::into_string)
            .filter_map(Result::ok)
            .collect::<Vec<_>>();

        let file = Select::with_theme(&ColorfulTheme::default())
            .default(0)
            .items(&file_names)
            .with_prompt("File to import")
            .report(false)
            .interact()
            .unwrap();

        files[file].path()
    };

    if !file_path.is_file() {
        return Err(format!("{} is not a file", file_path.display()))?;
    }

    let canonical_path = fs::canonicalize(file_path.clone())?;
    let canonical_path_str = canonical_path.to_str().ok_or("invalid path")?;

    match format {
        Format::Toml => import_toml(&client, &file_path),
        Format::Apkg => import_apkg(&client, &file_path, &canonical_path_str),
    }
}

fn import_toml(client: &AnkiClient, file_path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let notes = fs::read_to_string(file_path.clone())?;
    let export: Export = toml::from_str(&notes)?;

    println!(
        "{} {} as {}",
        "Importing".green(),
        file_path.display(),
        &export.deck_name
    );

    let notes: Vec<CreateNote> = export
        .cards
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
                deck_name: export.deck_name.clone(),
                model_name: info.model_name,
                fields: fields_values,
                tags: info.tags,
                options: Some(options),
            }
        })
        .collect();

    let notes_len = notes.len();

    let response: AnkiResponse<AddNotesResponse> = client.request(AddNotesRequest { notes })?;

    match response.result {
        Some(AddNotesResponse(imported_notes)) => {
            let imports_ok = imported_notes.iter().filter_map(|n| n.as_ref()).count();

            println!("{} {}/{} notes", "Imported".green(), imports_ok, notes_len);

            Ok(())
        }
        None => Err(format!(
            "could not import file: {}",
            response.error.unwrap_or("unknown error".to_string())
        ))?,
    }
}

fn import_apkg(
    client: &AnkiClient,
    file_path: &Path,
    canonical_path_str: &str,
) -> Result<(), Box<dyn Error>> {
    match client.request(ImportPackageRequest {
        path: canonical_path_str.to_owned(),
    })? {
        ImportPackageResponse(true) => Ok(()),
        ImportPackageResponse(false) => {
            Err(format!("could not import file: {}", file_path.display()))?
        }
    }
}

/// Export a deck to a file
/// * `dir` - The directory to export to
fn export_file(
    dir: Option<String>,
    deck_name: Option<String>,
    format: Format,
) -> Result<(), Box<dyn Error>> {
    let client = AnkiClient::new();

    let dir = match dir {
        Some(dir) => PathBuf::from(dir),
        None => home::home_dir()
            .ok_or("Could not find home directory. Specify a directory.")?
            .join(".anki"),
    };

    if !dir.is_dir() {
        return Err(format!("{} is not a directory", dir.display()))?;
    }

    let deck_name = match deck_name {
        Some(deck) => deck,
        None => {
            let res = client.request(DeckNamesRequest)?;

            let DeckNamesResponse(mut deck_names) = res;
            if deck_names.is_empty() {
                return Err("No decks found".to_owned())?;
            }

            deck_names.sort();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .items(&deck_names)
                .default(0)
                .with_prompt("Select a deck")
                .report(false)
                .interact()?;

            deck_names[selection].to_owned()
        }
    };

    println!("{} deck '{}'...", "Exporting".green(), deck_name);

    let export_file_path = dir.join(&deck_name);

    match format {
        Format::Apkg => export_anki2(&client, &deck_name, &export_file_path),
        Format::Toml => export_toml(&client, &deck_name, &export_file_path),
    }
}
fn export_toml(
    client: &AnkiClient,
    deck_name: &String,
    export_file_path: &Path,
) -> Result<(), Box<dyn Error>> {
    let FindNotesResponse(ids) = client.request(FindNotesRequest {
        query: format!("\"deck:{}\"", deck_name),
    })?;

    println!("{} {} cards...", "Exporting".green(), ids.len());

    let NotesInfoResponse(mut cards) = client.request(NotesInfoRequest { notes: ids })?;

    let export_file_path = export_file_path.with_extension("toml");

    // Sort by card id, so that for version control systems, the diff is easier to read
    cards.sort_by_cached_key(|c| c.note_id);

    let export = export::Export {
        deck_name: deck_name.to_owned(),
        cards,
    };
    fs::write(export_file_path.clone(), toml::to_string(&export)?)?;

    println!("{} to '{}'", "Exported".green(), export_file_path.display());
    Ok(())
}

fn export_anki2(
    client: &AnkiClient,
    deck_name: &str,
    export_file_path: &Path,
) -> Result<(), Box<dyn Error>> {
    match client.request(ExportPackageRequest {
        deck: deck_name.to_string(),
        include_scheduling: false,
        path: export_file_path
            .with_extension("apkg")
            .to_str()
            .expect("invalid path")
            .to_owned(),
    })? {
        Some(ExportPackageResponse(true)) => {
            println!("{} to '{}'", "Exported".green(), export_file_path.display());
            Ok(())
        }
        Some(ExportPackageResponse(false)) | None => Err("could not export file")?,
    }
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Import a file
    Import {
        /// The path from which to import.
        /// If the path refers to a directory, a list of files from which to choose will be shown.
        path: String,

        #[arg(short, long, default_value = "apkg")]
        format: Format,
    },
    /// Export a deck to a file
    Export {
        /// The dir in which to export
        /// Defaults to ~/.anki (or C:\Users\<user>\.anki on Windows)
        dir: Option<String>,
        /// The deck to export.
        /// If not specified, a list of decks from which to choose will be shown.
        deck: Option<String>,

        #[arg(short, long, default_value = "apkg")]
        format: Format,
    },
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Format {
    /// Anki package
    Apkg,

    /// TOML representation of the deck, for collaboration and consumption by other tools
    Toml,
}
