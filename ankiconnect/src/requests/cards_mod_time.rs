use crate::{AnkiRequestable, CardId};

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
pub struct CardModTimeRequest {
    #[serde(rename = "cards")]
    pub ids: Vec<CardId>,
}

#[derive(Debug, Deserialize)]
pub struct CardModTimeResponse(pub Vec<CardModTime>);

#[derive(Debug, Deserialize)]
pub struct CardModTime {
    #[serde(rename = "cardId")]
    pub card_id: CardId,
    #[serde(rename = "mod")]
    pub mod_time: u32,
}

impl AnkiRequestable for CardModTimeRequest {
    type Response = CardModTimeResponse;

    const ACTION: &'static str = "cardsModTime";
    const VERSION: u16 = 6;
}
