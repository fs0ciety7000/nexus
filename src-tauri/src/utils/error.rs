use thiserror::Error;

#[derive(Error, Debug)]
pub enum NexusError {
    #[error("System monitoring error: {0}")]
    SystemMonitor(String),

    #[error("Obsidian vault error: {0}")]
    ObsidianVault(String),

    #[error("Proton scanner error: {0}")]
    ProtonScanner(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Other error: {0}")]
    Other(String),
}

impl From<NexusError> for String {
    fn from(error: NexusError) -> Self {
        error.to_string()
    }
}
