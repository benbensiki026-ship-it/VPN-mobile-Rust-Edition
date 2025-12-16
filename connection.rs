use crate::{ConnectionInfo, ConnectionStatus, Result, VpnError, VpnServer, VpnStats};
use crate::protocol::ProtocolConfig;
use chrono::Utc;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::Duration;

pub struct VpnConnection {
    info: Arc<RwLock<ConnectionInfo>>,
    stats: Arc<RwLock<VpnStats>>,
    protocol_config: ProtocolConfig,
}

impl VpnConnection {
    pub fn new(protocol_config: ProtocolConfig) -> Self {
        Self {
            info: Arc::new(RwLock::new(ConnectionInfo {
                status: ConnectionStatus::Disconnected,
                server: None,
                connected_at: None,
                bytes_sent: 0,
                bytes_received: 0,
                duration: Duration::from_secs(0),
                ip_address: None,
            })),
            stats: Arc::new(RwLock::new(VpnStats {
                current_speed_up: 0.0,
                current_speed_down: 0.0,
                total_upload: 0,
                total_download: 0,
                latency: 0,
                packet_loss: 0.0,
            })),
            protocol_config,
        }
    }

    pub async fn connect(&self, server: VpnServer) -> Result<()> {
        // Update status to connecting
        {
            let mut info = self.info.write().await;
            info.status = ConnectionStatus::Connecting;
            info.server = Some(server.clone());
        }

        // Simulate connection process
        log::info!("Connecting to {} using {:?}", server.name, self.protocol_config.protocol);
        
        // In a real implementation, this would:
        // 1. Establish network connection
        // 2. Perform handshake
        // 3. Set up encryption
        // 4. Configure routing
        
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Update status to connected
        {
            let mut info = self.info.write().await;
            info.status = ConnectionStatus::Connected;
            info.connected_at = Some(Utc::now());
            info.ip_address = Some(format!("10.8.{}.{}", rand::random::<u8>(), rand::random::<u8>()));
        }

        log::info!("Successfully connected to {}", server.name);
        Ok(())
    }

    pub async fn disconnect(&self) -> Result<()> {
        {
            let mut info = self.info.write().await;
            if matches!(info.status, ConnectionStatus::Disconnected) {
                return Ok(());
            }
            info.status = ConnectionStatus::Disconnecting;
        }

        log::info!("Disconnecting from VPN");
        
        // Simulate disconnection
        tokio::time::sleep(Duration::from_secs(1)).await;

        {
            let mut info = self.info.write().await;
            info.status = ConnectionStatus::Disconnected;
            info.server = None;
            info.connected_at = None;
            info.ip_address = None;
        }

        log::info!("Disconnected successfully");
        Ok(())
    }

    pub async fn reconnect(&self) -> Result<()> {
        let server = {
            let info = self.info.read().await;
            info.server.clone()
        };

        if let Some(server) = server {
            {
                let mut info = self.info.write().await;
                info.status = ConnectionStatus::Reconnecting;
            }

            log::info!("Reconnecting to VPN");
            self.disconnect().await?;
            tokio::time::sleep(Duration::from_millis(500)).await;
            self.connect(server).await?;
            Ok(())
        } else {
            Err(VpnError::ConnectionFailed("No server to reconnect to".to_string()))
        }
    }

    pub async fn get_info(&self) -> ConnectionInfo {
        let info = self.info.read().await;
        let mut info_clone = info.clone();
        
        // Update duration if connected
        if let Some(connected_at) = info.connected_at {
            info_clone.duration = (Utc::now() - connected_at)
                .to_std()
                .unwrap_or(Duration::from_secs(0));
        }
        
        info_clone
    }

    pub async fn get_stats(&self) -> VpnStats {
        self.stats.read().await.clone()
    }

    pub async fn update_stats(&self) {
        // Simulate real-time stats updates
        let mut stats = self.stats.write().await;
        
        // Simulate traffic
        stats.current_speed_up = rand::random::<f64>() * 10.0;
        stats.current_speed_down = rand::random::<f64>() * 50.0;
        stats.total_upload += (stats.current_speed_up * 1024.0 * 1024.0) as u64;
        stats.total_download += (stats.current_speed_down * 1024.0 * 1024.0) as u64;
        stats.latency = 20 + rand::random::<u32>() % 50;
        stats.packet_loss = rand::random::<f32>() * 0.5;

        // Update connection info
        let mut info = self.info.write().await;
        info.bytes_sent = stats.total_upload;
        info.bytes_received = stats.total_download;
    }

    pub async fn is_connected(&self) -> bool {
        let info = self.info.read().await;
        matches!(info.status, ConnectionStatus::Connected)
    }

    pub fn get_protocol_config(&self) -> &ProtocolConfig {
        &self.protocol_config
    }

    pub fn set_protocol_config(&mut self, config: ProtocolConfig) {
        self.protocol_config = config;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::server::{Country, ServerLocation};

    #[tokio::test]
    async fn test_connection_lifecycle() {
        let config = ProtocolConfig::default();
        let connection = VpnConnection::new(config);

        let server = VpnServer {
            id: "test-1".to_string(),
            name: "Test Server".to_string(),
            location: ServerLocation {
                city: "Test City".to_string(),
                country: Country::UnitedStates,
                latitude: 0.0,
                longitude: 0.0,
            },
            host: "test.vpn.com".to_string(),
            port: 443,
            load: 50,
            latency: 30,
            bandwidth: 1000,
            is_premium: false,
            supports_p2p: true,
            supports_streaming: true,
            online: true,
        };

        // Test connection
        assert!(connection.connect(server).await.is_ok());
        assert!(connection.is_connected().await);

        // Test disconnection
        assert!(connection.disconnect().await.is_ok());
        assert!(!connection.is_connected().await);
    }
}
