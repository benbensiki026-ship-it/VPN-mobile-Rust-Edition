use vpn_mobile::*;
use connection::VpnConnection;
use server::{ServerManager, Country};
use protocol::{VpnProtocol, ProtocolConfig};
use config::VpnConfig;
use dns::DnsManager;
use killswitch::{KillSwitch, KillSwitchMode};
use split_tunnel::SplitTunnel;
use analytics::{Analytics, ConnectionLog};
use chrono::Utc;
use std::io::{self, Write};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    // Initialize logger
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    println!("╔═══════════════════════════════════════════╗");
    println!("║     VPN Mobile - Rust Edition v0.1.0     ║");
    println!("║   Secure, Fast, and Feature-Rich VPN     ║");
    println!("╚═══════════════════════════════════════════╝\n");

    // Initialize components
    let mut config = VpnConfig::default();
    let mut server_manager = ServerManager::new();
    let mut connection = VpnConnection::new(config.protocol_config.clone());
    let mut dns_manager = DnsManager::new();
    let mut kill_switch = KillSwitch::new(config.kill_switch.clone());
    let mut split_tunnel = SplitTunnel::new(config.split_tunnel.clone());
    let mut analytics = Analytics::new();

    loop {
        print_main_menu();
        
        let choice = get_user_input("Enter your choice: ");
        
        match choice.trim() {
            "1" => {
                // Quick connect
                quick_connect(&mut connection, &server_manager).await;
            }
            "2" => {
                // Select server by country
                select_server_by_country(&mut connection, &server_manager).await;
            }
            "3" => {
                // Disconnect
                if connection.is_connected().await {
                    println!("\n🔌 Disconnecting...");
                    if let Err(e) = connection.disconnect().await {
                        println!("❌ Error disconnecting: {}", e);
                    } else {
                        println!("✅ Disconnected successfully");
                    }
                } else {
                    println!("\n⚠️  Not connected to any server");
                }
            }
            "4" => {
                // Connection status
                show_connection_status(&connection).await;
            }
            "5" => {
                // Server list
                show_server_list(&server_manager);
            }
            "6" => {
                // Protocol settings
                protocol_settings(&mut connection, &mut config).await;
            }
            "7" => {
                // Security settings
                security_settings(&mut kill_switch, &mut dns_manager);
            }
            "8" => {
                // Split tunneling
                split_tunnel_menu(&mut split_tunnel);
            }
            "9" => {
                // Statistics
                show_statistics(&analytics);
            }
            "10" => {
                // Settings
                settings_menu(&mut config);
            }
            "0" => {
                // Exit
                if connection.is_connected().await {
                    println!("\n⚠️  Disconnecting before exit...");
                    let _ = connection.disconnect().await;
                }
                println!("\n👋 Thank you for using VPN Mobile!");
                break;
            }
            _ => {
                println!("\n❌ Invalid choice. Please try again.");
            }
        }
        
        println!("\nPress Enter to continue...");
        let _ = get_user_input("");
    }
}

fn print_main_menu() {
    println!("\n╔═══════════════════════════════════════════╗");
    println!("║              MAIN MENU                    ║");
    println!("╠═══════════════════════════════════════════╣");
    println!("║  1. 🚀 Quick Connect (Fastest Server)    ║");
    println!("║  2. 🌍 Select Server by Country          ║");
    println!("║  3. 🔌 Disconnect                         ║");
    println!("║  4. 📊 Connection Status                  ║");
    println!("║  5. 📋 Server List                        ║");
    println!("║  6. ⚙️  Protocol Settings                 ║");
    println!("║  7. 🔒 Security Settings                  ║");
    println!("║  8. 🔀 Split Tunneling                    ║");
    println!("║  9. 📈 Statistics                         ║");
    println!("║ 10. ⚙️  Settings                          ║");
    println!("║  0. 🚪 Exit                               ║");
    println!("╚═══════════════════════════════════════════╝");
}

fn get_user_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input
}

