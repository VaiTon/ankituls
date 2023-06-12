use serde::{Deserialize, Serialize};

use super::AnkiRequest;

#[derive(Debug, Serialize)]
pub struct FindNotesRequest {
    pub query: String,
}

#[derive(Debug, Deserialize)]
pub struct FindNotesResponse(pub Vec<u64>);

impl From<FindNotesRequest> for AnkiRequest<FindNotesRequest> {
    fn from(value: FindNotesRequest) -> Self {
        AnkiRequest {
            action: "findNotes",
            version: 6,
            params: Some(value),
        }
    }
}
