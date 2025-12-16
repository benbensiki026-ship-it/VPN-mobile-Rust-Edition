use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum Country {
    UnitedStates,
    UnitedKingdom,
    Canada,
    Germany,
    France,
    Netherlands,
    Switzerland,
    Sweden,
    Japan,
    Singapore,
    Australia,
    Brazil,
    India,
    SouthKorea,
    Spain,
    Italy,
    Norway,
    Denmark,
    Finland,
    Poland,
    Turkey,
    UnitedArabEmirates,
    SouthAfrica,
    Mexico,
    Argentina,
}

impl Country {
    pub fn code(&self) -> &str {
        match self {
            Country::UnitedStates => "US",
            Country::UnitedKingdom => "GB",
            Country::Canada => "CA",
            Country::Germany => "DE",
            Country::France => "FR",
            Country::Netherlands => "NL",
            Country::Switzerland => "CH",
            Country::Sweden => "SE",
            Country::Japan => "JP",
            Country::Singapore => "SG",
            Country::Australia => "AU",
            Country::Brazil => "BR",
            Country::India => "IN",
            Country::SouthKorea => "KR",
            Country::Spain => "ES",
            Country::Italy => "IT",
            Country::Norway => "NO",
            Country::Denmark => "DK",
            Country::Finland => "FI",
            Country::Poland => "PL",
            Country::Turkey => "TR",
            Country::UnitedArabEmirates => "AE",
            Country::SouthAfrica => "ZA",
            Country::Mexico => "MX",
            Country::Argentina => "AR",
        }
    }

    pub fn name(&self) -> &str {
        match self {
            Country::UnitedStates => "United States",
            Country::UnitedKingdom => "United Kingdom",
            Country::Canada => "Canada",
            Country::Germany => "Germany",
            Country::France => "France",
            Country::Netherlands => "Netherlands",
            Country::Switzerland => "Switzerland",
            Country::Sweden => "Sweden",
            Country::Japan => "Japan",
            Country::Singapore => "Singapore",
            Country::Australia => "Australia",
            Country::Brazil => "Brazil",
            Country::India => "India",
            Country::SouthKorea => "South Korea",
            Country::Spain => "Spain",
            Country::Italy => "Italy",
            Country::Norway => "Norway",
            Country::Denmark => "Denmark",
            Country::Finland => "Finland",
            Country::Poland => "Poland",
            Country::Turkey => "Turkey",
            Country::UnitedArabEmirates => "United Arab Emirates",
            Country::SouthAfrica => "South Africa",
            Country::Mexico => "Mexico",
            Country::Argentina => "Argentina",
        }
    }

    pub fn flag_emoji(&self) -> &str {
        match self {
            Country::UnitedStates => "🇺🇸",
            Country::UnitedKingdom => "🇬🇧",
            Country::Canada => "🇨🇦",
            Country::Germany => "🇩🇪",
            Country::France => "🇫🇷",
            Country::Netherlands => "🇳🇱",
            Country::Switzerland => "🇨🇭",
            Country::Sweden => "🇸🇪",
            Country::Japan => "🇯🇵",
            Country::Singapore => "🇸🇬",
            Country::Australia => "🇦🇺",
            Country::Brazil => "🇧🇷",
            Country::India => "🇮🇳",
            Country::SouthKorea => "🇰🇷",
            Country::Spain => "🇪🇸",
            Country::Italy => "🇮🇹",
            Country::Norway => "🇳🇴",
            Country::Denmark => "🇩🇰",
            Country::Finland => "🇫🇮",
            Country::Poland => "🇵🇱",
            Country::Turkey => "🇹🇷",
            Country::UnitedArabEmirates => "🇦🇪",
            Country::SouthAfrica => "🇿🇦",
            Country::Mexico => "🇲🇽",
            Country::Argentina => "🇦🇷",
        }
    }

