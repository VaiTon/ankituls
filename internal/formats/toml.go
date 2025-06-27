// SPDX-FileCopyrightText: 2025 Eyad Issa <eyadlorenzo@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

package formats

import (
	"io"

	"github.com/pelletier/go-toml/v2"
)

// MarshalTOML writes the Export as TOML to the provided writer, with deterministic sorting and schema comment.
func MarshalTOML(w io.Writer, e *Export) error {
	e.sortForExport()
	data, err := toml.Marshal(e)
	if err != nil {
		return err
	}
	comment := "# ankituls export schema version = 1\n"
	if _, err := w.Write([]byte(comment)); err != nil {
		return err
	}
	_, err = w.Write(data)
	return err
}

// UnmarshalTOML reads TOML from the reader and decodes into Export.
func UnmarshalTOML(r io.Reader, e *Export) error {
	data, err := io.ReadAll(r)
	if err != nil {
		return err
	}
	return toml.Unmarshal(data, e)
}
