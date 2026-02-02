use serde::{Deserialize, Serialize};
use specta::Type;

/// Real-time system statistics
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct SystemStats {
    /// CPU usage percentage (0-100)
    pub cpu_usage: f32,
    
    /// Per-core CPU usage
    pub cpu_cores: Vec<f32>,
    
    /// GPU usage percentage (0-100), None if not available
    pub gpu_usage: Option<f32>,
    
    /// GPU temperature in Celsius
    pub gpu_temp: Option<f32>,
    
    /// CPU temperature in Celsius
    pub cpu_temp: Option<f32>,
    
    /// RAM usage in MB
    pub ram_used: u64,
    
    /// Total RAM in MB
    pub ram_total: u64,
    
    /// Timestamp of measurement
    pub timestamp: u64,
}

/// Event payload for system stats streaming
#[derive(Debug, Clone, Serialize, Type)]
pub struct SystemStatsEvent(pub SystemStats);

/// CPU information
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct CpuInfo {
    pub name: String,
    pub vendor: String,
    pub frequency: u64, // MHz
    pub cores: usize,
}

/// GPU information
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct GpuInfo {
    pub name: String,
    pub driver_version: String,
    pub vram_total: u64, // MB
    pub vram_used: u64,  // MB
}
