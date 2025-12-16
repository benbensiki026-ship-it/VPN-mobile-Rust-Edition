use serde::{Deserialize, Serialize};
use crate::protocol::{VpnProtocol, ProtocolConfig};
use crate::killswitch::KillSwitchConfig;
use crate::split_tunnel::SplitTunnelConfig;
use crate::dns::DnsMode;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnConfig {
    // General settings
    pub auto_connect: bool,
    pub auto_connect_server: Option<String>,
    pub start_on_boot: bool,
    pub minimize_to_tray: bool,
    
    // Protocol settings
    pub protocol_config: ProtocolConfig,
    
    // Security settings
    pub kill_switch: KillSwitchConfig,
    pub dns_mode: DnsMode,
    pub ipv6_leak_protection: bool,
    pub webrtc_leak_protection: bool,
    
    // Advanced settings
    pub split_tunnel: SplitTunnelConfig,
    pub mtu: u16,
    pub connect_timeout: u32,  // seconds
    pub reconnect_on_disconnect: bool,
    pub reconnect_attempts: u32,
    
    // Privacy settings
    pub block_trackers: bool,
    pub block_ads: bool,
    pub block_malware: bool,
    pub anonymous_usage_stats: bool,
    
    // Notification settings
    pub show_notifications: bool,
    pub notify_on_connect: bool,
    pub notify_on_disconnect: bool,
    pub notify_on_ip_change: bool,
}

impl Default for VpnConfig {
    fn default() -> Self {
        Self {
            // General
            auto_connect: false,
            auto_connect_server: None,
            start_on_boot: false,
            minimize_to_tray: true,
            
            // Protocol
            protocol_config: ProtocolConfig::default(),
            
            // Security
            kill_switch: KillSwitchConfig::default(),
            dns_mode: DnsMode::Auto,
            ipv6_leak_protection: true,
            webrtc_leak_protection: true,
            
            // Advanced
            split_tunnel: SplitTunnelConfig::default(),
            mtu: 1420,
            connect_timeout: 30,
            reconnect_on_disconnect: true,
            reconnect_attempts: 3,
            
            // Privacy
            block_trackers: false,
            block_ads: false,
            block_malware: true,
            anonymous_usage_stats: false,
            
            // Notifications
            show_notifications: true,
            notify_on_connect: true,
            notify_on_disconnect: true,
            notify_on_ip_change: false,
        }
    }
}

impl VpnConfig {
    pub fn load_from_file(path: &PathBuf) -> crate::Result<Self> {
        let contents = fs::read_to_string(path)
            .map_err(|e| crate::VpnError::ConfigError(format!("Failed to read config: {}", e)))?;
        
        let config: VpnConfig = serde_json::from_str(&contents)
            .map_err(|e| crate::VpnError::ConfigError(format!("Failed to parse config: {}", e)))?;
        
        Ok(config)
    }

