package ankiconnect

import "github.com/VaiTon/ankituls/internal/formats"

// Internal representation of AnkiConnect types

type CardId uint64
type NoteId uint64
type DeckId uint64

type NoteField struct {
	Value string `json:"value"`
	Order uint64 `json:"order"`
}

type NoteInfo struct {
	NoteId    NoteId               `json:"noteId"`
	Tags      []string             `json:"tags"`
	Fields    map[string]NoteField `json:"fields"`
	ModelName string               `json:"modelName"`
}

// ConvertNoteInfoToNote converts an ankiconnect.NoteInfo to a formats.Note.
func ConvertNoteInfoToNote(n NoteInfo) formats.Note {
	fields := make(map[string]formats.NoteField, len(n.Fields))
	for k, v := range n.Fields {
		fields[k] = formats.NoteField{
			Value: v.Value,
			Order: v.Order,
		}
	}
	return formats.Note{
		NoteId:    uint64(n.NoteId),
		Tags:      append([]string{}, n.Tags...),
		Fields:    fields,
		ModelName: n.ModelName,
	}
}

// ConvertNoteInfoSliceToNotes converts a slice of ankiconnect.NoteInfo to []formats.Note.
func ConvertNoteInfoSliceToNotes(notesInfo []NoteInfo) []formats.Note {
	notes := make([]formats.Note, len(notesInfo))
	for i, n := range notesInfo {
		notes[i] = ConvertNoteInfoToNote(n)
	}
	return notes
}

// ConvertDeckToNotes converts a deck name and a slice of NoteInfo to []formats.Note.
func ConvertDeckToNotes(deck string, notesInfo []NoteInfo) []formats.Note {
	notes := make([]formats.Note, len(notesInfo))
	for i, n := range notesInfo {
		fields := make(map[string]formats.NoteField, len(n.Fields))
		for k, v := range n.Fields {
			fields[k] = formats.NoteField{
				Value: v.Value,
				Order: v.Order,
			}
		}
		notes[i] = formats.Note{
			NoteId:    uint64(n.NoteId),
			Tags:      append([]string{}, n.Tags...),
			Fields:    fields,
			ModelName: n.ModelName,
		}
	}
	return notes
}
