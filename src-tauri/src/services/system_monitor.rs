use crate::models::system::SystemStats;
use anyhow::{Context, Result};
use nvml_wrapper::Nvml;
use sysinfo::{CpuRefreshKind, RefreshKind, System};
use std::time::{SystemTime, UNIX_EPOCH};

/// Hardware telemetry aggregator
pub struct SystemMonitor {
    sys: System,
    nvml: Option<Nvml>,
}

impl SystemMonitor {
    /// Initialize the system monitor
    pub fn new() -> Result<Self> {
        let sys = System::new_with_specifics(
            RefreshKind::new()
                .with_cpu(CpuRefreshKind::everything())
                .with_memory(),
        );

        // Try to initialize NVML for Nvidia GPU monitoring
        let nvml = match Nvml::init() {
            Ok(n) => {
                println!("✓ NVML initialized successfully");
                Some(n)
            }
            Err(e) => {
                eprintln!("⚠ NVML initialization failed: {}. GPU monitoring disabled.", e);
                None
            }
        };

        Ok(Self { sys, nvml })
    }

    /// Capture current hardware snapshot
    pub fn snapshot(&mut self) -> Result<SystemStats> {
        // Refresh system data
        self.sys.refresh_cpu_all();
        self.sys.refresh_memory();

        // Calculate average CPU usage
        let cpu_usage = self.sys.global_cpu_usage();

        // Get per-core usage
        let cpu_cores: Vec<f32> = self
            .sys
            .cpus()
            .iter()
            .map(|cpu| cpu.cpu_usage())
            .collect();

        // Get GPU stats if available
        let (gpu_usage, gpu_temp) = self.get_gpu_stats()?;

        // Get CPU temperature (Linux-specific via sysinfo)
        let cpu_temp = self.get_cpu_temp();

        // RAM usage
        let ram_used = self.sys.used_memory() / (1024 * 1024); // Convert to MB
        let ram_total = self.sys.total_memory() / (1024 * 1024);

        // Current timestamp
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .context("Failed to get system time")?
            .as_secs();

        Ok(SystemStats {
            cpu_usage,
            cpu_cores,
            gpu_usage,
            gpu_temp,
            cpu_temp,
            ram_used,
            ram_total,
            timestamp,
        })
    }

    /// Get GPU usage and temperature via NVML
    fn get_gpu_stats(&self) -> Result<(Option<f32>, Option<f32>)> {
        let nvml = match &self.nvml {
            Some(n) => n,
            None => return Ok((None, None)),
        };

        // Get first GPU (device 0)
        let device = nvml.device_by_index(0).ok();
        
        if let Some(device) = device {
            let utilization = device
                .utilization_rates()
                .ok()
                .map(|u| u.gpu as f32);

            let temperature = device
                .temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
                .ok()
                .map(|t| t as f32);

            Ok((utilization, temperature))
        } else {
            Ok((None, None))
        }
    }

    /// Get CPU temperature (best-effort via components)
    fn get_cpu_temp(&mut self) -> Option<f32> {
        // On Linux, sysinfo can read thermal zones
        // This is hardware-specific and may not work on all systems
        self.sys.refresh_components_list();
        self.sys.refresh_components();

        // Look for a component labeled "CPU" or "Core"
        for component in self.sys.components() {
            let label = component.label().to_lowercase();
            if label.contains("cpu") || label.contains("core") || label.contains("package") {
                return Some(component.temperature());
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_monitor_init() {
        let monitor = SystemMonitor::new();
        assert!(monitor.is_ok());
    }

    #[test]
    fn test_snapshot() {
        let mut monitor = SystemMonitor::new().unwrap();
        let stats = monitor.snapshot();
        assert!(stats.is_ok());
        
        let stats = stats.unwrap();
        assert!(stats.cpu_usage >= 0.0 && stats.cpu_usage <= 100.0);
        assert!(stats.ram_total > 0);
    }
}
