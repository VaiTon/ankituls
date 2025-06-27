// SPDX-FileCopyrightText: 2025 Eyad Issa <eyadlorenzo@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

package main

import (
	"os"

	"github.com/fatih/color"
	"github.com/spf13/cobra"
)

func main() {
	var rootCmd = &cobra.Command{
		Use:   "ankitu",
		Short: "Ankitu - Anki deck import/export tool",
	}

	var exportCmd = &cobra.Command{
		Use:   "export [deck]",
		Short: "Export a deck to TOML/JSON/YAML (prints to stdout)",
		Args:  cobra.ExactArgs(1),
		Run:   exportDeck,
	}
	exportCmd.Flags().StringP("format", "F", "toml", "Export format: toml, json, yaml")

	var listCmd = &cobra.Command{
		Use:   "list",
		Short: "List available decks",
		Run:   listDecks,
	}

	var force bool
	var importCmd = &cobra.Command{
		Use:   "import [file]",
		Short: "Import a deck from TOML",
		Args:  cobra.ExactArgs(1),
		Run:   importDeck,
	}
	importCmd.Flags().BoolVarP(&force, "force", "f", false, "Overwrite deck if it already exists")

	rootCmd.AddCommand(exportCmd, importCmd, listCmd)
	if err := rootCmd.Execute(); err != nil {
		color.Red("Error: %v", err)
		os.Exit(1)
	}
}
