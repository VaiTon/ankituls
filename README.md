# Ankitu

Ankitu is a tool to import and export Anki decks.
It is written in Rust and uses the [AnkiConnect](https://github.com/FooSoft/anki-connect) API.

It can be used to import / export decks from / to Anki:

| Format | Import | Export | Notes |
| ------ | ------ | ------ | ----- |
| APKG   | ✔️ | ✔️ | Native file type. You can also use the Anki GUI. |
| TOML   | ✔️ | ✔️ | Recommended for git versioning. |
| JSON   | |  | TODO |


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
