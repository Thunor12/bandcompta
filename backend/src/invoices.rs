use std::path::{Path, PathBuf};

use bandcompta_shared::UploadResponse;

const INVOICES_DIR: &str = "invoices";

pub fn ensure_invoices_dir() -> std::io::Result<()> {
    std::fs::create_dir_all(INVOICES_DIR)
}

pub async fn save_invoice(filename: &str, data: &[u8]) -> Result<UploadResponse, String> {
    ensure_invoices_dir().map_err(|err| err.to_string())?;

    let safe_name = sanitize_filename(filename);
    if safe_name.is_empty() {
        return Err("nom de fichier invalide".into());
    }

    let stored_path = Path::new(INVOICES_DIR).join(&safe_name);
    tokio::fs::write(&stored_path, data)
        .await
        .map_err(|err| err.to_string())?;

    Ok(UploadResponse {
        path: format!("/{INVOICES_DIR}/{safe_name}"),
    })
}

fn sanitize_filename(filename: &str) -> String {
    Path::new(filename)
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("")
        .chars()
        .map(|ch| if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | '_') {
            ch
        } else {
            '_'
        })
        .collect()
}
