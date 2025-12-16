#!/bin/bash
# Example usage script for VPN Mobile

echo "VPN Mobile - Example Usage"
echo "=========================="
echo ""

# Check if vpn-cli is available
if ! command -v cargo &> /dev/null; then
    echo "Error: Cargo not found. Please install Rust first."
    echo "Visit: https://rustup.rs/"
    exit 1
fi

# Build the project if not already built
if [ ! -f "target/release/vpn-cli" ]; then
    echo "Building VPN Mobile..."
    cargo build --release
    echo ""
fi

echo "Starting VPN Mobile CLI..."
echo ""
echo "Available commands once running:"
echo "  1. Quick Connect - Connect to fastest server"
echo "  2. Select Country - Choose specific country"
echo "  3. Disconnect - Disconnect from VPN"
echo "  4. Status - View connection status"
echo "  5. Server List - Browse all servers"
echo "  6. Protocol Settings - Change VPN protocol"
echo "  7. Security - Configure kill switch and DNS"
echo "  8. Split Tunneling - Configure app routing"
echo "  9. Statistics - View usage stats"
echo "  10. Settings - General settings"
echo ""
echo "Press Enter to start..."
read

# Run the application
cargo run --release
