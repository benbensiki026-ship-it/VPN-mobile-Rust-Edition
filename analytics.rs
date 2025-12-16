use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc, Duration};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectionLog {
    pub timestamp: DateTime<Utc>,
    pub server_id: String,
    pub server_name: String,
    pub country: String,
    pub duration: Duration,
    pub bytes_sent: u64,
    pub bytes_received: u64,
    pub disconnection_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageStatistics {
    pub total_connections: u64,
    pub total_data_sent: u64,
    pub total_data_received: u64,
    pub total_duration: Duration,
    pub average_session_duration: Duration,
    pub most_used_countries: Vec<(String, u32)>,
    pub most_used_servers: Vec<(String, u32)>,
    pub last_30_days_data: u64,
}

pub struct Analytics {
    connection_logs: Vec<ConnectionLog>,
    statistics: UsageStatistics,
}

impl Analytics {
    pub fn new() -> Self {
        Self {
            connection_logs: Vec::new(),
            statistics: UsageStatistics {
                total_connections: 0,
                total_data_sent: 0,
                total_data_received: 0,
                total_duration: Duration::zero(),
                average_session_duration: Duration::zero(),
                most_used_countries: Vec::new(),
                most_used_servers: Vec::new(),
                last_30_days_data: 0,
            },
        }
    }

    pub fn log_connection(&mut self, log: ConnectionLog) {
        self.connection_logs.push(log.clone());
        self.update_statistics(log);
    }

    fn update_statistics(&mut self, log: ConnectionLog) {
        self.statistics.total_connections += 1;
        self.statistics.total_data_sent += log.bytes_sent;
        self.statistics.total_data_received += log.bytes_received;
        self.statistics.total_duration = self.statistics.total_duration + log.duration;
        
        // Calculate average session duration
        if self.statistics.total_connections > 0 {
            let total_seconds = self.statistics.total_duration.num_seconds();
            let avg_seconds = total_seconds / self.statistics.total_connections as i64;
            self.statistics.average_session_duration = Duration::seconds(avg_seconds);
        }

        // Update country usage
        self.update_country_usage(&log.country);
        
        // Update server usage
        self.update_server_usage(&log.server_name);
        
        // Update last 30 days data
        self.calculate_recent_data();
    }

    fn update_country_usage(&mut self, country: &str) {
        if let Some(entry) = self.statistics.most_used_countries.iter_mut().find(|(c, _)| c == country) {
            entry.1 += 1;
        } else {
            self.statistics.most_used_countries.push((country.to_string(), 1));
        }
        
        // Sort by usage count
        self.statistics.most_used_countries.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Keep only top 10
        if self.statistics.most_used_countries.len() > 10 {
            self.statistics.most_used_countries.truncate(10);
        }
    }

    fn update_server_usage(&mut self, server: &str) {
        if let Some(entry) = self.statistics.most_used_servers.iter_mut().find(|(s, _)| s == server) {
            entry.1 += 1;
        } else {
            self.statistics.most_used_servers.push((server.to_string(), 1));
        }
        
        // Sort by usage count
        self.statistics.most_used_servers.sort_by(|a, b| b.1.cmp(&a.1));
        
        // Keep only top 10
        if self.statistics.most_used_servers.len() > 10 {
            self.statistics.most_used_servers.truncate(10);
        }
    }

    fn calculate_recent_data(&mut self) {
        let thirty_days_ago = Utc::now() - Duration::days(30);
        
        self.statistics.last_30_days_data = self.connection_logs.iter()
            .filter(|log| log.timestamp > thirty_days_ago)
            .map(|log| log.bytes_sent + log.bytes_received)
            .sum();
    }

    pub fn get_statistics(&self) -> &UsageStatistics {
        &self.statistics
    }

    pub fn get_connection_history(&self, limit: usize) -> Vec<&ConnectionLog> {
        self.connection_logs.iter().rev().take(limit).collect()
    }

    pub fn get_data_usage_by_day(&self, days: i64) -> HashMap<String, u64> {
        let start_date = Utc::now() - Duration::days(days);
        let mut usage_by_day: HashMap<String, u64> = HashMap::new();
        
        for log in &self.connection_logs {
            if log.timestamp > start_date {
                let date = log.timestamp.format("%Y-%m-%d").to_string();
                let total_data = log.bytes_sent + log.bytes_received;
                *usage_by_day.entry(date).or_insert(0) += total_data;
            }
        }
        
        usage_by_day
    }

    pub fn get_connection_count_by_country(&self) -> HashMap<String, u32> {
        let mut counts: HashMap<String, u32> = HashMap::new();
        
        for log in &self.connection_logs {
            *counts.entry(log.country.clone()).or_insert(0) += 1;
        }
        
        counts
    }

    pub fn export_logs(&self) -> String {
        serde_json::to_string_pretty(&self.connection_logs).unwrap_or_default()
    }

    pub fn clear_logs(&mut self) {
        self.connection_logs.clear();
        self.statistics = UsageStatistics {
            total_connections: 0,
            total_data_sent: 0,
            total_data_received: 0,
            total_duration: Duration::zero(),
            average_session_duration: Duration::zero(),
            most_used_countries: Vec::new(),
            most_used_servers: Vec::new(),
            last_30_days_data: 0,
        };
    }

    pub fn format_bytes(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        const TB: u64 = GB * 1024;

        if bytes >= TB {
            format!("{:.2} TB", bytes as f64 / TB as f64)
        } else if bytes >= GB {
            format!("{:.2} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.2} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.2} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} B", bytes)
        }
    }

    pub fn format_duration(duration: Duration) -> String {
        let total_seconds = duration.num_seconds();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;

        if hours > 0 {
            format!("{}h {}m {}s", hours, minutes, seconds)
        } else if minutes > 0 {
            format!("{}m {}s", minutes, seconds)
        } else {
            format!("{}s", seconds)
        }
    }

    pub fn generate_summary_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str("=== VPN Usage Summary ===\n\n");
        
        report.push_str(&format!("Total Connections: {}\n", self.statistics.total_connections));
        report.push_str(&format!("Total Data Sent: {}\n", Self::format_bytes(self.statistics.total_data_sent)));
        report.push_str(&format!("Total Data Received: {}\n", Self::format_bytes(self.statistics.total_data_received)));
        report.push_str(&format!("Total Duration: {}\n", Self::format_duration(self.statistics.total_duration)));
        report.push_str(&format!("Average Session: {}\n", Self::format_duration(self.statistics.average_session_duration)));
        report.push_str(&format!("Last 30 Days Data: {}\n\n", Self::format_bytes(self.statistics.last_30_days_data)));
        
        if !self.statistics.most_used_countries.is_empty() {
            report.push_str("Most Used Countries:\n");
            for (country, count) in &self.statistics.most_used_countries {
                report.push_str(&format!("  {} - {} connections\n", country, count));
            }
            report.push('\n');
        }
        
        if !self.statistics.most_used_servers.is_empty() {
            report.push_str("Most Used Servers:\n");
            for (server, count) in &self.statistics.most_used_servers {
                report.push_str(&format!("  {} - {} connections\n", server, count));
            }
        }
        
        report
    }
}

impl Default for Analytics {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(Analytics::format_bytes(500), "500 B");
        assert_eq!(Analytics::format_bytes(1024), "1.00 KB");
        assert_eq!(Analytics::format_bytes(1024 * 1024), "1.00 MB");
        assert_eq!(Analytics::format_bytes(1024 * 1024 * 1024), "1.00 GB");
    }

    #[test]
    fn test_analytics_logging() {
        let mut analytics = Analytics::new();
        
        let log = ConnectionLog {
            timestamp: Utc::now(),
            server_id: "us-1".to_string(),
            server_name: "New York #1".to_string(),
            country: "United States".to_string(),
            duration: Duration::minutes(30),
            bytes_sent: 1024 * 1024 * 100, // 100 MB
            bytes_received: 1024 * 1024 * 500, // 500 MB
            disconnection_reason: None,
        };
        
        analytics.log_connection(log);
        
        assert_eq!(analytics.statistics.total_connections, 1);
        assert_eq!(analytics.statistics.total_data_sent, 1024 * 1024 * 100);
    }
}
