package main

import (
	"ankituls/internal/ankiconnect"
	"ankituls/internal/formats"
	"fmt"
	"os"
	"strings"

	"github.com/spf13/cobra"
	"github.com/fatih/color"
)

func exportDeck(cmd *cobra.Command, args []string) {
	format, _ := cmd.Flags().GetString("format")
	if len(args) < 1 {
		color.Red("Usage: export [deck]")
		os.Exit(1)
	}
	deck := args[0]
	client := ankiconnect.NewAnkiClient()

	// Check if deck exists
	var deckNames []string
	if err := client.Request("deckNames", nil, 6, &deckNames); err != nil {
		color.Red("Failed to get deck names: %v", err)
		os.Exit(1)
	}
	found := false
	for _, d := range deckNames {
		if d == deck {
			found = true
			break
		}
	}
	if !found {
		color.Red("Deck %q does not exist.", deck)
		fmt.Fprintf(os.Stderr, "Use \"%s list\" to list all available decks.\n", os.Args[0])
		os.Exit(1)
	}

	// Find notes
	var noteIDs []uint64
	findReq := ankiconnect.FindNotesRequest{Query: fmt.Sprintf(`"deck:%s"`, deck)}
	if err := client.Request("findNotes", findReq, 6, &noteIDs); err != nil {
		color.Red("Failed to find notes: %v", err)
		os.Exit(1)
	}

	// Get notes info
	var notesInfo []ankiconnect.NoteInfo
	notesReq := ankiconnect.NotesInfoRequest{Notes: noteIDs}
	if err := client.Request("notesInfo", notesReq, 6, &notesInfo); err != nil {
		color.Red("Failed to get notes info: %v", err)
		os.Exit(1)
	}

	// Convert ankiconnect.NoteInfo to formats.Note using ankiconnect package function
	notes := ankiconnect.ConvertNoteInfoSliceToNotes(notesInfo)

	// Gather unique model names used by the notes
	modelNameSet := make(map[string]struct{})
	for _, n := range notesInfo {
		modelNameSet[n.ModelName] = struct{}{}
	}
	modelNames := make([]string, 0, len(modelNameSet))
	for name := range modelNameSet {
		modelNames = append(modelNames, name)
	}

	// Fetch model definitions from AnkiConnect
	var models []any
	if len(modelNames) > 0 {
		modelReq := map[string]any{"modelNames": modelNames}
		if err := client.Request("findModelsByName", modelReq, 6, &models); err != nil {
			color.Red("Failed to fetch model definitions: %v", err)
			os.Exit(1)
		}
	}

	export := formats.Export{
		Version:  1,
		DeckName: deck,
		Notes:    notes,
		Models:   models,
	}

	err := formats.MarshalToWriter(strings.ToLower(format), os.Stdout, &export)
	if err != nil {
		color.Red("Failed to marshal %s: %v", format, err)
		os.Exit(1)
	}
}
