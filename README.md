# VPN Mobile - Rust Edition

A feature-rich, secure, and fast VPN mobile application built entirely in Rust.

## 🌟 Features

### Core VPN Features
- ✅ **Multi-Protocol Support**
  - WireGuard (Modern, fast, recommended)
  - OpenVPN (Industry standard, highly secure)
  - IKEv2/IPSec (Fast and stable for mobile)
  - L2TP/IPSec (Wide compatibility)
  - PPTP (Legacy support)

### 🌍 Global Server Network
- **25+ Countries** with multiple servers each
- 🇺🇸 United States (New York, Los Angeles, Chicago, Miami, Seattle)
- 🇬🇧 United Kingdom (London, Manchester, Edinburgh)
- 🇨🇦 Canada (Toronto, Montreal, Vancouver)
- 🇩🇪 Germany (Frankfurt, Berlin, Munich)
- 🇫🇷 France (Paris, Marseille, Lyon)
- 🇳🇱 Netherlands (Amsterdam, Rotterdam)
- 🇨🇭 Switzerland (Zurich, Geneva)
- 🇸🇪 Sweden (Stockholm, Gothenburg)
- 🇯🇵 Japan (Tokyo, Osaka, Kyoto)
- 🇸🇬 Singapore
- 🇦🇺 Australia (Sydney, Melbourne, Brisbane)
- 🇧🇷 Brazil (Sao Paulo, Rio de Janeiro)
- 🇮🇳 India (Mumbai, Delhi, Bangalore)
- 🇰🇷 South Korea (Seoul, Busan)
- 🇪🇸 Spain (Madrid, Barcelona)
- 🇮🇹 Italy (Rome, Milan)
- 🇳🇴 Norway (Oslo)
- 🇩🇰 Denmark (Copenhagen)
- 🇫🇮 Finland (Helsinki)
- 🇵🇱 Poland (Warsaw, Krakow)
- 🇹🇷 Turkey (Istanbul, Ankara)
- 🇦🇪 UAE (Dubai, Abu Dhabi)
- 🇿🇦 South Africa (Johannesburg, Cape Town)
- 🇲🇽 Mexico (Mexico City, Guadalajara)
- 🇦🇷 Argentina (Buenos Aires, Cordoba)

### 🔒 Security Features
- **Kill Switch** - Blocks internet if VPN disconnects
  - Automatic mode (blocks on disconnect)
  - Always-on mode (blocks all non-VPN traffic)
  - LAN traffic allowance
  - Per-app and per-IP exceptions
- **DNS Leak Protection** - Prevents DNS queries from leaking
- **IPv6 Leak Protection** - Blocks IPv6 traffic
- **WebRTC Leak Protection** - Prevents WebRTC leaks
- **AES-256 Encryption** - Military-grade encryption
- **Custom DNS Servers**
  - Cloudflare (1.1.1.1)
  - Google (8.8.8.8)
  - Quad9 (9.9.9.9)
  - OpenDNS
  - AdGuard DNS

### 🎯 Advanced Features
- **Split Tunneling**
  - Include mode (only selected apps use VPN)
  - Exclude mode (selected apps bypass VPN)
  - Domain-based routing
  - IP range routing
  - Preset configurations (Streaming, Banking, Torrenting)

- **Smart Server Selection**
  - Auto-connect to fastest server
  - Filter by features (P2P, Streaming, Premium)
  - Server load monitoring
  - Latency-based selection
  - Favorite servers
  - Recent connections

- **Privacy Features**
  - Ad blocking
  - Tracker blocking
  - Malware blocking
  - Anonymous usage statistics

### 📊 Analytics & Monitoring
- Real-time connection statistics
- Upload/download speed monitoring
- Data usage tracking (daily, monthly)
- Connection history
- Server usage statistics
- Packet loss monitoring
- Latency tracking

### ⚙️ Configuration
- Auto-connect on startup
- Start on boot
- Customizable reconnection attempts
- MTU settings
- Connection timeout configuration
- Notification preferences
- Preset configurations:
  - Maximum Security
  - Streaming Optimized
  - Torrenting

## 🚀 Installation

### Prerequisites
- Rust 1.70 or higher
- Cargo package manager

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/vpn-mobile-rust.git
cd vpn-mobile-rust

# Build the project
cargo build --release

# Run the application
cargo run --release
```

### Using the CLI Binary

```bash
# After building, the binary will be at:
./target/release/vpn-cli

# You can also install it globally:
cargo install --path .
```

## 📖 Usage

### Quick Start

```bash
# Run the application
./target/release/vpn-cli

# Or if installed globally
vpn-cli
```

### Main Menu Options

1. **Quick Connect** - Automatically connects to the fastest available server
2. **Select Server by Country** - Choose a specific country and server
3. **Disconnect** - Disconnect from the current VPN connection
4. **Connection Status** - View detailed connection information and statistics
5. **Server List** - Browse all available servers
6. **Protocol Settings** - Change VPN protocol
7. **Security Settings** - Configure kill switch, DNS protection
8. **Split Tunneling** - Configure app-based routing
9. **Statistics** - View usage statistics and history
10. **Settings** - Configure application preferences

### Example Usage

```bash
# Quick connect to fastest server
vpn-cli
# Select option 1

# Connect to specific country (e.g., United States)
vpn-cli
# Select option 2
# Choose country number

# View connection status
vpn-cli
# Select option 4
```

## 🔧 Configuration

Configuration file location:
- Linux: `~/.config/vpn-mobile/config.json`
- macOS: `~/Library/Application Support/VPN Mobile/config.json`
- Windows: `%APPDATA%\VPN Mobile\config.json`
- Android: `/data/data/com.vpn.mobile/files/config.json`
- iOS: `/var/mobile/Library/Application Support/VPN Mobile/config.json`

## 🛠️ Development

### Project Structure

```
vpn-mobile-rust/
├── src/
│   ├── lib.rs              # Main library file
│   ├── main.rs             # CLI application
│   ├── connection.rs       # VPN connection management
│   ├── server.rs           # Server management and selection
│   ├── protocol.rs         # VPN protocol definitions
│   ├── encryption.rs       # Encryption utilities
│   ├── dns.rs              # DNS leak protection
│   ├── killswitch.rs       # Kill switch functionality
│   ├── split_tunnel.rs     # Split tunneling
│   ├── analytics.rs        # Usage statistics
│   └── config.rs           # Configuration management
├── Cargo.toml
└── README.md
```

### Running Tests

```bash
# Run all tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_connection_lifecycle
```

## 🔐 Security Considerations

This is a demonstration VPN application. For production use, you should:

1. Implement actual VPN protocols using proper libraries
2. Add secure authentication mechanisms
3. Implement proper key exchange
4. Add certificate validation
5. Implement proper firewall rules for kill switch
6. Add secure storage for credentials
7. Implement proper error handling for network failures
8. Add rate limiting and DDoS protection

## 📝 License

MIT License - See LICENSE file for details

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 📧 Support

For issues, questions, or suggestions, please open an issue on GitHub.

## 🎉 Acknowledgments

Built with:
- Rust - Systems programming language
- Tokio - Async runtime
- Serde - Serialization framework
- AES-GCM - Encryption library

## 🗺️ Roadmap

- [ ] Mobile app UI (iOS/Android)
- [ ] Desktop GUI (using Tauri or Iced)
- [ ] Actual protocol implementations
- [ ] Multi-hop connections
- [ ] Obfuscation techniques
- [ ] P2P support
- [ ] Port forwarding
- [ ] SOCKS5 proxy
- [ ] Shadowsocks support
- [ ] WireGuard/OpenVPN config import/export
