use serde::{Deserialize, Serialize};
use std::time::Duration;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub mod connection;
pub mod server;
pub mod protocol;
pub mod encryption;
pub mod dns;
pub mod killswitch;
pub mod split_tunnel;
pub mod analytics;
pub mod config;

// Re-export main types
pub use connection::VpnConnection;
pub use server::{VpnServer, ServerLocation};
pub use protocol::VpnProtocol;
pub use config::VpnConfig;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionStatus {
    Disconnected,
    Connecting,
    Connected,
    Disconnecting,
    Reconnecting,
    Error(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionInfo {
    pub status: ConnectionStatus,
    pub server: Option<VpnServer>,
    pub connected_at: Option<DateTime<Utc>>,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub duration: Duration,
    pub ip_address: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnStats {
    pub current_speed_up: f64,    // MB/s
    pub current_speed_down: f64,  // MB/s
    pub total_upload: u64,         // bytes
    pub total_download: u64,       // bytes
    pub latency: u32,              // ms
    pub packet_loss: f32,          // percentage
}

#[derive(Debug, thiserror::Error)]
pub enum VpnError {
    #[error("Connection failed: {0}")]
    ConnectionFailed(String),
    
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    #[error("Server unavailable: {0}")]
    ServerUnavailable(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Network error: {0}")]
    NetworkError(String),
    
    #[error("Encryption error: {0}")]
    EncryptionError(String),
}

pub type Result<T> = std::result::Result<T, VpnError>;
