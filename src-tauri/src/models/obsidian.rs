use serde::{Deserialize, Serialize};
use specta::Type;

/// Metadata for a single note
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct NoteMetadata {
    /// Unique identifier (relative path from vault root)
    pub id: String,
    
    /// Display title (from frontmatter or filename)
    pub title: String,
    
    /// Full file path
    pub path: String,
    
    /// Last modified timestamp (Unix epoch)
    pub modified: u64,
    
    /// File size in bytes
    pub size: u64,
    
    /// Tags from frontmatter
    pub tags: Vec<String>,
}

/// Full note content with metadata
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct Note {
    pub metadata: NoteMetadata,
    
    /// Raw markdown content
    pub content: String,
    
    /// Parsed frontmatter (YAML)
    pub frontmatter: Option<String>,
}

/// Request to save/update a note
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SaveNoteRequest {
    /// Note ID (path relative to vault)
    pub id: String,
    
    /// New markdown content
    pub content: String,
}
