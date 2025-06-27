// SPDX-FileCopyrightText: 2025 Eyad Issa <eyadlorenzo@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

package formats

import (
	"bytes"
	"reflect"
	"strings"
	"testing"
)

func sampleExportJSON() *Export {
	return &Export{
		Version:  1,
		DeckName: "Test Deck",
		Notes: []Note{
			{
				NoteId:    1,
				Tags:      []string{"tag1", "tag2"},
				Fields:    map[string]NoteField{"Front": {Value: "Q", Order: 1}, "Back": {Value: "A", Order: 2}},
				ModelName: "Basic",
			},
		},
		Models: []any{"model1"},
	}
}

func TestMarshalJSON_UnmarshalJSON_RoundTrip(t *testing.T) {
	original := sampleExportJSON()
	var buf bytes.Buffer

	if err := MarshalJSON(&buf, original); err != nil {
		t.Fatalf("MarshalJSON failed: %v", err)
	}

	got := &Export{}
	if err := UnmarshalJSON(strings.NewReader(buf.String()), got); err != nil {
		t.Fatalf("UnmarshalJSON failed: %v", err)
	}

	if !reflect.DeepEqual(original, got) {
		t.Errorf("JSON round-trip mismatch.\nOriginal: %+v\nGot: %+v", original, got)
	}
}

func TestMarshalJSON_EmptyExport(t *testing.T) {
	empty := &Export{}
	var buf bytes.Buffer

	if err := MarshalJSON(&buf, empty); err != nil {
		t.Fatalf("MarshalJSON failed: %v", err)
	}

	got := &Export{}
	if err := UnmarshalJSON(strings.NewReader(buf.String()), got); err != nil {
		t.Fatalf("UnmarshalJSON failed: %v", err)
	}

	if !reflect.DeepEqual(empty, got) {
		t.Errorf("Empty Export JSON round-trip mismatch.\nOriginal: %+v\nGot: %+v", empty, got)
	}
}

func TestUnmarshalJSON_Invalid(t *testing.T) {
	invalid := "{not: valid: json}"
	err := UnmarshalJSON(strings.NewReader(invalid), &Export{})
	if err == nil {
		t.Error("expected error for invalid JSON, got nil")
	}
}

func TestUnmarshalJSON_Partial(t *testing.T) {
	// Only DeckName provided, rest should be zero values
	partial := `{"deck_name":"Partial Deck"}`
	got := &Export{}
	if err := UnmarshalJSON(strings.NewReader(partial), got); err != nil {
		t.Fatalf("UnmarshalJSON failed: %v", err)
	}
	if got.DeckName != "Partial Deck" {
		t.Errorf("Expected DeckName 'Partial Deck', got %q", got.DeckName)
	}
	if got.Version != 0 || len(got.Notes) != 0 || len(got.Models) != 0 {
		t.Errorf("Expected zero values for other fields, got: %+v", got)
	}
}

func TestMarshalJSON_DeterministicOrder(t *testing.T) {
	// Notes and fields out of order, should be sorted by MarshalJSON
	e := &Export{
		Version:  1,
		DeckName: "SortTest",
		Notes: []Note{
			{
				NoteId:    2,
				Tags:      []string{"b", "a"},
				Fields:    map[string]NoteField{"z": {Value: "Z", Order: 2}, "a": {Value: "A", Order: 1}},
				ModelName: "Basic",
			},
			{
				NoteId:    1,
				Tags:      []string{"d", "c"},
				Fields:    map[string]NoteField{"y": {Value: "Y", Order: 2}, "b": {Value: "B", Order: 1}},
				ModelName: "Basic",
			},
		},
		Models: []any{},
	}
	var buf bytes.Buffer
	if err := MarshalJSON(&buf, e); err != nil {
		t.Fatalf("MarshalJSON failed: %v", err)
	}
	// Unmarshal and check order
	got := &Export{}
	if err := UnmarshalJSON(strings.NewReader(buf.String()), got); err != nil {
		t.Fatalf("UnmarshalJSON failed: %v", err)
	}
	if got.Notes[0].NoteId != 1 || got.Notes[1].NoteId != 2 {
		t.Errorf("Notes not sorted by NoteId: %+v", got.Notes)
	}
	for _, note := range got.Notes {
		if !sortStringsIsSorted(note.Tags) {
			t.Errorf("Tags not sorted: %+v", note.Tags)
		}
	}
}

// Helper for checking string slice order
func sortStringsIsSorted(s []string) bool {
	for i := 1; i < len(s); i++ {
		if s[i-1] > s[i] {
			return false
		}
	}
	return true
}
