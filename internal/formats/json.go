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
func UnmarshalJSON(r io.Reader, e *Export) error {
	dec := json.NewDecoder(r)
	return dec.Decode(e)
}
