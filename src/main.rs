mod ankiconnect;

use std::{collections::HashMap, error::Error, ffi::OsString, fs, path::PathBuf, process::exit};

use ankiconnect::{AnkiRequest, AnkiResponse, ExportPackageParams, ImportPackageParams};
use clap::{Parser, Subcommand};
use dialoguer::{theme::ColorfulTheme, Select};
use owo_colors::OwoColorize;

fn main() {
    let args = Args::parse();

    let result = match args.cmd {
        Command::Import { path: file } => import_file(file),
        Command::Export { dir: file, deck } => export_file(file, deck),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "error:".bold().red(), e);
        exit(1);
    }
}

fn import_file(path: String) -> Result<(), Box<dyn Error>> {
    let client = ankiconnect::AnkiClient::new();

    let path = PathBuf::from(&path);

    let file = if path.is_file() {
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
                let is_ext = file_name.map(|n| n.ends_with(".apkg")).unwrap_or(false);

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
            .map(fs::DirEntry::file_name)
            .map(OsString::into_string)
            .map(Result::unwrap)
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

    if !PathBuf::from(&file).is_file() {
        return Err(format!("{} is not a file", file.display()).to_owned())?;
    }

    let path = file.to_str().unwrap().to_owned();

    let response: AnkiResponse<bool> = client.request(&AnkiRequest {
        action: "importPackage".to_string(),
        version: 6,
        params: Some(ImportPackageParams { path: path.clone() }),
    })?;

    match response.result {
        Some(true) => {
            println!("Imported {}", path.green());
            Ok(())
        }
        Some(false) | None => Err("could not import file".to_owned())?,
    }
}

/// Export a deck to a file
/// * `dir` - The directory to export to
fn export_file(dir: Option<String>, deck_name: Option<String>) -> Result<(), Box<dyn Error>> {
    let client = ankiconnect::AnkiClient::new();

    let dir = match dir {
        Some(dir) => PathBuf::from(dir),
        None => {
            let home =
                home::home_dir().ok_or("Could not find home directory. Specify a directory.")?;
            home.join(".anki")
        }
    };
    if !dir.is_dir() {
        return Err(format!("{} is not a directory", dir.display()).to_owned())?;
    }

    let deck = match deck_name {
        Some(deck) => deck,
        None => {
            let res: AnkiResponse<HashMap<String, u64>> = client.request(&AnkiRequest::<()> {
                action: "deckNamesAndIds".to_string(),
                version: 6,
                params: None,
            })?;

            let res = res.result.ok_or("Could not get decks")?;

            let mut deck_names = res.keys().collect::<Vec<&String>>();
            if deck_names.is_empty() {
                return Err("No decks found".to_owned())?;
            }

            deck_names.sort();

            let selection = Select::with_theme(&ColorfulTheme::default())
                .items(&deck_names)
                .default(0)
                .with_prompt("Select a deck")
                .report(false)
                .interact()
                .unwrap();

            deck_names[selection].to_owned()
        }
    };

    let file_name = dir.join(&deck).with_extension("apkg");

    let params = ExportPackageParams {
        deck_name: deck,
        path: file_name.to_str().unwrap().to_owned(),
        include_scheduling: false,
    };

    let response: AnkiResponse<bool> = client.request(&AnkiRequest {
        action: "exportPackage".to_string(),
        version: 6,
        params: Some(params),
    })?;

    match response.result {
        Some(true) => Ok(()),
        Some(false) => Err(response.error.unwrap_or("Unknown".to_string()))?,
        None => Err("no result")?,
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
    },
    /// Export a deck to a file
    Export {
        /// The dir in which to export
        /// Defaults to ~/.anki (or C:\Users\<user>\.anki on Windows
        dir: Option<String>,
        /// The deck to export.
        /// If not specified, a list of decks from which to choose will be shown.
        deck: Option<String>,
    },
}
