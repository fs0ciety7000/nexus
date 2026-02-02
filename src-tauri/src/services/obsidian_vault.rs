use crate::models::obsidian::{Note, NoteMetadata, SaveNoteRequest};
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Obsidian vault manager
pub struct ObsidianVault {
    vault_path: PathBuf,
}

impl ObsidianVault {
    /// Create a new vault manager
    pub fn new<P: AsRef<Path>>(vault_path: P) -> Result<Self> {
        let vault_path = vault_path.as_ref().to_path_buf();
        
        if !vault_path.exists() {
            anyhow::bail!("Vault path does not exist: {:?}", vault_path);
        }

        Ok(Self { vault_path })
    }

    /// List all markdown notes in the vault
    pub fn list_notes(&self) -> Result<Vec<NoteMetadata>> {
        let mut notes = Vec::new();

        for entry in WalkDir::new(&self.vault_path)
            .follow_links(true)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            
            // Only process .md files
            if !path.is_file() || path.extension().and_then(|s| s.to_str()) != Some("md") {
                continue;
            }

            // Skip hidden files and .obsidian directory
            if path
                .components()
                .any(|c| c.as_os_str().to_string_lossy().starts_with('.'))
            {
                continue;
            }

            match self.create_note_metadata(path) {
                Ok(metadata) => notes.push(metadata),
                Err(e) => {
                    eprintln!("Failed to read metadata for {:?}: {}", path, e);
                }
            }
        }

        // Sort by modification time (newest first)
        notes.sort_by(|a, b| b.modified.cmp(&a.modified));

        Ok(notes)
    }

    /// Read a specific note by ID (relative path)
    pub fn read_note(&self, id: &str) -> Result<Note> {
        let full_path = self.vault_path.join(id);
        
        if !full_path.exists() {
            anyhow::bail!("Note not found: {}", id);
        }

        let content = fs::read_to_string(&full_path)
            .context("Failed to read note content")?;

        let metadata = self.create_note_metadata(&full_path)?;

        // Parse frontmatter (simple extraction between --- delimiters)
        let frontmatter = Self::extract_frontmatter(&content);

        Ok(Note {
            metadata,
            content,
            frontmatter,
        })
    }

    /// Save/update a note
    pub fn save_note(&self, request: SaveNoteRequest) -> Result<()> {
        let full_path = self.vault_path.join(&request.id);

        // Create parent directories if needed
        if let Some(parent) = full_path.parent() {
            fs::create_dir_all(parent).context("Failed to create parent directories")?;
        }

        fs::write(&full_path, &request.content)
            .context("Failed to write note content")?;

        Ok(())
    }

    /// Create metadata from file path
    fn create_note_metadata(&self, path: &Path) -> Result<NoteMetadata> {
        let metadata = fs::metadata(path).context("Failed to read file metadata")?;

        // Get relative path from vault root
        let id = path
            .strip_prefix(&self.vault_path)
            .context("Path not in vault")?
            .to_string_lossy()
            .to_string();

        // Extract title (filename without extension, or from frontmatter)
        let title = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Untitled")
            .to_string();

        let modified = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let size = metadata.len();

        // TODO: Parse tags from frontmatter
        let tags = Vec::new();

        Ok(NoteMetadata {
            id,
            title,
            path: path.to_string_lossy().to_string(),
            modified,
            size,
            tags,
        })
    }

    /// Extract YAML frontmatter from markdown content
    fn extract_frontmatter(content: &str) -> Option<String> {
        if !content.starts_with("---\n") {
            return None;
        }

        let rest = &content[4..];
        if let Some(end_idx) = rest.find("\n---\n") {
            Some(rest[..end_idx].to_string())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vault_creation() {
        // This test requires a valid vault path
        let vault_path = "/var/home/phantomhex/Documents/Data/Notes/fs0ciety/";
        
        if Path::new(vault_path).exists() {
            let vault = ObsidianVault::new(vault_path);
            assert!(vault.is_ok());
        }
    }

    #[test]
    fn test_frontmatter_extraction() {
        let content = "---\ntitle: Test\ntags: [test]\n---\n\n# Content";
        let fm = ObsidianVault::extract_frontmatter(content);
        assert!(fm.is_some());
        assert!(fm.unwrap().contains("title: Test"));
    }
}
