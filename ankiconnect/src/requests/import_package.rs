use serde::{Deserialize, Serialize};

use crate::AnkiRequestable;

#[derive(Debug, Serialize)]
pub struct ImportPackageRequest {
    pub path: String,
}

#[derive(Debug, Deserialize)]
pub struct ImportPackageResponse(pub bool);

impl AnkiRequestable for ImportPackageRequest {
    type Response = ImportPackageResponse;

    const ACTION: &'static str = "importPackage";
    const VERSION: u16 = 6;
}
