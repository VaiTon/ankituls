// SPDX-FileCopyrightText: 2025 Eyad Issa <eyadlorenzo@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

package ankiconnect

type DeckNamesRequest struct{}

type DeckNamesResponse []string

type FindNotesRequest struct {
	Query string `json:"query"`
}

type FindNotesResponse []uint64

type NotesInfoRequest struct {
	Notes []uint64 `json:"notes"`
}

type NotesInfoResponse []NoteInfo

type CreateDeckRequest struct {
	Deck string `json:"deck"`
}

type AddNotesRequest struct {
	Notes []CreateNote `json:"notes"`
}

type CreateNote struct {
	DeckName  string             `json:"deckName"`
	ModelName string             `json:"modelName"`
	Fields    map[string]string  `json:"fields"`
	Tags      []string           `json:"tags"`
	Options   *CreateNoteOptions `json:"options,omitempty"`
}

type CreateNoteOptions struct {
	AllowDuplicate bool   `json:"allowDuplicate"`
	DuplicateScope string `json:"duplicateScope"`
}

type DeleteDecksRequest struct {
	Decks    []string `json:"decks"`
	CardsToo bool     `json:"cardsToo"`
}
