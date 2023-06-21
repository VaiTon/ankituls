use std::{
    error::Error,
    fs,
    path::{Path, PathBuf},
    process::exit,
};

use ankiconnect::{
    requests::{DeckNamesRequest, DeckNamesResponse},
    *,
};
use clap::{Parser, Subcommand, ValueEnum};
use dialoguer::{theme::ColorfulTheme, Select};
use owo_colors::OwoColorize;

use ankitu::{export_format, import_apkg, import_toml, Export, ExportFormat, ImportResult};

mod export;

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
        path: String,
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
        format: ExportFormat,
    },
    /// Import and export a deck to a file, effectively syncing it
    Sync {
        /// The dir in which to export
        /// Defaults to ~/.anki (or C:\Users\<user>\.anki on Windows)
        dir: Option<String>,
        /// The deck to sync.
        /// If not specified, a list of decks from which to choose will be shown.
        deck: Option<String>,

        #[arg(short, long)]
        all: bool,

        #[arg(short, long, default_value = "apkg")]
        format: ExportFormat,
    },
}
fn main() {
    let args = Args::parse();

    let result = match args.cmd {
        Command::Import { path } => {
            let path = PathBuf::from(&path);

            if path.is_file() {
                import_file(&path)
            } else {
                import_from_dir(&path)
            }
        }
        Command::Export { dir, deck, format } => {
            let dir = to_dir_or_default(dir).expect("failed to get dir");
            export_file(&dir, deck, format).map(|_| ())
        }
        Command::Sync {
            dir,
            deck,
            format,
            all,
        } => {
            let dir = to_dir_or_default(dir).expect("failed to get dir");
            if all {
                sync_all_decks(&dir, format)
            } else {
                sync_one_deck(&dir, deck, format)
            }
        }
    };

    if let Err(e) = result {
        eprintln!("{} {}", "error:".bold().red(), e);
        exit(1);
    }
}

fn import_from_dir(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let dir = fs::read_dir(path)?;

    let allowed_extensions: Vec<_> = ExportFormat::value_variants()
        .iter()
        .map(|f| f.to_string())
        .collect();

    let file_names: Vec<String> = dir
        .filter_map(|p| {
            let p = p.ok()?;
            let name = p.file_name().to_str()?.to_owned();

            let p_type = p.file_type().ok()?;
            let extension = p.path().extension()?.to_os_string().into_string().ok()?;

            if p_type.is_file() && allowed_extensions.contains(&extension) {
                Some(name)
            } else {
                None
            }
        })
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Select a file to import")
        .default(0)
        .items(&file_names)
        .interact()?;

    let file = &file_names[selection];

    let file = path.join(file);

    import_file(&file)
}

fn sync_one_deck(
    dir: &Path,
    deck_name: Option<String>,
    format: ExportFormat,
) -> Result<(), Box<dyn Error>> {
    let client = AnkiClient::new();

    let deck_name = match deck_name {
        Some(deck_name) => deck_name,
        None => select_deck_name_from_anki(&client)?,
    };

    println!("{} {}", "Syncing".green(), &deck_name);

    {
        let dir: &Path = &dir;
        let client = &client;
        let path = dir
            .join(deck_name.clone())
            .with_extension(format.to_string());

        sync_file(&path, format, client, &deck_name)
    }
}

fn sync_all_decks(dir: &Path, format: ExportFormat) -> Result<(), Box<dyn Error>> {
    let client = AnkiClient::new();

    // Get all deck names from Anki
    let DeckNamesResponse(deck_names) = client.request(DeckNamesRequest)?;

    println!("{} {} decks", "Syncing".green(), deck_names.len());

    let extension = format.to_string();

    for deck_name in deck_names {
        let file_path = dir.join(&deck_name).with_extension(&extension);
        sync_file(&file_path, format, &client, &deck_name)?;
    }

    Ok(())
}

fn sync_file(
    path: &Path,
    format: ExportFormat,
    client: &AnkiClient,
    deck_name: &str,
) -> Result<(), Box<dyn Error>> {
    // import first to ensure we don't overwrite any changes
    if path.exists() {
        import_file(path)?;
    }

    // export after to ensure we have the latest changes
    export_format(format, client, deck_name, path)?;
    println!("{} {}", "Synced".green(), deck_name);
    Ok(())
}

fn import_file(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let client = AnkiClient::new();

    let canonical_path = fs::canonicalize(file_path)?;
    let format = match file_path.extension() {
        Some(ext) if ext == "toml" => ExportFormat::Toml,
        Some(ext) if ext == "apkg" => ExportFormat::Apkg,
        _ => Err(format!(
            "file extension not supported: {}",
            file_path.display()
        ))?,
    };

    let result = match format {
        ExportFormat::Toml => Some(import_toml(&client, &canonical_path)?),
        ExportFormat::Apkg => {
            import_apkg(&client, &canonical_path)?;
            None
        }
    };

    match result {
        Some(ImportResult {
            imported_notes,
            total_notes,
        }) => {
            println!(
                "{} {} notes out of {} total notes",
                "Imported".green(),
                imported_notes,
                total_notes
            )
        }
        None => println!("{} {}", "Imported".green(), canonical_path.display()),
    }

    Ok(())
}

/// Export a deck to a file
/// * `dir` - The directory to export to
fn export_file(
    dir: &Path,
    deck_name: Option<String>,
    format: ExportFormat,
) -> Result<PathBuf, Box<dyn Error>> {
    let client = AnkiClient::new();

    let deck_name = match deck_name {
        Some(deck) => deck,
        None => select_deck_name_from_anki(&client)?,
    };

    let export_file_path = dir.join(&deck_name);

    println!("{} deck '{}'...", "Exporting".green(), deck_name);

    export_format(format, &client, &deck_name, &export_file_path)?;

    println!("{} {}", "Exported".green(), export_file_path.display());
    Ok(export_file_path)
}

fn to_dir_or_default(dir: Option<String>) -> Result<PathBuf, Box<dyn Error>> {
    match dir {
        None => home::home_dir()
            .ok_or(Err("Could not find home directory. Specify a directory.")?)
            .map(|p| p.join(".anki")),

        Some(dir) => match PathBuf::from(dir) {
            p if p.is_dir() => Ok(p),
            p => Err(format!("{} is not a directory", p.display()))?,
        },
    }
}

fn select_deck_name_from_anki(client: &AnkiClient) -> Result<String, Box<dyn Error>> {
    let res = client.request(DeckNamesRequest)?;
    let DeckNamesResponse(mut deck_names) = res;

    if deck_names.is_empty() {
        return Err("No decks found")?;
    }

    deck_names.sort();
    let selection = Select::with_theme(&ColorfulTheme::default())
        .items(&deck_names)
        .default(0)
        .with_prompt("Select a deck")
        .report(false)
        .interact()?;

    Ok(deck_names[selection].to_owned())
}