    pub fn all() -> Vec<Country> {
        vec![
            Country::UnitedStates,
            Country::UnitedKingdom,
            Country::Canada,
            Country::Germany,
            Country::France,
            Country::Netherlands,
            Country::Switzerland,
            Country::Sweden,
            Country::Japan,
            Country::Singapore,
            Country::Australia,
            Country::Brazil,
            Country::India,
            Country::SouthKorea,
            Country::Spain,
            Country::Italy,
            Country::Norway,
            Country::Denmark,
            Country::Finland,
            Country::Poland,
            Country::Turkey,
            Country::UnitedArabEmirates,
            Country::SouthAfrica,
            Country::Mexico,
            Country::Argentina,
        ]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerLocation {
    pub city: String,
    pub country: Country,
    pub latitude: f64,
    pub longitude: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VpnServer {
    pub id: String,
    pub name: String,
    pub location: ServerLocation,
    pub host: String,
    pub port: u16,
    pub load: u8,              // 0-100 percentage
    pub latency: u32,          // milliseconds
    pub bandwidth: u64,        // Mbps
    pub is_premium: bool,
    pub supports_p2p: bool,
    pub supports_streaming: bool,
    pub online: bool,
}

impl VpnServer {
    pub fn is_available(&self) -> bool {
        self.online && self.load < 95
    }

    pub fn score(&self) -> f64 {
        // Calculate server score based on load, latency, and bandwidth
        let load_score = (100.0 - self.load as f64) / 100.0;
        let latency_score = 1.0 / (1.0 + (self.latency as f64 / 100.0));
        let bandwidth_score = (self.bandwidth as f64 / 1000.0).min(1.0);
        
        (load_score * 0.4 + latency_score * 0.4 + bandwidth_score * 0.2) * 100.0
    }
}

pub struct ServerManager {
    servers: HashMap<Country, Vec<VpnServer>>,
    favorites: Vec<String>,
    recent: Vec<String>,
}

impl ServerManager {
    pub fn new() -> Self {
        let mut manager = Self {
            servers: HashMap::new(),
            favorites: Vec::new(),
            recent: Vec::new(),
        };
        manager.initialize_servers();
        manager
    }

    fn initialize_servers(&mut self) {
        // Add servers for each country
        for country in Country::all() {
            let servers = self.generate_servers_for_country(&country);
            self.servers.insert(country, servers);
        }
    }

    fn generate_servers_for_country(&self, country: &Country) -> Vec<VpnServer> {
        let cities = self.get_cities_for_country(country);
        cities.iter().enumerate().map(|(idx, city)| {
            VpnServer {
                id: format!("{}-{}", country.code(), idx + 1),
                name: format!("{} #{}", city, idx + 1),
                location: ServerLocation {
                    city: city.to_string(),
                    country: country.clone(),
                    latitude: 0.0,  // Would be actual coordinates
                    longitude: 0.0,
                },
                host: format!("{}.vpn.server.com", city.to_lowercase().replace(" ", "-")),
                port: 443,
                load: (idx * 7) as u8 % 90,
                latency: (idx * 13 + 20) as u32 % 150,
                bandwidth: 1000 + (idx as u64 * 100),
                is_premium: idx % 3 == 0,
                supports_p2p: idx % 2 == 0,
                supports_streaming: true,
                online: true,
            }
        }).collect()
    }

    fn get_cities_for_country(&self, country: &Country) -> Vec<String> {
        match country {
            Country::UnitedStates => vec!["New York", "Los Angeles", "Chicago", "Miami", "Seattle"],
            Country::UnitedKingdom => vec!["London", "Manchester", "Edinburgh"],
            Country::Canada => vec!["Toronto", "Montreal", "Vancouver"],
            Country::Germany => vec!["Frankfurt", "Berlin", "Munich"],
            Country::France => vec!["Paris", "Marseille", "Lyon"],
            Country::Netherlands => vec!["Amsterdam", "Rotterdam"],
            Country::Switzerland => vec!["Zurich", "Geneva"],
            Country::Sweden => vec!["Stockholm", "Gothenburg"],
            Country::Japan => vec!["Tokyo", "Osaka", "Kyoto"],
            Country::Singapore => vec!["Singapore"],
            Country::Australia => vec!["Sydney", "Melbourne", "Brisbane"],
            Country::Brazil => vec!["Sao Paulo", "Rio de Janeiro"],
            Country::India => vec!["Mumbai", "Delhi", "Bangalore"],
            Country::SouthKorea => vec!["Seoul", "Busan"],
            Country::Spain => vec!["Madrid", "Barcelona"],
            Country::Italy => vec!["Rome", "Milan"],
            Country::Norway => vec!["Oslo"],
            Country::Denmark => vec!["Copenhagen"],
            Country::Finland => vec!["Helsinki"],
            Country::Poland => vec!["Warsaw", "Krakow"],
            Country::Turkey => vec!["Istanbul", "Ankara"],
            Country::UnitedArabEmirates => vec!["Dubai", "Abu Dhabi"],
            Country::SouthAfrica => vec!["Johannesburg", "Cape Town"],
            Country::Mexico => vec!["Mexico City", "Guadalajara"],
            Country::Argentina => vec!["Buenos Aires", "Cordoba"],
        }.iter().map(|s| s.to_string()).collect()
    }

    pub fn get_servers_by_country(&self, country: &Country) -> Option<&Vec<VpnServer>> {
        self.servers.get(country)
    }

    pub fn get_all_servers(&self) -> Vec<&VpnServer> {
        self.servers.values().flatten().collect()
    }

    pub fn get_fastest_server(&self) -> Option<&VpnServer> {
        self.get_all_servers()
            .into_iter()
            .filter(|s| s.is_available())
            .max_by(|a, b| a.score().partial_cmp(&b.score()).unwrap())
    }

    pub fn get_fastest_in_country(&self, country: &Country) -> Option<&VpnServer> {
        self.servers.get(country)
            .and_then(|servers| {
                servers.iter()
                    .filter(|s| s.is_available())
                    .max_by(|a, b| a.score().partial_cmp(&b.score()).unwrap())
            })
    }

    pub fn add_favorite(&mut self, server_id: String) {
        if !self.favorites.contains(&server_id) {
            self.favorites.push(server_id);
        }
    }

    pub fn remove_favorite(&mut self, server_id: &str) {
        self.favorites.retain(|id| id != server_id);
    }

    pub fn add_recent(&mut self, server_id: String) {
        self.recent.retain(|id| id != &server_id);
        self.recent.insert(0, server_id);
        if self.recent.len() > 10 {
            self.recent.truncate(10);
        }
    }

    pub fn get_favorites(&self) -> Vec<&VpnServer> {
        self.favorites.iter()
            .filter_map(|id| self.find_server_by_id(id))
            .collect()
    }

    pub fn get_recent(&self) -> Vec<&VpnServer> {
        self.recent.iter()
            .filter_map(|id| self.find_server_by_id(id))
            .collect()
    }

    fn find_server_by_id(&self, id: &str) -> Option<&VpnServer> {
        self.servers.values()
            .flatten()
            .find(|s| s.id == id)
    }
}
