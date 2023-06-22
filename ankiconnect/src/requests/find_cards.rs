use serde::{Deserialize, Serialize};

use crate::{AnkiRequestable, CardId};

#[derive(Debug, Serialize)]
pub struct FindCardsRequest {
    pub query: String,
}

#[derive(Debug, Deserialize)]
pub struct FindCardsResponse(pub Vec<CardId>);

impl AnkiRequestable for FindCardsRequest {
    type Response = FindCardsResponse;

    const ACTION: &'static str = "findCards";
    const VERSION: u16 = 6;
}
