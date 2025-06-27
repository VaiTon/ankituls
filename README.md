# Ankitu

Ankitu is a tool to import and export Anki decks.
It is written in Rust and uses the [AnkiConnect](https://github.com/FooSoft/anki-connect) API.

It can be used to import / export decks from / to Anki:

| Format | Import  | Export  | Notes                                             |
| ------ | ------- | ------- | ------------------------------------------------- |
| TOML   | :check: | :check: | Recommended for git versioning. See schema below. |
| JSON   |         |         | TODO                                              |
| YAML   |         |         | TODO                                              |

> **Note:** Ankitu is still in development and not all features are implemented yet.

## TOML Export Schema & Versioning

TOML exports are designed to be stable and versionable for long-term use and git tracking.

- Each export includes a `version` field at the top level.
- Notes, fields, and tags are sorted deterministically for reproducible output.
- The schema is documented below. If the schema changes, the version will be incremented and migration tools will be provided.

### Example

```toml
# ankituls export schema version = 1
version = 1
deck_name = "My Deck"

[[notes]]
noteId = 123456789
modelName = "Basic"
tags = ["tag1", "tag2"]

[notes.fields]
Front = { value = "Front content", order = 0 }
Back = { value = "Back content", order = 1 }
```

### Top-level fields

- `version` (int): Schema version of the export file.
- `deck_name` (string): Name of the deck.
- `notes` (array): List of notes.

### Note fields

- `noteId` (int): Unique note identifier.
- `modelName` (string): Name of the note model.
- `tags` (array of string): Tags for the note (sorted).
- `fields` (table): Field name to value mapping (sorted by field name).

---

## Installation

```bash
cargo install --git https://github.com/VaiTon/ankitu.git
```

## Usage

Export/import a deck:

```bash
ankitu export <DECK>   # print TOML export of deck to stdout

ankitu import <FOLDER> # import deck from TOML
```
