use std::{error::Error, fmt::Debug};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod requests;

pub mod note;
pub use note::*;

pub trait AnkiRequestable
where
    Self: Serialize + Sized,
{
    const VERSION: u16;
    const ACTION: &'static str;
    type Response: DeserializeOwned;

    fn params(self) -> Option<Self> {
        Some(self)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, Copy, Hash, PartialEq, Eq)]
pub struct CardId(pub u64);
#[derive(Debug, Deserialize, Serialize, Clone, Copy, PartialEq)]
pub struct NoteId(pub u64);
#[derive(Debug, Deserialize, Serialize, Clone, Copy)]
pub struct DeckId(pub u64);

#[derive(Debug, Deserialize)]
struct AnkiResponse<T> {
    pub result: Option<T>,
    pub error: Option<String>,
}

impl<T> From<AnkiResponse<T>> for Result<T, String> {
    fn from(value: AnkiResponse<T>) -> Self {
        match value.error {
            Some(error) => Err(error)?,
            None => value.result.ok_or_else(|| "No result".into()),
        }
    }
}

#[derive(Debug, Serialize)]
struct AnkiRequest<Params> {
    action: &'static str,
    version: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Params>,
}
pub struct AnkiClient {
    client: ureq::Agent,
    url: String,
}

impl Default for AnkiClient {
    fn default() -> Self {
        Self::new()
    }
}

impl AnkiClient {
    pub fn new() -> Self {
        AnkiClient {
            client: ureq::agent(),
            url: "http://localhost:8765".to_string(),
        }
    }

    pub fn request<Request>(&self, request: Request) -> Result<Request::Response, Box<dyn Error>>
    where
        Request: AnkiRequestable + Serialize,
    {
        let request = AnkiRequest {
            action: Request::ACTION,
            version: Request::VERSION,
            params: request.params(),
        };

        let response = self.client.post(&self.url).send_json(&request)?;
        let response: AnkiResponse<Request::Response> = response.into_json()?;

        Result::from(response).map_err(|error| error.into())
    }
}
