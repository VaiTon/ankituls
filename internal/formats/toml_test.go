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

func sampleExportTOML() *Export {
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

func TestMarshalTOML_RoundTrip(t *testing.T) {
	original := sampleExportTOML()
	var buf bytes.Buffer

	if err := MarshalTOML(&buf, original); err != nil {
		t.Fatalf("MarshalTOML failed: %v", err)
	}

	got := &Export{}
	if err := UnmarshalTOML(strings.NewReader(buf.String()), got); err != nil {
		t.Fatalf("UnmarshalTOML failed: %v", err)
	}

	if !reflect.DeepEqual(original, got) {
		t.Errorf("TOML round-trip mismatch.\nOriginal: %+v\nGot: %+v", original, got)
	}
}

func TestUnmarshalTOML_Invalid(t *testing.T) {
	invalid := "not = 'valid = toml"
	err := UnmarshalTOML(strings.NewReader(invalid), &Export{})
	if err == nil {
		t.Error("expected error for invalid TOML, got nil")
	}
}

func TestMarshalTOML_EmptyExport(t *testing.T) {
	empty := &Export{
		Notes:  []Note{},
		Models: []any{},
	}
	var buf bytes.Buffer
	if err := MarshalTOML(&buf, empty); err != nil {
		t.Fatalf("MarshalTOML failed for empty Export: %v", err)
	}
	got := &Export{}
	if err := UnmarshalTOML(strings.NewReader(buf.String()), got); err != nil {
		t.Fatalf("UnmarshalTOML failed for empty Export: %v", err)
	}
	if !reflect.DeepEqual(empty, got) {
		t.Errorf("TOML empty Export round-trip mismatch.\nOriginal: %+v\nGot: %+v", empty, got)
		t.Logf("Original Notes nil? %v, len=%d", empty.Notes == nil, len(empty.Notes))
		t.Logf("Got Notes nil? %v, len=%d", got.Notes == nil, len(got.Notes))
		t.Logf("Original Models nil? %v, len=%d", empty.Models == nil, len(empty.Models))
		t.Logf("Got Models nil? %v, len=%d", got.Models == nil, len(got.Models))
	}
}
