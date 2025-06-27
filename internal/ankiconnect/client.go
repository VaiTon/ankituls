// SPDX-FileCopyrightText: 2025 Eyad Issa <eyadlorenzo@gmail.com>
//
// SPDX-License-Identifier: GPL-3.0-or-later

package ankiconnect

import (
	"bytes"
	"encoding/json"
	"fmt"
	"io"
	"net/http"
)

// AnkiClient is a client for communicating with AnkiConnect.
type AnkiClient struct {
	URL string
}

// NewAnkiClient creates a new AnkiConnect client with the default URL.
func NewAnkiClient() *AnkiClient {
	return &AnkiClient{
		URL: "http://localhost:8765", // Default AnkiConnect URL
	}
}

// AnkiRequest represents a request to AnkiConnect.
type AnkiRequest struct {
	Action  string `json:"action"`
	Version int    `json:"version"`
	Params  any    `json:"params,omitempty"`
}

// AnkiResponse represents a response from AnkiConnect.
type AnkiResponse struct {
	Result json.RawMessage `json:"result"`
	Error  *string         `json:"error"`
}

// Request sends a request to AnkiConnect and decodes the result into 'result' if not nil.
// 'action' is the AnkiConnect action, 'params' is the request parameters, 'version' is the API version.
func (c *AnkiClient) Request(action string, params any, version int, result any) error {
	reqBody, err := json.Marshal(AnkiRequest{
		Action:  action,
		Version: version,
		Params:  params,
	})
	if err != nil {
		return fmt.Errorf("failed to marshal request: %w", err)
	}

	resp, err := http.Post(c.URL, "application/json", bytes.NewReader(reqBody))
	if err != nil {
		return fmt.Errorf("failed to send request: %w", err)
	}
	defer resp.Body.Close()

	body, err := io.ReadAll(resp.Body)
	if err != nil {
		return fmt.Errorf("failed to read response: %w", err)
	}

	var ankiResp AnkiResponse
	if err := json.Unmarshal(body, &ankiResp); err != nil {
		return fmt.Errorf("failed to unmarshal response: %w", err)
	}
	if ankiResp.Error != nil {
		return fmt.Errorf("AnkiConnect error: %s", *ankiResp.Error)
	}
	if result != nil {
		if err := json.Unmarshal(ankiResp.Result, result); err != nil {
			return fmt.Errorf("failed to decode result: %w", err)
		}
	}
	return nil
}
