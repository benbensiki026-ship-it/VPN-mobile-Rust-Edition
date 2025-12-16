use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use crate::{Result, VpnError};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum SplitTunnelMode {
    Disabled,
    IncludeOnly,  // Only selected apps use VPN
    ExcludeOnly,  // Selected apps bypass VPN
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SplitTunnelConfig {
    pub mode: SplitTunnelMode,
    pub apps: HashSet<String>,
    pub domains: HashSet<String>,
    pub ip_ranges: HashSet<String>,
}

impl Default for SplitTunnelConfig {
    fn default() -> Self {
        Self {
            mode: SplitTunnelMode::Disabled,
            apps: HashSet::new(),
            domains: HashSet::new(),
            ip_ranges: HashSet::new(),
        }
    }
}

pub struct SplitTunnel {
    config: SplitTunnelConfig,
    active: bool,
}

impl SplitTunnel {
    pub fn new(config: SplitTunnelConfig) -> Self {
        Self {
            config,
            active: false,
        }
    }

    pub fn enable(&mut self) -> Result<()> {
        if self.config.mode == SplitTunnelMode::Disabled {
            return Err(VpnError::ConfigError(
                "Split tunneling is disabled in configuration".to_string()
            ));
        }

        log::info!("Enabling split tunneling (mode: {:?})", self.config.mode);
        
        self.apply_routing_rules()?;
        self.active = true;
        
        log::info!("Split tunneling enabled successfully");
        Ok(())
    }

    pub fn disable(&mut self) -> Result<()> {
        log::info!("Disabling split tunneling");
        
        self.remove_routing_rules()?;
        self.active = false;
        
        log::info!("Split tunneling disabled successfully");
        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_mode(&mut self, mode: SplitTunnelMode) {
        self.config.mode = mode;
    }

    pub fn get_mode(&self) -> SplitTunnelMode {
        self.config.mode
    }

    // App management
    pub fn add_app(&mut self, app: String) {
        self.config.apps.insert(app);
    }

    pub fn remove_app(&mut self, app: &str) {
        self.config.apps.remove(app);
    }

    pub fn get_apps(&self) -> Vec<String> {
        self.config.apps.iter().cloned().collect()
    }

    pub fn clear_apps(&mut self) {
        self.config.apps.clear();
    }

    // Domain management
    pub fn add_domain(&mut self, domain: String) {
        self.config.domains.insert(domain);
    }

    pub fn remove_domain(&mut self, domain: &str) {
        self.config.domains.remove(domain);
    }

    pub fn get_domains(&self) -> Vec<String> {
        self.config.domains.iter().cloned().collect()
    }

    // IP range management
    pub fn add_ip_range(&mut self, ip_range: String) {
        self.config.ip_ranges.insert(ip_range);
    }

    pub fn remove_ip_range(&mut self, ip_range: &str) {
        self.config.ip_ranges.remove(ip_range);
    }

    pub fn get_ip_ranges(&self) -> Vec<String> {
        self.config.ip_ranges.iter().cloned().collect()
    }

    fn apply_routing_rules(&self) -> Result<()> {
        log::info!("Applying split tunnel routing rules");
        
        match self.config.mode {
            SplitTunnelMode::Disabled => {
                return Ok(());
            }
            SplitTunnelMode::IncludeOnly => {
                log::info!("Mode: Only selected traffic uses VPN");
                log::info!("Apps using VPN: {:?}", self.config.apps);
            }
            SplitTunnelMode::ExcludeOnly => {
                log::info!("Mode: Selected traffic bypasses VPN");
                log::info!("Apps bypassing VPN: {:?}", self.config.apps);
            }
        }
        
        // Route domains
        if !self.config.domains.is_empty() {
            log::info!("Routing domains: {:?}", self.config.domains);
        }
        
        // Route IP ranges
        if !self.config.ip_ranges.is_empty() {
            log::info!("Routing IP ranges: {:?}", self.config.ip_ranges);
        }
        
        Ok(())
    }

    fn remove_routing_rules(&self) -> Result<()> {
        log::info!("Removing split tunnel routing rules");
        Ok(())
    }

    pub fn should_route_through_vpn(&self, app: &str) -> bool {
        if !self.active || self.config.mode == SplitTunnelMode::Disabled {
            return true; // Default: route everything through VPN
        }

        match self.config.mode {
            SplitTunnelMode::IncludeOnly => {
                // Only selected apps use VPN
                self.config.apps.contains(app)
            }
            SplitTunnelMode::ExcludeOnly => {
                // Selected apps bypass VPN
                !self.config.apps.contains(app)
            }
            SplitTunnelMode::Disabled => true,
        }
    }

    pub fn get_preset_configs() -> Vec<PresetConfig> {
        vec![
            PresetConfig {
                name: "Streaming Apps".to_string(),
                description: "Route streaming apps outside VPN for better performance".to_string(),
                mode: SplitTunnelMode::ExcludeOnly,
                apps: vec![
                    "Netflix".to_string(),
                    "Disney+".to_string(),
                    "Hulu".to_string(),
                    "Amazon Prime".to_string(),
                    "YouTube".to_string(),
                ],
            },
            PresetConfig {
                name: "Banking Apps".to_string(),
                description: "Keep banking apps outside VPN to avoid location issues".to_string(),
                mode: SplitTunnelMode::ExcludeOnly,
                apps: vec![
                    "Banking App".to_string(),
                    "PayPal".to_string(),
                    "Venmo".to_string(),
                ],
            },
            PresetConfig {
                name: "Torrent Only".to_string(),
                description: "Only route torrent traffic through VPN".to_string(),
                mode: SplitTunnelMode::IncludeOnly,
                apps: vec![
                    "qBittorrent".to_string(),
                    "Transmission".to_string(),
                    "uTorrent".to_string(),
                ],
            },
            PresetConfig {
                name: "Local Network".to_string(),
                description: "Exclude local network ranges".to_string(),
                mode: SplitTunnelMode::ExcludeOnly,
                apps: vec![],
            },
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresetConfig {
    pub name: String,
    pub description: String,
    pub mode: SplitTunnelMode,
    pub apps: Vec<String>,
}

impl PresetConfig {
    pub fn apply_to(&self, split_tunnel: &mut SplitTunnel) {
        split_tunnel.set_mode(self.mode);
        split_tunnel.clear_apps();
        for app in &self.apps {
            split_tunnel.add_app(app.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_tunnel_include_mode() {
        let mut config = SplitTunnelConfig::default();
        config.mode = SplitTunnelMode::IncludeOnly;
        
        let mut split_tunnel = SplitTunnel::new(config);
        split_tunnel.add_app("Firefox".to_string());
        split_tunnel.add_app("Chrome".to_string());
        split_tunnel.enable().unwrap();
        
        assert!(split_tunnel.should_route_through_vpn("Firefox"));
        assert!(split_tunnel.should_route_through_vpn("Chrome"));
        assert!(!split_tunnel.should_route_through_vpn("Safari"));
    }

    #[test]
    fn test_split_tunnel_exclude_mode() {
        let mut config = SplitTunnelConfig::default();
        config.mode = SplitTunnelMode::ExcludeOnly;
        
        let mut split_tunnel = SplitTunnel::new(config);
        split_tunnel.add_app("Netflix".to_string());
        split_tunnel.enable().unwrap();
        
        assert!(!split_tunnel.should_route_through_vpn("Netflix"));
        assert!(split_tunnel.should_route_through_vpn("Chrome"));
    }

    #[test]
    fn test_preset_configs() {
        let presets = SplitTunnel::get_preset_configs();
        assert!(presets.len() >= 4);
        
        let streaming_preset = &presets[0];
        assert_eq!(streaming_preset.name, "Streaming Apps");
        assert!(!streaming_preset.apps.is_empty());
    }
}
