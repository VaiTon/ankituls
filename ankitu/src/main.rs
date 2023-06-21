use std::{
    collections::HashMap,
    error::Error,
    fs::{self},
    path::{Path, PathBuf},
    process::exit,
};

use ankiconnect::*;
use ankitu::{export_apkg, export_toml, import_apkg, import_toml, Export, Format, ImportResult};
use clap::{Parser, Subcommand, ValueEnum};
use dialoguer::{theme::ColorfulTheme, Select};
use owo_colors::OwoColorize;

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
        format: Format,
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
        format: Format,
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
        Command::Export {
            dir: file,
            deck,
            format,
        } => export_file(file, deck, format).map(|_| ()),
        Command::Sync {
            dir,
            deck,
            format,
            all: false,
        } => sync_one_deck(dir, deck, format),
        Command::Sync {
            dir,
            deck: _,
            format,
            all: true,
        } => sync_all_decks(dir, format),
    };

    if let Err(e) = result {
        eprintln!("{} {}", "error:".bold().red(), e);
        exit(1);
    }
}

fn import_from_dir(path: &PathBuf) -> Result<(), Box<dyn Error>> {
    let dir = fs::read_dir(path)?;

    let allowed_extensions: Vec<_> = Format::value_variants()
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
    dir: Option<String>,
    deck_name: Option<String>,
    format: Format,
) -> Result<(), Box<dyn Error>> {
    let client = AnkiClient::new();

    let deck_name = match deck_name {
        Some(deck_name) => deck_name,
        None => select_deck_name_from_anki(&client)?,
    };

    println!("{} {}", "Syncing".green(), &deck_name);
    let dir = export_dir_or_default(dir)?;

    sync_impl(deck_name, &dir, format, &client)
}

fn sync_all_decks(dir: Option<String>, format: Format) -> Result<(), Box<dyn Error>> {
    let client = AnkiClient::new();

    let DeckNamesResponse(deck_names) = client.request(DeckNamesRequest)?;

    println!("{} all decks", "Syncing".green());

    let dir = export_dir_or_default(dir)?;

    for deck_name in deck_names {
        sync_impl(deck_name, &dir, format, &client)?;
    }

    Ok(())
}

fn sync_impl(
    deck_name: String,
    dir: &Path,
    format: Format,
    client: &AnkiClient,
) -> Result<(), Box<dyn Error>> {
    let path = dir
        .join(deck_name.clone())
        .with_extension(format.to_string());

    // import first to ensure we don't overwrite any changes
    if path.is_file() {
        import_file(&path)?;
    }

    // export after to ensure we have the latest changes
    export_format(format, client, &deck_name, &path)?;
    println!("{} {}", "Synced".green(), &deck_name);
    Ok(())
}

fn import_file(file_path: &Path) -> Result<(), Box<dyn Error>> {
    let client = AnkiClient::new();

    let canonical_path = fs::canonicalize(file_path)?;
    let format = match file_path.extension() {
        Some(ext) if ext == "toml" => Format::Toml,
        Some(ext) if ext == "apkg" => Format::Apkg,
        _ => Err(format!(
            "file extension not supported: {}",
            file_path.display()
        ))?,
    };

    let result = match format {
        Format::Toml => Some(import_toml(&client, &canonical_path)?),
        Format::Apkg => {
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
    dir: Option<String>,
    deck_name: Option<String>,
    format: Format,
) -> Result<PathBuf, Box<dyn Error>> {
    let client = AnkiClient::new();

    let dir = export_dir_or_default(dir)?;

    let deck_name = match deck_name {
        Some(deck) => deck,
        None => select_deck_name_from_anki(&client)?,
    };

    println!("{} deck '{}'...", "Exporting".green(), deck_name);
    let export_file_path = dir.join(&deck_name);

    export_format(format, &client, &deck_name, &export_file_path)?;

    println!("{} {}", "Exported".green(), export_file_path.display());
    Ok(export_file_path)
}

fn export_dir_or_default(dir: Option<String>) -> Result<PathBuf, Box<dyn Error>> {
    // TODO: collapse with dir.join
    let dir = match dir {
        Some(dir) => PathBuf::from(dir),
        None => home::home_dir()
            .ok_or("Could not find home directory. Specify a directory.")?
            .join(".anki"),
    };
    if !dir.is_dir() {
        return Err(format!("{} is not a directory", dir.display()))?;
    }
    Ok(dir)
}

fn export_format(
    format: Format,
    client: &AnkiClient,
    deck_name: &str,
    export_file_path: &Path,
) -> Result<(), Box<dyn Error>> {
    match format {
        Format::Toml => export_toml(client, deck_name, export_file_path),
        Format::Apkg => export_apkg(client, deck_name, export_file_path),
    }
}

fn select_deck_name_from_anki(client: &AnkiClient) -> Result<String, Box<dyn Error>> {
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

    Ok(deck_names[selection].to_owned())
}
