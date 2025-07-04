// SPDX-FileCopyrightText: 2025 Eyad Issa <eyadlorenzo@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

package formats

import (
	"encoding/json"
	"io"
)

// MarshalJSON writes the Export as pretty JSON to the provided writer, with deterministic sorting.
func MarshalJSON(w io.Writer, e *Export) error {
	e.sortForExport()
	enc := json.NewEncoder(w)
	enc.SetIndent("", "  ")
	return enc.Encode(e)
}

// UnmarshalJSON reads JSON from the reader and decodes into Export.
// It enforces deterministic order by calling sortForExport after decoding.
func UnmarshalJSON(r io.Reader, e *Export) error {
	dec := json.NewDecoder(r)
	if err := dec.Decode(e); err != nil {
		return err
	}
	e.sortForExport()
	return nil
}