async fn quick_connect(connection: &mut VpnConnection, server_manager: &ServerManager) {
    println!("\n🔍 Finding the fastest server...");
    
    if let Some(server) = server_manager.get_fastest_server() {
        println!("✨ Found: {} {} ({})", 
            server.location.country.flag_emoji(),
            server.name,
            server.location.country.name()
        );
        println!("   Load: {}% | Latency: {}ms | Score: {:.1}/100", 
            server.load, server.latency, server.score()
        );
        
        println!("\n🔐 Connecting...");
        match connection.connect(server.clone()).await {
            Ok(_) => {
                println!("✅ Connected successfully!");
                show_connection_info(connection).await;
            }
            Err(e) => {
                println!("❌ Connection failed: {}", e);
            }
        }
    } else {
        println!("❌ No available servers found");
    }
}

async fn select_server_by_country(connection: &mut VpnConnection, server_manager: &ServerManager) {
    println!("\n╔═══════════════════════════════════════════╗");
    println!("║        SELECT COUNTRY                     ║");
    println!("╚═══════════════════════════════════════════╝");
    
    let countries = Country::all();
    for (idx, country) in countries.iter().enumerate() {
        if idx % 2 == 0 {
            print!("{:2}. {} {:<20}", idx + 1, country.flag_emoji(), country.name());
        } else {
            println!("{:2}. {} {}", idx + 1, country.flag_emoji(), country.name());
        }
    }
    if countries.len() % 2 != 0 {
        println!();
    }
    
    let choice = get_user_input("\nEnter country number (0 to cancel): ");
    
    if let Ok(num) = choice.trim().parse::<usize>() {
        if num == 0 {
            return;
        }
        if num > 0 && num <= countries.len() {
            let country = &countries[num - 1];
            
            if let Some(server) = server_manager.get_fastest_in_country(country) {
                println!("\n✨ Selected: {} {}", server.location.country.flag_emoji(), server.name);
                println!("   Load: {}% | Latency: {}ms | Score: {:.1}/100", 
                    server.load, server.latency, server.score()
                );
                
                println!("\n🔐 Connecting...");
                match connection.connect(server.clone()).await {
                    Ok(_) => {
                        println!("✅ Connected successfully!");
                        show_connection_info(connection).await;
                    }
                    Err(e) => {
                        println!("❌ Connection failed: {}", e);
                    }
                }
            } else {
                println!("❌ No servers available in {}", country.name());
            }
        }
    }
}

async fn show_connection_status(connection: &VpnConnection) {
    let info = connection.get_info().await;
    let stats = connection.get_stats().await;
    
    println!("\n╔═══════════════════════════════════════════╗");
    println!("║        CONNECTION STATUS                  ║");
    println!("╚═══════════════════════════════════════════╝");
    
    println!("Status: {:?}", info.status);
    
    if let Some(server) = &info.server {
        println!("\n🌍 Server: {} {}", server.location.country.flag_emoji(), server.name);
        println!("   Location: {}, {}", server.location.city, server.location.country.name());
        println!("   Protocol: {:?}", connection.get_protocol_config().protocol);
    }
    
    if let Some(ip) = &info.ip_address {
        println!("\n🔐 Your IP: {}", ip);
    }
    
    if let Some(connected_at) = info.connected_at {
        println!("\n⏱️  Connected: {}", connected_at.format("%Y-%m-%d %H:%M:%S"));
        println!("   Duration: {}", Analytics::format_duration(info.duration));
    }
    
    println!("\n📊 Statistics:");
    println!("   ⬆️  Upload: {} ({:.2} MB/s)", 
        Analytics::format_bytes(stats.total_upload), stats.current_speed_up);
    println!("   ⬇️  Download: {} ({:.2} MB/s)", 
        Analytics::format_bytes(stats.total_download), stats.current_speed_down);
    println!("   📶 Latency: {}ms", stats.latency);
    println!("   📉 Packet Loss: {:.2}%", stats.packet_loss);
}

