use crate::models::proton::ProtonVersion;
use anyhow::Result;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

/// Proton compatibility tool scanner
pub struct ProtonScanner {
    search_paths: Vec<PathBuf>,
}

impl ProtonScanner {
    /// Create a new scanner with standard Bazzite/Steam paths
    pub fn new() -> Self {
        let home = home::home_dir().unwrap_or_else(|| PathBuf::from("/home/user"));

        let search_paths = vec![
            home.join(".steam/compatibilitytools.d"),
            home.join(".local/share/Steam/compatibilitytools.d"),
            home.join(".var/app/com.valvesoftware.Steam/.local/share/Steam/compatibilitytools.d"),
        ];

        Self { search_paths }
    }

    /// Scan for all installed Proton versions
    pub fn list_versions(&self) -> Result<Vec<ProtonVersion>> {
        let mut versions = Vec::new();

        for base_path in &self.search_paths {
            if !base_path.exists() {
                continue;
            }

            println!("Scanning: {:?}", base_path);

            // List immediate subdirectories (each is a Proton version)
            for entry in fs::read_dir(base_path)? {
                let entry = entry?;
                let path = entry.path();

                if !path.is_dir() {
                    continue;
                }

                // Skip hidden directories
                if path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .map(|s| s.starts_with('.'))
                    .unwrap_or(false)
                {
                    continue;
                }

                match Self::parse_proton_version(&path) {
                    Ok(version) => versions.push(version),
                    Err(e) => {
                        eprintln!("Failed to parse Proton version at {:?}: {}", path, e);
                    }
                }
            }
        }

        // Sort by version (newest first)
        versions.sort_by(|a, b| b.name.cmp(&a.name));

        Ok(versions)
    }

    /// Parse a Proton installation directory
    fn parse_proton_version(path: &Path) -> Result<ProtonVersion> {
        let id = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("Unknown")
            .to_string();

        let name = id.clone();

        // Check if it's GE-Proton
        let is_ge_proton = id.contains("GE-Proton") || id.contains("Proton-GE");

        // Try to extract version number
        let version = Self::extract_version_number(&id);

        // Calculate directory size
        let size = Self::calculate_dir_size(path).unwrap_or(0) / (1024 * 1024); // MB

        // Get installation date (creation time or modified time)
        let installed_date = fs::metadata(path)
            .ok()
            .and_then(|m| m.created().or_else(|_| m.modified()).ok())
            .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
            .map(|d| d.as_secs());

        Ok(ProtonVersion {
            id,
            name,
            path: path.to_string_lossy().to_string(),
            version,
            size,
            is_ge_proton,
            installed_date,
        })
    }

    /// Extract version number from Proton name
    fn extract_version_number(name: &str) -> Option<String> {
        // Try to match patterns like "GE-Proton8-25" or "Proton-7.0"
        let parts: Vec<&str> = name.split('-').collect();
        
        for part in parts.iter().rev() {
            if part.chars().any(|c| c.is_ascii_digit()) {
                return Some(part.to_string());
            }
        }

        None
    }

    /// Calculate total size of a directory
    fn calculate_dir_size(path: &Path) -> Result<u64> {
        let mut total_size = 0u64;

        for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                total_size += entry.metadata()?.len();
            }
        }

        Ok(total_size)
    }
}

impl Default for ProtonScanner {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scanner_creation() {
        let scanner = ProtonScanner::new();
        assert!(!scanner.search_paths.is_empty());
    }

    #[test]
    fn test_version_extraction() {
        assert_eq!(
            ProtonScanner::extract_version_number("GE-Proton8-25"),
            Some("25".to_string())
        );
        assert_eq!(
            ProtonScanner::extract_version_number("Proton-7.0"),
            Some("7.0".to_string())
        );
    }
}
