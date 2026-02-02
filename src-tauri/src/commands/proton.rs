use crate::models::proton::ProtonVersion;
use crate::services::proton_scanner::ProtonScanner;

/// List all installed Proton versions
#[tauri::command]
#[specta::specta]
pub async fn list_proton_versions() -> Result<Vec<ProtonVersion>, String> {
    let scanner = ProtonScanner::new();
    scanner.list_versions().map_err(|e| e.to_string())
}
