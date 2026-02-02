use serde::{Deserialize, Serialize};
use specta::Type;

/// Installed Proton version information
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ProtonVersion {
    /// Version identifier (e.g., "GE-Proton8-25")
    pub id: String,
    
    /// Display name
    pub name: String,
    
    /// Full installation path
    pub path: String,
    
    /// Version number (parsed if available)
    pub version: Option<String>,
    
    /// Installation size in MB
    pub size: u64,
    
    /// Whether this is a GE-Proton variant
    pub is_ge_proton: bool,
    
    /// Installation date (Unix epoch)
    pub installed_date: Option<u64>,
}

/// Proton compatibility tool metadata
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct ProtonCompatTool {
    pub version: ProtonVersion,
    
    /// Whether this version is currently in use by any game
    pub in_use: bool,
    
    /// List of game app IDs using this version
    pub used_by_games: Vec<u32>,
}
