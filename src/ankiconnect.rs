use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub struct AnkiClient {
    client: reqwest::blocking::Client,
    url: String,
}

impl AnkiClient {
    pub fn new() -> Self {
        AnkiClient {
            client: reqwest::blocking::Client::new(),
            url: "http://localhost:8765".to_string(),
        }
    }

    pub fn request<Request: Serialize, Response: DeserializeOwned>(
        &self,
        request: &AnkiRequest<Request>,
    ) -> Result<AnkiResponse<Response>, reqwest::Error> {
        self.client
            .post(&self.url)
            .json(request)
            .send()?
            .json::<AnkiResponse<Response>>()
    }
}

#[derive(Serialize)]
pub struct AnkiRequest<T> {
    pub action: String,
    pub version: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<T>,
}

#[derive(Deserialize, Debug)]
pub struct AnkiResponse<T> {
    pub error: Option<String>,
    pub result: Option<T>,
}
#[derive(Serialize)]
pub struct ImportPackageParams {
    pub path: String,
}

#[derive(Serialize)]
pub struct ExportPackageParams {
    #[serde(rename = "deck")]
    pub deck_name: String,
    pub path: String,
    #[serde(rename = "includeSched")]
    pub include_scheduling: bool,
}