async fn show_connection_info(connection: &VpnConnection) {
    sleep(Duration::from_millis(500)).await;
    let info = connection.get_info().await;
    
    if let Some(ip) = &info.ip_address {
        println!("\n🔐 Your new IP: {}", ip);
    }
    println!("🛡️  You are now protected!");
}

fn show_server_list(server_manager: &ServerManager) {
    println!("\n╔═══════════════════════════════════════════╗");
    println!("║         AVAILABLE SERVERS                 ║");
    println!("╚═══════════════════════════════════════════╝");
    
    for country in Country::all() {
        if let Some(servers) = server_manager.get_servers_by_country(&country) {
            println!("\n{} {} ({} servers)", 
                country.flag_emoji(), country.name(), servers.len());
            
            for server in servers.iter().take(3) {
                let status = if server.online { "🟢" } else { "🔴" };
                println!("  {} {} - Load: {}% | Latency: {}ms | Score: {:.1}", 
                    status, server.name, server.load, server.latency, server.score());
            }
        }
    }
}

async fn protocol_settings(connection: &mut VpnConnection, config: &mut VpnConfig) {
    println!("\n╔═══════════════════════════════════════════╗");
    println!("║        PROTOCOL SETTINGS                  ║");
    println!("╚═══════════════════════════════════════════╝");
    
    let protocols = VpnProtocol::all();
    for (idx, protocol) in protocols.iter().enumerate() {
        let recommended = if protocol.is_recommended() { "⭐" } else { "  " };
        println!("{}. {} {} - {}", 
            idx + 1, recommended, protocol.name(), protocol.description());
        println!("   Security: {}/10 | Speed: {}/10", 
            protocol.security_level(), protocol.speed_rating());
    }
    
    let choice = get_user_input("\nSelect protocol (0 to cancel): ");
    
    if let Ok(num) = choice.trim().parse::<usize>() {
        if num > 0 && num <= protocols.len() {
            let protocol = protocols[num - 1];
            let new_config = ProtocolConfig::new(protocol);
            
            config.protocol_config = new_config.clone();
            connection.set_protocol_config(new_config);
            
            println!("\n✅ Protocol changed to: {}", protocol.name());
            
            if connection.is_connected().await {
                println!("⚠️  You need to reconnect for changes to take effect");
            }
        }
    }
}

fn security_settings(kill_switch: &mut KillSwitch, dns_manager: &mut DnsManager) {
    println!("\n╔═══════════════════════════════════════════╗");
    println!("║        SECURITY SETTINGS                  ║");
    println!("╚═══════════════════════════════════════════╝");
    println!("1. Kill Switch: {}", 
        if kill_switch.is_active() { "🟢 Enabled" } else { "🔴 Disabled" });
    println!("2. DNS Leak Protection: {}", 
        if dns_manager.is_leak_protected() { "🟢 Enabled" } else { "🔴 Disabled" });
    println!("3. DNS Settings");
    println!("0. Back");
    
    let choice = get_user_input("\nEnter your choice: ");
    
    match choice.trim() {
        "1" => {
            if kill_switch.is_active() {
                let _ = kill_switch.disable();
                println!("✅ Kill switch disabled");
            } else {
                let _ = kill_switch.enable();
                println!("✅ Kill switch enabled");
            }
        }
        "2" => {
            let enabled = dns_manager.is_leak_protected();
            dns_manager.enable_leak_protection(!enabled);
            println!("✅ DNS leak protection {}", if !enabled { "enabled" } else { "disabled" });
        }
        "3" => {
            println!("\nAvailable DNS Servers:");
            let servers = DnsManager::get_available_dns_servers();
            for (idx, server) in servers.iter().enumerate() {
                println!("{}. {} - {}", idx + 1, server.name, server.primary);
            }
        }
        _ => {}
    }
}

