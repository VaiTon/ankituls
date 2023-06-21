use serde::{Deserialize, Serialize};

use crate::AnkiRequest;

#[derive(Debug, Serialize)]
pub struct ExportPackageRequest {
    pub deck: String,
    pub path: String,
    #[serde(rename = "includeSched")]
    pub include_scheduling: bool,
}
#[derive(Debug, Deserialize)]
pub struct ExportPackageResponse(pub bool);

impl From<ExportPackageRequest> for AnkiRequest<ExportPackageRequest> {
    fn from(value: ExportPackageRequest) -> Self {
        AnkiRequest {
            action: "exportPackage",
            version: 6,
            params: Some(value),
        }
    }
}
