package main

import (
	"ankituls/internal/ankiconnect"
	"fmt"
	"os"

	"github.com/spf13/cobra"
	"github.com/fatih/color"
)

func listDecks(cmd *cobra.Command, args []string) {
	client := ankiconnect.NewAnkiClient()

	var deckNames []string
	if err := client.Request("deckNames", nil, 6, &deckNames); err != nil {
		color.Red("Failed to get deck names: %v", err)
		os.Exit(1)
	}

	if len(deckNames) == 0 {
		color.Yellow("No decks found in Anki.")
		return
	}

	fmt.Println("Available decks:")
	for _, deck := range deckNames {
		fmt.Println(" -", deck)
	}
}
