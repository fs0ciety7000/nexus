use crate::models::obsidian::{Note, NoteMetadata, SaveNoteRequest};
use crate::services::obsidian_vault::ObsidianVault;

const VAULT_PATH: &str = "/var/home/phantomhex/Documents/Data/Notes/fs0ciety/";

/// List all notes in the Obsidian vault
#[tauri::command]
#[specta::specta]
pub async fn list_notes() -> Result<Vec<NoteMetadata>, String> {
    let vault = ObsidianVault::new(VAULT_PATH).map_err(|e| e.to_string())?;
    vault.list_notes().map_err(|e| e.to_string())
}

/// Read a specific note by ID
#[tauri::command]
#[specta::specta]
pub async fn read_note(id: String) -> Result<Note, String> {
    let vault = ObsidianVault::new(VAULT_PATH).map_err(|e| e.to_string())?;
    vault.read_note(&id).map_err(|e| e.to_string())
}

/// Save/update a note
#[tauri::command]
#[specta::specta]
pub async fn save_note(request: SaveNoteRequest) -> Result<(), String> {
    let vault = ObsidianVault::new(VAULT_PATH).map_err(|e| e.to_string())?;
    vault.save_note(request).map_err(|e| e.to_string())
}
