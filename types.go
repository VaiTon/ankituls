// SPDX-FileCopyrightText: 2025 Eyad Issa <eyadlorenzo@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

package ankituls

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

type Export struct {
	Version  int        `json:"version" toml:"version"`
	DeckName string     `json:"deck_name" toml:"deck_name"`
	Notes    []NoteInfo `json:"notes" toml:"notes"`
}

type ExportCardMod struct {
	CardId CardId `json:"card_id"`
	Time   uint32 `json:"time"`
}

type ExportFormat string

const (
	ExportFormatToml ExportFormat = "toml"
)
