package formats

import (
	"bytes"
	"reflect"
	"strings"
	"testing"
)

func sampleExportYAML() *Export {
	return &Export{
		Version:  1,
		DeckName: "YAML Deck",
		Notes: []Note{
			{
				NoteId:    42,
				Tags:      []string{"yaml", "test"},
				Fields:    map[string]NoteField{"Front": {Value: "Q", Order: 1}, "Back": {Value: "A", Order: 2}},
				ModelName: "Basic",
			},
		},
		Models: []any{"model-yaml"},
	}
}

func TestMarshalUnmarshalYAML(t *testing.T) {
	original := sampleExportYAML()
	var buf bytes.Buffer

	if err := MarshalYAML(&buf, original); err != nil {
		t.Fatalf("MarshalYAML failed: %v", err)
	}

	got := &Export{}
	if err := UnmarshalYAML(strings.NewReader(buf.String()), got); err != nil {
		t.Fatalf("UnmarshalYAML failed: %v", err)
	}

	if !reflect.DeepEqual(original, got) {
		t.Errorf("YAML round-trip mismatch.\nOriginal: %+v\nGot: %+v", original, got)
	}
}

func TestUnmarshalYAML_Invalid(t *testing.T) {
	invalid := "not: valid: yaml: ["
	err := UnmarshalYAML(strings.NewReader(invalid), &Export{})
	if err == nil {
		t.Error("expected error for invalid YAML, got nil")
	}
}

func TestMarshalYAML_EmptyExport(t *testing.T) {
	empty := &Export{
		Notes:  []Note{},
		Models: []any{},
	}
	var buf bytes.Buffer
	if err := MarshalYAML(&buf, empty); err != nil {
		t.Fatalf("MarshalYAML failed for empty Export: %v", err)
	}
	got := &Export{}
	if err := UnmarshalYAML(strings.NewReader(buf.String()), got); err != nil {
		t.Fatalf("UnmarshalYAML failed for empty Export: %v", err)
	}
	if !reflect.DeepEqual(empty, got) {
		t.Errorf("YAML empty Export round-trip mismatch.\nOriginal: %+v\nGot: %+v", empty, got)
		t.Logf("Original Notes nil? %v, len=%d", empty.Notes == nil, len(empty.Notes))
		t.Logf("Got Notes nil? %v, len=%d", got.Notes == nil, len(got.Notes))
		t.Logf("Original Models nil? %v, len=%d", empty.Models == nil, len(empty.Models))
		t.Logf("Got Models nil? %v, len=%d", got.Models == nil, len(got.Models))
	}
}
