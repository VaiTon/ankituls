// SPDX-FileCopyrightText: 2025 Eyad Issa <eyadlorenzo@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

package formats

import (
	"io"

	"gopkg.in/yaml.v3"
)

// quoteAllStrings recursively sets the Style of all string nodes to DoubleQuotedStyle.
func quoteAllStrings(n *yaml.Node) {
	if n.Kind == yaml.ScalarNode && n.Tag == "!!str" {
		n.Style = yaml.DoubleQuotedStyle
	}
	for _, c := range n.Content {
		quoteAllStrings(c)
	}
}

// MarshalYAML writes the Export as YAML to the provided writer, with deterministic sorting, quoting, and schema comment.
func MarshalYAML(w io.Writer, e *Export) error {
	e.sortForExport()
	var node yaml.Node
	if err := node.Encode(e); err != nil {
		return err
	}
	quoteAllStrings(&node)
	node.HeadComment = "ankituls export schema version = 1"
	enc := yaml.NewEncoder(w)
	defer enc.Close()
	return enc.Encode(&node)
}

// UnmarshalYAML reads YAML from the reader and decodes into Export.
func UnmarshalYAML(r io.Reader, e *Export) error {
	dec := yaml.NewDecoder(r)
	return dec.Decode(e)
}
