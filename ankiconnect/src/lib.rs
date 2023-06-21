use std::{error::Error, fmt::Debug};

use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod requests;

pub mod note;
pub use note::*;

#[derive(Debug, Deserialize)]
pub struct AnkiResponse<T> {
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
pub struct AnkiRequest<Params> {
    action: &'static str,
    version: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<Params>,
}
pub struct AnkiClient {
    client: reqwest::blocking::Client,
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
            client: reqwest::blocking::Client::new(),
            url: "http://localhost:8765".to_string(),
        }
    }

    pub fn request<Request, Response>(&self, request: Request) -> Result<Response, Box<dyn Error>>
    where
        AnkiRequest<Request>: From<Request>,
        Request: Serialize,
        Response: DeserializeOwned,
        Response: Debug,
    {
        let request = AnkiRequest::from(request);

        let response = self.client.post(&self.url).json(&request).send()?;
        let response: AnkiResponse<Response> = response.json()?;

        Result::from(response).map_err(|error| error.into())
    }
}
