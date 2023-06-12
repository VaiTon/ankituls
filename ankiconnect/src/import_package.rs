use serde::{Deserialize, Serialize};

use super::AnkiRequest;

#[derive(Debug, Serialize)]
pub struct ImportPackageRequest {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct ImportPackageResponse(pub bool);

impl From<ImportPackageRequest> for AnkiRequest<ImportPackageRequest> {
    fn from(value: ImportPackageRequest) -> Self {
        AnkiRequest {
            action: "importPackage",
            version: 6,
            params: Some(value),
        }
    }
}
