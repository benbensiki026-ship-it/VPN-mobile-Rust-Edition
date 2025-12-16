#!/bin/bash

# VPN Mobile - Build and Package Script
# This script builds the project and creates a distributable archive

set -e

echo "╔═══════════════════════════════════════════╗"
echo "║   VPN Mobile - Build & Package Script    ║"
echo "╚═══════════════════════════════════════════╝"
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if Rust is installed
if ! command -v cargo &> /dev/null; then
    echo -e "${RED}Error: Cargo is not installed.${NC}"
    echo "Please install Rust from https://rustup.rs/"
    exit 1
fi

echo -e "${GREEN}✓${NC} Rust/Cargo found"

# Clean previous builds
echo ""
echo "🧹 Cleaning previous builds..."
cargo clean
echo -e "${GREEN}✓${NC} Clean complete"

# Run tests
echo ""
echo "🧪 Running tests..."
if cargo test --quiet; then
    echo -e "${GREEN}✓${NC} All tests passed"
else
    echo -e "${RED}✗${NC} Some tests failed"
    echo "Do you want to continue anyway? (y/n)"
    read -r response
    if [[ ! "$response" =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Build in release mode
echo ""
echo "🔨 Building in release mode..."
if cargo build --release; then
    echo -e "${GREEN}✓${NC} Build successful"
else
    echo -e "${RED}✗${NC} Build failed"
    exit 1
fi

# Get version from Cargo.toml
VERSION=$(grep "^version" Cargo.toml | head -1 | cut -d'"' -f2)
echo ""
echo "📦 Package version: $VERSION"

# Create package directory
PACKAGE_NAME="vpn-mobile-rust-v${VERSION}"
PACKAGE_DIR="${PACKAGE_NAME}"

echo ""
echo "📁 Creating package directory: ${PACKAGE_DIR}"
rm -rf "${PACKAGE_DIR}"
mkdir -p "${PACKAGE_DIR}"
mkdir -p "${PACKAGE_DIR}/bin"
mkdir -p "${PACKAGE_DIR}/docs"
mkdir -p "${PACKAGE_DIR}/config"

# Copy binary
echo "📋 Copying binary..."
cp target/release/vpn-cli "${PACKAGE_DIR}/bin/"
chmod +x "${PACKAGE_DIR}/bin/vpn-cli"

# Copy documentation
echo "📋 Copying documentation..."
cp README.md "${PACKAGE_DIR}/docs/"
cp Cargo.toml "${PACKAGE_DIR}/docs/"

# Create default config
echo "📋 Creating default configuration..."
cat > "${PACKAGE_DIR}/config/default-config.json" << 'EOF'
{
  "auto_connect": false,
  "auto_connect_server": null,
  "start_on_boot": false,
  "minimize_to_tray": true,
  "protocol_config": {
    "protocol": "WireGuard",
    "port": 51820,
    "use_tcp": false,
    "obfuscation": false,
    "mtu": 1420
  },
  "kill_switch": {
    "mode": "Automatic",
    "allow_lan": true,
    "allowed_ips": [],
    "allowed_apps": []
  },
  "dns_mode": "Auto",
  "ipv6_leak_protection": true,
  "webrtc_leak_protection": true,
  "split_tunnel": {
    "mode": "Disabled",
    "apps": [],
    "domains": [],
    "ip_ranges": []
  },
  "mtu": 1420,
  "connect_timeout": 30,
  "reconnect_on_disconnect": true,
  "reconnect_attempts": 3,
  "block_trackers": false,
  "block_ads": false,
  "block_malware": true,
  "anonymous_usage_stats": false,
  "show_notifications": true,
  "notify_on_connect": true,
  "notify_on_disconnect": true,
  "notify_on_ip_change": false
}
EOF

# Create installation script
echo "📋 Creating installation script..."
cat > "${PACKAGE_DIR}/install.sh" << 'INSTALL_EOF'
#!/bin/bash

echo "Installing VPN Mobile..."

# Check if running as root
if [ "$EUID" -ne 0 ]; then 
    echo "Please run as root (use sudo)"
    exit 1
fi

# Create installation directory
INSTALL_DIR="/opt/vpn-mobile"
mkdir -p "$INSTALL_DIR"

# Copy binary
cp bin/vpn-cli "$INSTALL_DIR/"
chmod +x "$INSTALL_DIR/vpn-cli"

# Create symlink
ln -sf "$INSTALL_DIR/vpn-cli" /usr/local/bin/vpn-cli

# Create config directory
CONFIG_DIR="$HOME/.config/vpn-mobile"
mkdir -p "$CONFIG_DIR"

# Copy default config if not exists
if [ ! -f "$CONFIG_DIR/config.json" ]; then
    cp config/default-config.json "$CONFIG_DIR/config.json"
fi

echo "✓ Installation complete!"
echo "Run 'vpn-cli' to start the VPN application"
INSTALL_EOF

chmod +x "${PACKAGE_DIR}/install.sh"

# Create usage guide
echo "📋 Creating usage guide..."
cat > "${PACKAGE_DIR}/USAGE.txt" << 'USAGE_EOF'
VPN Mobile - Quick Start Guide
================================

RUNNING THE APPLICATION:
------------------------
1. Navigate to the bin/ directory
2. Run: ./vpn-cli

Or after installation:
   vpn-cli

MAIN FEATURES:
--------------
1. Quick Connect - Auto-connect to fastest server
2. Server Selection - Choose specific country/server
3. Protocol Support - WireGuard, OpenVPN, IKEv2, L2TP, PPTP
4. Kill Switch - Protect against connection drops
5. DNS Protection - Prevent DNS leaks
6. Split Tunneling - Route specific apps through VPN
7. Statistics - Track usage and performance

CONFIGURATION:
--------------
Config file location:
  Linux: ~/.config/vpn-mobile/config.json
  macOS: ~/Library/Application Support/VPN Mobile/config.json

SECURITY FEATURES:
------------------
- AES-256 encryption
- DNS leak protection
- IPv6 leak protection
- WebRTC leak protection
- Internet kill switch
- Custom DNS servers

For more information, see docs/README.md
USAGE_EOF

# Create archive
echo ""
echo "📦 Creating archive..."
ARCHIVE_NAME="${PACKAGE_NAME}.tar.gz"

tar -czf "${ARCHIVE_NAME}" "${PACKAGE_DIR}"

# Calculate size
SIZE=$(du -h "${ARCHIVE_NAME}" | cut -f1)

echo -e "${GREEN}✓${NC} Archive created: ${ARCHIVE_NAME}"
echo "   Size: ${SIZE}"

# Create checksum
echo ""
echo "🔐 Generating checksum..."
sha256sum "${ARCHIVE_NAME}" > "${ARCHIVE_NAME}.sha256"
echo -e "${GREEN}✓${NC} Checksum saved to: ${ARCHIVE_NAME}.sha256"

# Create zip archive as well
echo ""
echo "📦 Creating zip archive..."
ZIP_NAME="${PACKAGE_NAME}.zip"
zip -r -q "${ZIP_NAME}" "${PACKAGE_DIR}"
ZIP_SIZE=$(du -h "${ZIP_NAME}" | cut -f1)
echo -e "${GREEN}✓${NC} Zip archive created: ${ZIP_NAME}"
echo "   Size: ${ZIP_SIZE}"

# Summary
echo ""
echo "╔═══════════════════════════════════════════╗"
echo "║          BUILD SUMMARY                    ║"
echo "╚═══════════════════════════════════════════╝"
echo ""
echo "Package: ${PACKAGE_NAME}"
echo "Version: ${VERSION}"
echo "Archives:"
echo "  • ${ARCHIVE_NAME} (${SIZE})"
echo "  • ${ZIP_NAME} (${ZIP_SIZE})"
echo "  • ${ARCHIVE_NAME}.sha256"
echo ""
echo -e "${GREEN}✓ Build and packaging complete!${NC}"
echo ""
echo "To extract and install:"
echo "  tar -xzf ${ARCHIVE_NAME}"
echo "  cd ${PACKAGE_DIR}"
echo "  sudo ./install.sh"
echo ""
echo "Or run directly:"
echo "  ./${PACKAGE_DIR}/bin/vpn-cli"
