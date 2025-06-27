package formats

import (
	"errors"
	"io"
	"sort"
)

// NoteField represents a field in a note.
type NoteField struct {
	Value string `json:"value" toml:"value" yaml:"value"`
	Order uint64 `json:"order" toml:"order" yaml:"order"`
}

// Note represents a single note.
type Note struct {
	NoteId    uint64               `json:"noteId" toml:"noteId" yaml:"noteId"`
	Tags      []string             `json:"tags" toml:"tags" yaml:"tags"`
	Fields    map[string]NoteField `json:"fields" toml:"fields" yaml:"fields"`
	ModelName string               `json:"modelName" toml:"modelName" yaml:"modelName"`
}

// Export is the top-level structure for exporting decks.
type Export struct {
	Version  int    `json:"version" toml:"version" yaml:"version"`
	DeckName string `json:"deck_name" toml:"deck_name" yaml:"deck_name"`
	Notes    []Note `json:"notes" toml:"notes" yaml:"notes"`
}

// sortForExport sorts notes, tags, and fields for deterministic output.
func (e *Export) sortForExport() {
	sort.Slice(e.Notes, func(i, j int) bool {
		return e.Notes[i].NoteId < e.Notes[j].NoteId
	})
	for i := range e.Notes {
		sort.Strings(e.Notes[i].Tags)
		// Sort fields by key and reassign to a new map to ensure deterministic order
		fields := e.Notes[i].Fields
		keys := make([]string, 0, len(fields))
		for k := range fields {
			keys = append(keys, k)
		}
		sort.Strings(keys)
		sortedFields := make(map[string]NoteField, len(fields))
		for _, k := range keys {
			sortedFields[k] = fields[k]
		}
		e.Notes[i].Fields = sortedFields
	}
}

// ToExport constructs an Export struct from raw data.
func ToExport(version int, deckName string, notes []Note) *Export {
	return &Export{
		Version:  version,
		DeckName: deckName,
		Notes:    notes,
	}
}

// MarshalToWriter marshals Export to the given format ("toml", "json", "yaml") and writes to the writer.
func MarshalToWriter(format string, w io.Writer, e *Export) error {
	switch format {
	case "toml":
		return MarshalTOML(w, e)
	case "json":
		return MarshalJSON(w, e)
	case "yaml", "yml":
		return MarshalYAML(w, e)
	default:
		return errors.New("unsupported format: " + format)
	}
}

// UnmarshalFromReader unmarshals Export from the given format ("toml", "json", "yaml") from the reader.
func UnmarshalFromReader(format string, r io.Reader, e *Export) error {
	switch format {
	case "toml":
		return UnmarshalTOML(r, e)
	case "json":
		return UnmarshalJSON(r, e)
	case "yaml", "yml":
		return UnmarshalYAML(r, e)
	default:
		return errors.New("unsupported format: " + format)
	}
}
