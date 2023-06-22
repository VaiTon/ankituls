use serde::{Deserialize, Serialize};

use crate::{AnkiRequestable};

#[derive(Debug, Serialize)]
pub struct ExportPackageRequest {
    pub deck: String,
    pub path: String,
    #[serde(rename = "includeSched")]
    pub include_scheduling: bool,
}
#[derive(Debug, Deserialize)]
pub struct ExportPackageResponse(pub bool);

impl AnkiRequestable for ExportPackageRequest {
    type Response = ExportPackageResponse;

    const ACTION: &'static str = "exportPackage";
    const VERSION: u16 = 6;
}