    pub fn save_to_file(&self, path: &PathBuf) -> crate::Result<()> {
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| crate::VpnError::ConfigError(format!("Failed to serialize config: {}", e)))?;
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| crate::VpnError::ConfigError(format!("Failed to create config directory: {}", e)))?;
        }
        
        fs::write(path, json)
            .map_err(|e| crate::VpnError::ConfigError(format!("Failed to write config: {}", e)))?;
        
        Ok(())
    }

    pub fn get_config_path() -> PathBuf {
        // Default config path
        #[cfg(target_os = "linux")]
        {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join(".config/vpn-mobile/config.json")
        }
        
        #[cfg(target_os = "macos")]
        {
            let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(home).join("Library/Application Support/VPN Mobile/config.json")
        }
        
        #[cfg(target_os = "windows")]
        {
            let appdata = std::env::var("APPDATA").unwrap_or_else(|_| ".".to_string());
            PathBuf::from(appdata).join("VPN Mobile\\config.json")
        }
        
        #[cfg(target_os = "android")]
        {
            PathBuf::from("/data/data/com.vpn.mobile/files/config.json")
        }
        
        #[cfg(target_os = "ios")]
        {
            PathBuf::from("/var/mobile/Library/Application Support/VPN Mobile/config.json")
        }
        
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows", target_os = "android", target_os = "ios")))]
        {
            PathBuf::from("config.json")
        }
    }

    pub fn preset_maximum_security() -> Self {
        Self {
            protocol_config: ProtocolConfig::new(VpnProtocol::WireGuard)
                .with_obfuscation(true),
            kill_switch: KillSwitchConfig {
                mode: crate::killswitch::KillSwitchMode::Always,
                allow_lan: false,
                allowed_ips: Vec::new(),
                allowed_apps: Vec::new(),
            },
            ipv6_leak_protection: true,
            webrtc_leak_protection: true,
            block_trackers: true,
            block_ads: true,
            block_malware: true,
            ..Default::default()
        }
    }

    pub fn preset_streaming() -> Self {
        Self {
            protocol_config: ProtocolConfig::new(VpnProtocol::WireGuard),
            kill_switch: KillSwitchConfig {
                mode: crate::killswitch::KillSwitchMode::Disabled,
                ..Default::default()
            },
            reconnect_on_disconnect: true,
            reconnect_attempts: 5,
            ..Default::default()
        }
    }

    pub fn preset_torrenting() -> Self {
        Self {
            protocol_config: ProtocolConfig::new(VpnProtocol::OpenVPN),
            kill_switch: KillSwitchConfig {
                mode: crate::killswitch::KillSwitchMode::Always,
                allow_lan: true,
                ..Default::default()
            },
            split_tunnel: SplitTunnelConfig {
                mode: crate::split_tunnel::SplitTunnelMode::IncludeOnly,
                ..Default::default()
            },
            ..Default::default()
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.connect_timeout < 5 {
            return Err("Connect timeout must be at least 5 seconds".to_string());
        }
        
        if self.reconnect_attempts > 10 {
            return Err("Reconnect attempts cannot exceed 10".to_string());
        }
        
        if self.mtu < 1280 || self.mtu > 1500 {
            return Err("MTU must be between 1280 and 1500".to_string());
        }
        
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserProfile {
    pub username: String,
    pub email: String,
    pub subscription_tier: SubscriptionTier,
    pub subscription_expires: Option<chrono::DateTime<chrono::Utc>>,
    pub max_devices: u32,
    pub data_limit: Option<u64>,  // bytes per month, None = unlimited
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SubscriptionTier {
    Free,
    Premium,
    Business,
}

impl SubscriptionTier {
    pub fn name(&self) -> &str {
        match self {
            SubscriptionTier::Free => "Free",
            SubscriptionTier::Premium => "Premium",
            SubscriptionTier::Business => "Business",
        }
    }

    pub fn max_devices(&self) -> u32 {
        match self {
            SubscriptionTier::Free => 1,
            SubscriptionTier::Premium => 5,
            SubscriptionTier::Business => 10,
        }
    }

    pub fn has_feature(&self, feature: &str) -> bool {
        match self {
            SubscriptionTier::Free => matches!(feature, "basic_vpn"),
            SubscriptionTier::Premium => true,
            SubscriptionTier::Business => true,
        }
    }
}

impl Default for UserProfile {
    fn default() -> Self {
        Self {
            username: "user".to_string(),
            email: "user@example.com".to_string(),
            subscription_tier: SubscriptionTier::Free,
            subscription_expires: None,
            max_devices: 1,
            data_limit: Some(10 * 1024 * 1024 * 1024), // 10 GB
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = VpnConfig::default();
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_preset_configs() {
        let max_security = VpnConfig::preset_maximum_security();
        assert!(max_security.validate().is_ok());
        assert!(max_security.block_malware);
        
        let streaming = VpnConfig::preset_streaming();
        assert!(streaming.validate().is_ok());
        
        let torrenting = VpnConfig::preset_torrenting();
        assert!(torrenting.validate().is_ok());
    }

    #[test]
    fn test_config_validation() {
        let mut config = VpnConfig::default();
        config.connect_timeout = 2;
        assert!(config.validate().is_err());
        
        config.connect_timeout = 30;
        config.mtu = 1000;
        assert!(config.validate().is_err());
    }
}
