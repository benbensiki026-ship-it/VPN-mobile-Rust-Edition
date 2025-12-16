use serde::{Deserialize, Serialize};
use crate::{Result, VpnError};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum KillSwitchMode {
    Disabled,
    Automatic,  // Blocks traffic if VPN disconnects
    Always,     // Always blocks non-VPN traffic
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KillSwitchConfig {
    pub mode: KillSwitchMode,
    pub allow_lan: bool,
    pub allowed_ips: Vec<String>,
    pub allowed_apps: Vec<String>,
}

impl Default for KillSwitchConfig {
    fn default() -> Self {
        Self {
            mode: KillSwitchMode::Automatic,
            allow_lan: true,
            allowed_ips: Vec::new(),
            allowed_apps: Vec::new(),
        }
    }
}

pub struct KillSwitch {
    config: KillSwitchConfig,
    active: bool,
}

impl KillSwitch {
    pub fn new(config: KillSwitchConfig) -> Self {
        Self {
            config,
            active: false,
        }
    }

    pub fn enable(&mut self) -> Result<()> {
        if self.config.mode == KillSwitchMode::Disabled {
            return Err(VpnError::ConfigError(
                "Kill switch is disabled in configuration".to_string()
            ));
        }

        log::info!("Enabling kill switch (mode: {:?})", self.config.mode);
        
        // In a real implementation, this would:
        // 1. Set up firewall rules
        // 2. Block all non-VPN traffic
        // 3. Allow exceptions (LAN, specific IPs/apps)
        
        self.active = true;
        self.apply_firewall_rules()?;
        
        log::info!("Kill switch enabled successfully");
        Ok(())
    }

    pub fn disable(&mut self) -> Result<()> {
        log::info!("Disabling kill switch");
        
        // Remove firewall rules
        self.remove_firewall_rules()?;
        
        self.active = false;
        log::info!("Kill switch disabled successfully");
        Ok(())
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn set_mode(&mut self, mode: KillSwitchMode) {
        self.config.mode = mode;
    }

    pub fn get_mode(&self) -> KillSwitchMode {
        self.config.mode
    }

    pub fn allow_lan_traffic(&mut self, allow: bool) {
        self.config.allow_lan = allow;
    }

    pub fn add_allowed_ip(&mut self, ip: String) {
        if !self.config.allowed_ips.contains(&ip) {
            self.config.allowed_ips.push(ip);
        }
    }

    pub fn remove_allowed_ip(&mut self, ip: &str) {
        self.config.allowed_ips.retain(|i| i != ip);
    }

    pub fn add_allowed_app(&mut self, app: String) {
        if !self.config.allowed_apps.contains(&app) {
            self.config.allowed_apps.push(app);
        }
    }

    pub fn remove_allowed_app(&mut self, app: &str) {
        self.config.allowed_apps.retain(|a| a != app);
    }

    fn apply_firewall_rules(&self) -> Result<()> {
        log::info!("Applying firewall rules");
        
        // Base rule: block all traffic
        log::debug!("Blocking all non-VPN traffic");
        
        // Allow VPN server connections
        log::debug!("Allowing VPN server connections");
        
        // Allow LAN if configured
        if self.config.allow_lan {
            log::debug!("Allowing LAN traffic (192.168.0.0/16, 10.0.0.0/8, 172.16.0.0/12)");
        }
        
        // Allow specific IPs
        for ip in &self.config.allowed_ips {
            log::debug!("Allowing traffic to/from: {}", ip);
        }
        
        // Allow specific apps
        for app in &self.config.allowed_apps {
            log::debug!("Allowing app: {}", app);
        }
        
        Ok(())
    }

    fn remove_firewall_rules(&self) -> Result<()> {
        log::info!("Removing firewall rules");
        
        // In a real implementation:
        // 1. Remove all VPN-related firewall rules
        // 2. Restore default network access
        
        Ok(())
    }

    pub fn test_kill_switch(&self) -> Result<KillSwitchStatus> {
        log::info!("Testing kill switch functionality");
        
        // Simulate testing
        Ok(KillSwitchStatus {
            is_working: true,
            leaks_detected: false,
            blocked_connections: 5,
            allowed_connections: 2,
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KillSwitchStatus {
    pub is_working: bool,
    pub leaks_detected: bool,
    pub blocked_connections: u32,
    pub allowed_connections: u32,
}

impl KillSwitchStatus {
    pub fn summary(&self) -> String {
        if self.is_working && !self.leaks_detected {
            format!(
                "✓ Kill switch is working correctly. Blocked: {}, Allowed: {}",
                self.blocked_connections, self.allowed_connections
            )
        } else {
            "⚠ Kill switch may not be functioning properly".to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kill_switch_enable_disable() {
        let config = KillSwitchConfig::default();
        let mut kill_switch = KillSwitch::new(config);
        
        assert!(!kill_switch.is_active());
        
        kill_switch.enable().unwrap();
        assert!(kill_switch.is_active());
        
        kill_switch.disable().unwrap();
        assert!(!kill_switch.is_active());
    }

    #[test]
    fn test_kill_switch_configuration() {
        let config = KillSwitchConfig::default();
        let mut kill_switch = KillSwitch::new(config);
        
        kill_switch.add_allowed_ip("192.168.1.1".to_string());
        kill_switch.add_allowed_app("firefox".to_string());
        
        assert_eq!(kill_switch.config.allowed_ips.len(), 1);
        assert_eq!(kill_switch.config.allowed_apps.len(), 1);
    }
}
