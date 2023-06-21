use std::{error::Error, path::Path};

use ankiconnect::{AnkiClient, ImportPackageRequest, ImportPackageResponse};

pub fn import_apkg(client: &AnkiClient, path: &Path) -> Result<(), Box<dyn Error>> {
    let path = path.to_str().ok_or("invalid path")?.to_owned();

    let response = client.request(ImportPackageRequest { path })?;

    match response {
        ImportPackageResponse(true) => Ok(()),
        ImportPackageResponse(false) => Err("could not import file")?,
    }
}
