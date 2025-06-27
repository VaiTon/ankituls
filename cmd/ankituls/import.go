package main

import (
	"github.com/VaiTon/ankituls/internal/ankiconnect"
	"github.com/VaiTon/ankituls/internal/formats"
	"os"
	"strings"

	"github.com/fatih/color"
	"github.com/spf13/cobra"
)

func importDeck(cmd *cobra.Command, args []string) {
	force, _ := cmd.Flags().GetBool("force")

	file := args[0]
	format := ""
	switch {
	case strings.HasSuffix(file, ".toml"):
		format = "toml"
	case strings.HasSuffix(file, ".json"):
		format = "json"
	case strings.HasSuffix(file, ".yaml"), strings.HasSuffix(file, ".yml"):
		format = "yaml"
	default:
		color.Red("Unknown file extension for import: %s", file)
		os.Exit(1)
	}

	f, err := os.Open(file)
	if err != nil {
		color.Red("Failed to open file: %v", err)
		os.Exit(1)
	}
	defer f.Close()

	var export formats.Export
	if err := formats.UnmarshalFromReader(format, f, &export); err != nil {
		color.Red("Failed to parse %s: %v", format, err)
		os.Exit(1)
	}

	client := ankiconnect.NewAnkiClient()

	// Check if deck exists
	var deckNames []string
	if err := client.Request("deckNames", nil, 6, &deckNames); err != nil {
		color.Red("Failed to get deck names: %v", err)
		os.Exit(1)
	}
	exists := false
	for _, d := range deckNames {
		if d == export.DeckName {
			exists = true
			break
		}
	}
	if exists && !force {
		color.Red("Deck %q already exists. Use --force to overwrite.", export.DeckName)
		os.Exit(1)
	}
	if exists && force {
		delReq := ankiconnect.DeleteDecksRequest{
			Decks:    []string{export.DeckName},
			CardsToo: true,
		}
		if err := client.Request("deleteDecks", delReq, 6, nil); err != nil {
			color.Red("Failed to delete deck: %v", err)
			os.Exit(1)
		}
	}
	// Create deck
	createReq := ankiconnect.CreateDeckRequest{Deck: export.DeckName}
	if err := client.Request("createDeck", createReq, 6, nil); err != nil {
		color.Red("Failed to create deck: %v", err)
		os.Exit(1)
	}
	// Add notes
	notes := make([]ankiconnect.CreateNote, 0, len(export.Notes))
	for _, n := range export.Notes {
		fields := make(map[string]string)
		for k, v := range n.Fields {
			fields[k] = v.Value
		}
		notes = append(notes, ankiconnect.CreateNote{
			DeckName:  export.DeckName,
			ModelName: n.ModelName,
			Fields:    fields,
			Tags:      n.Tags,
		})
	}
	addReq := ankiconnect.AddNotesRequest{Notes: notes}
	if err := client.Request("addNotes", addReq, 6, nil); err != nil {
		color.Red("Failed to add notes: %v", err)
		os.Exit(1)
	}
	color.Green("Imported deck %s", export.DeckName)
}
