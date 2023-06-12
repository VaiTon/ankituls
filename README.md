# Ankitu

Ankitu is a tool to import and export Anki decks.
It is written in Rust and uses the [AnkiConnect](https://github.com/FooSoft/anki-connect) API.

It can be used to import / export decks from / to Anki:

- [x] Export decks to TOML (recommended for git versioning)
- [x] Import decks from TOML (recommended for git versioning)
- [x] Export decks to APKG
- [x] Import decks from APKG

> **Note:** Ankitu is still in development and not all features are implemented yet.

## Installation

```bash
cargo install --git https://github.com/VaiTon/ankitu.git
```

## Usage

Export/import a deck:

```bash
ankitu export <FOLDER> # export deck to APKG
ankitu export <FOLDER> --format toml # export deck to TOML

ankitu import <FOLDER> # import deck from APKG
ankitu import <FOLDER> --format toml # import deck from TOML
```
