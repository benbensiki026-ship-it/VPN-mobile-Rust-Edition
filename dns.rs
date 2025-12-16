use serde::{Deserialize, Serialize};
use crate::{Result, VpnError};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsServer {
    pub name: String,
    pub primary: String,
    pub secondary: Option<String>,
    pub supports_dnssec: bool,
    pub supports_doh: bool,  // DNS over HTTPS
    pub supports_dot: bool,  // DNS over TLS
}

impl DnsServer {
    pub fn cloudflare() -> Self {
        Self {
            name: "Cloudflare".to_string(),
            primary: "1.1.1.1".to_string(),
            secondary: Some("1.0.0.1".to_string()),
            supports_dnssec: true,
            supports_doh: true,
            supports_dot: true,
        }
    }

    pub fn google() -> Self {
        Self {
            name: "Google".to_string(),
            primary: "8.8.8.8".to_string(),
            secondary: Some("8.8.4.4".to_string()),
            supports_dnssec: true,
            supports_doh: true,
            supports_dot: true,
        }
    }

    pub fn quad9() -> Self {
        Self {
            name: "Quad9".to_string(),
            primary: "9.9.9.9".to_string(),
            secondary: Some("149.112.112.112".to_string()),
            supports_dnssec: true,
            supports_doh: true,
            supports_dot: true,
        }
    }

    pub fn opendns() -> Self {
        Self {
            name: "OpenDNS".to_string(),
            primary: "208.67.222.222".to_string(),
            secondary: Some("208.67.220.220".to_string()),
            supports_dnssec: true,
            supports_doh: false,
            supports_dot: false,
        }
    }

    pub fn adguard() -> Self {
        Self {
            name: "AdGuard DNS".to_string(),
            primary: "94.140.14.14".to_string(),
            secondary: Some("94.140.15.15".to_string()),
            supports_dnssec: true,
            supports_doh: true,
            supports_dot: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DnsMode {
    Auto,              // Use VPN server's DNS
    Custom(DnsServer), // Use custom DNS servers
    System,            // Use system DNS (leak risk)
}

pub struct DnsManager {
    mode: DnsMode,
    leak_protection: bool,
    dns_filtering: bool,
    block_malware: bool,
    block_ads: bool,
}

impl DnsManager {
    pub fn new() -> Self {
        Self {
            mode: DnsMode::Auto,
            leak_protection: true,
            dns_filtering: false,
            block_malware: true,
            block_ads: false,
        }
    }

    pub fn set_mode(&mut self, mode: DnsMode) {
        self.mode = mode;
    }

    pub fn get_mode(&self) -> &DnsMode {
        &self.mode
    }

    pub fn enable_leak_protection(&mut self, enabled: bool) {
        self.leak_protection = enabled;
    }

    pub fn enable_dns_filtering(&mut self, enabled: bool) {
        self.dns_filtering = enabled;
    }

    pub fn enable_malware_blocking(&mut self, enabled: bool) {
        self.block_malware = enabled;
    }

    pub fn enable_ad_blocking(&mut self, enabled: bool) {
        self.block_ads = enabled;
    }

    pub fn is_leak_protected(&self) -> bool {
        self.leak_protection
    }

    pub async fn check_dns_leak(&self) -> Result<DnsLeakTest> {
        log::info!("Performing DNS leak test...");
        
        // Simulate DNS leak test
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        
        Ok(DnsLeakTest {
            is_leaking: false,
            detected_servers: vec![
                "1.1.1.1".to_string(),
                "1.0.0.1".to_string(),
            ],
            isp_detected: false,
            location_matches_vpn: true,
        })
    }

    pub fn get_available_dns_servers() -> Vec<DnsServer> {
        vec![
            DnsServer::cloudflare(),
            DnsServer::google(),
            DnsServer::quad9(),
            DnsServer::opendns(),
            DnsServer::adguard(),
        ]
    }

    pub fn apply_dns_configuration(&self) -> Result<()> {
        log::info!("Applying DNS configuration: {:?}", self.mode);
        
        // In a real implementation, this would:
        // 1. Configure system DNS settings
        // 2. Set up DNS leak protection
        // 3. Configure DNS filtering rules
        
        if self.leak_protection {
            log::info!("DNS leak protection enabled");
        }
        
        if self.dns_filtering {
            log::info!("DNS filtering enabled");
        }
        
        if self.block_malware {
            log::info!("Malware blocking enabled");
        }
        
        if self.block_ads {
            log::info!("Ad blocking enabled");
        }
        
        Ok(())
    }
}

impl Default for DnsManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DnsLeakTest {
    pub is_leaking: bool,
    pub detected_servers: Vec<String>,
    pub isp_detected: bool,
    pub location_matches_vpn: bool,
}

impl DnsLeakTest {
    pub fn is_secure(&self) -> bool {
        !self.is_leaking && !self.isp_detected && self.location_matches_vpn
    }

    pub fn summary(&self) -> String {
        if self.is_secure() {
            "✓ Your DNS is secure. No leaks detected.".to_string()
        } else {
            let mut issues = Vec::new();
            if self.is_leaking {
                issues.push("DNS leak detected");
            }
            if self.isp_detected {
                issues.push("ISP DNS detected");
            }
            if !self.location_matches_vpn {
                issues.push("Location mismatch");
            }
            format!("⚠ Issues found: {}", issues.join(", "))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dns_leak_check() {
        let manager = DnsManager::new();
        let result = manager.check_dns_leak().await.unwrap();
        assert!(result.is_secure());
    }

    #[test]
    fn test_dns_servers() {
        let cloudflare = DnsServer::cloudflare();
        assert_eq!(cloudflare.primary, "1.1.1.1");
        assert!(cloudflare.supports_doh);
        
        let servers = DnsManager::get_available_dns_servers();
        assert!(servers.len() >= 5);
    }
}
