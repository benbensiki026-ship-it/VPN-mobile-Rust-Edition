use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum VpnProtocol {
    OpenVPN,
    WireGuard,
    IKEv2,
    L2TP,
    PPTP,
}

impl VpnProtocol {
    pub fn name(&self) -> &str {
        match self {
            VpnProtocol::OpenVPN => "OpenVPN",
            VpnProtocol::WireGuard => "WireGuard",
            VpnProtocol::IKEv2 => "IKEv2/IPSec",
            VpnProtocol::L2TP => "L2TP/IPSec",
            VpnProtocol::PPTP => "PPTP",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            VpnProtocol::OpenVPN => "Industry standard, highly secure and reliable",
            VpnProtocol::WireGuard => "Modern, fast, and efficient protocol",
            VpnProtocol::IKEv2 => "Fast and stable, great for mobile",
            VpnProtocol::L2TP => "Good security with wide compatibility",
            VpnProtocol::PPTP => "Legacy protocol, fast but less secure",
        }
    }

    pub fn default_port(&self) -> u16 {
        match self {
            VpnProtocol::OpenVPN => 1194,
            VpnProtocol::WireGuard => 51820,
            VpnProtocol::IKEv2 => 500,
            VpnProtocol::L2TP => 1701,
            VpnProtocol::PPTP => 1723,
        }
    }

    pub fn is_recommended(&self) -> bool {
        matches!(self, VpnProtocol::WireGuard | VpnProtocol::OpenVPN)
    }

    pub fn security_level(&self) -> u8 {
        match self {
            VpnProtocol::WireGuard => 10,
            VpnProtocol::OpenVPN => 9,
            VpnProtocol::IKEv2 => 8,
            VpnProtocol::L2TP => 7,
            VpnProtocol::PPTP => 4,
        }
    }

    pub fn speed_rating(&self) -> u8 {
        match self {
            VpnProtocol::WireGuard => 10,
            VpnProtocol::PPTP => 9,
            VpnProtocol::IKEv2 => 8,
            VpnProtocol::OpenVPN => 7,
            VpnProtocol::L2TP => 6,
        }
    }

    pub fn all() -> Vec<VpnProtocol> {
        vec![
            VpnProtocol::WireGuard,
            VpnProtocol::OpenVPN,
            VpnProtocol::IKEv2,
            VpnProtocol::L2TP,
            VpnProtocol::PPTP,
        ]
    }
}

impl Default for VpnProtocol {
    fn default() -> Self {
        VpnProtocol::WireGuard
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolConfig {
    pub protocol: VpnProtocol,
    pub port: u16,
    pub use_tcp: bool,
    pub obfuscation: bool,
    pub mtu: u16,
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            protocol: VpnProtocol::default(),
            port: VpnProtocol::default().default_port(),
            use_tcp: false,
            obfuscation: false,
            mtu: 1420,
        }
    }
}

impl ProtocolConfig {
    pub fn new(protocol: VpnProtocol) -> Self {
        Self {
            protocol,
            port: protocol.default_port(),
            use_tcp: false,
            obfuscation: false,
            mtu: 1420,
        }
    }

    pub fn with_obfuscation(mut self, enabled: bool) -> Self {
        self.obfuscation = enabled;
        self
    }

    pub fn with_tcp(mut self, use_tcp: bool) -> Self {
        self.use_tcp = use_tcp;
        self
    }

    pub fn with_mtu(mut self, mtu: u16) -> Self {
        self.mtu = mtu;
        self
    }
}