fn split_tunnel_menu(split_tunnel: &mut SplitTunnel) {
    println!("\n╔═══════════════════════════════════════════╗");
    println!("║        SPLIT TUNNELING                    ║");
    println!("╚═══════════════════════════════════════════╝");
    println!("Status: {}", if split_tunnel.is_active() { "🟢 Enabled" } else { "🔴 Disabled" });
    println!("\n1. Enable/Disable");
    println!("2. Add App");
    println!("3. Remove App");
    println!("4. View Apps");
    println!("5. Load Preset");
    println!("0. Back");
    
    let choice = get_user_input("\nEnter your choice: ");
    
    match choice.trim() {
        "1" => {
            if split_tunnel.is_active() {
                let _ = split_tunnel.disable();
                println!("✅ Split tunneling disabled");
            } else {
                let _ = split_tunnel.enable();
                println!("✅ Split tunneling enabled");
            }
        }
        "4" => {
            let apps = split_tunnel.get_apps();
            if apps.is_empty() {
                println!("No apps configured");
            } else {
                println!("\nConfigured apps:");
                for app in apps {
                    println!("  • {}", app);
                }
            }
        }
        "5" => {
            let presets = SplitTunnel::get_preset_configs();
            println!("\nAvailable presets:");
            for (idx, preset) in presets.iter().enumerate() {
                println!("{}. {} - {}", idx + 1, preset.name, preset.description);
            }
        }
        _ => {}
    }
}

fn show_statistics(analytics: &Analytics) {
    println!("\n{}", analytics.generate_summary_report());
}

fn settings_menu(config: &mut VpnConfig) {
    println!("\n╔═══════════════════════════════════════════╗");
    println!("║            SETTINGS                       ║");
    println!("╚═══════════════════════════════════════════╝");
    println!("1. Auto-connect: {}", if config.auto_connect { "🟢 On" } else { "🔴 Off" });
    println!("2. Start on boot: {}", if config.start_on_boot { "🟢 On" } else { "🔴 Off" });
    println!("3. Show notifications: {}", if config.show_notifications { "🟢 On" } else { "🔴 Off" });
    println!("4. Block ads: {}", if config.block_ads { "🟢 On" } else { "🔴 Off" });
    println!("5. Block malware: {}", if config.block_malware { "🟢 On" } else { "🔴 Off" });
    println!("6. Load Preset Configuration");
    println!("0. Back");
    
    let choice = get_user_input("\nEnter your choice: ");
    
    match choice.trim() {
        "1" => {
            config.auto_connect = !config.auto_connect;
            println!("✅ Auto-connect {}", if config.auto_connect { "enabled" } else { "disabled" });
        }
        "2" => {
            config.start_on_boot = !config.start_on_boot;
            println!("✅ Start on boot {}", if config.start_on_boot { "enabled" } else { "disabled" });
        }
        "3" => {
            config.show_notifications = !config.show_notifications;
            println!("✅ Notifications {}", if config.show_notifications { "enabled" } else { "disabled" });
        }
        "4" => {
            config.block_ads = !config.block_ads;
            println!("✅ Ad blocking {}", if config.block_ads { "enabled" } else { "disabled" });
        }
        "5" => {
            config.block_malware = !config.block_malware;
            println!("✅ Malware blocking {}", if config.block_malware { "enabled" } else { "disabled" });
        }
        "6" => {
            println!("\nPreset Configurations:");
            println!("1. Maximum Security");
            println!("2. Streaming Optimized");
            println!("3. Torrenting");
            
            let preset_choice = get_user_input("Select preset: ");
            match preset_choice.trim() {
                "1" => {
                    *config = VpnConfig::preset_maximum_security();
                    println!("✅ Loaded Maximum Security preset");
                }
                "2" => {
                    *config = VpnConfig::preset_streaming();
                    println!("✅ Loaded Streaming preset");
                }
                "3" => {
                    *config = VpnConfig::preset_torrenting();
                    println!("✅ Loaded Torrenting preset");
                }
                _ => {}
            }
        }
        _ => {}
    }
}
