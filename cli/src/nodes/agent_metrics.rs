use crate::display::{self, kv, section};
use crate::http::{auth, base_url, get_json};
use colored::Colorize;
use serde::Deserialize;

#[derive(Deserialize)]
struct AgentMetrics {
    cpu_usage_percent: Option<f32>,
    cpu_cores: Option<i32>,
    memory_total_bytes: Option<i64>,
    memory_used_bytes: Option<i64>,
    memory_usage_percent: Option<f32>,
    disk_total_bytes: Option<i64>,
    disk_used_bytes: Option<i64>,
    disk_usage_percent: Option<f32>,
    network_rx_bytes: Option<i64>,
    network_tx_bytes: Option<i64>,
    hostname: Option<String>,
    os_name: Option<String>,
    os_version: Option<String>,
    kernel_version: Option<String>,
    uptime_seconds: Option<i64>,
    timestamp: Option<String>,
}

fn usage_bar(pct: f32, width: usize) -> String {
    let filled = ((pct / 100.0) * width as f32) as usize;
    let empty = width - filled.min(width);
    let color = if pct > 90.0 {
        colored::Color::Red
    } else if pct > 70.0 {
        colored::Color::Yellow
    } else {
        colored::Color::Green
    };
    format!(
        "[{}{}] {:.1}%",
        "#".repeat(filled).color(color),
        ".".repeat(empty).dimmed(),
        pct
    )
}

fn format_uptime(seconds: i64) -> String {
    let days = seconds / 86400;
    let hours = (seconds % 86400) / 3600;
    let minutes = (seconds % 3600) / 60;
    format!("{}d {}h {}m", days, hours, minutes)
}

fn print_metrics(id: &str, m: &AgentMetrics) {
    section(&format!("Node Metrics  /  {}", &id[..8.min(id.len())]));

    if let Some(ref ts) = m.timestamp {
        kv("As of", &ts[..16.min(ts.len())]);
    }
    if let Some(ref h) = m.hostname {
        kv("Hostname", h);
    }
    if let Some(ref os) = m.os_name {
        kv(
            "OS",
            &format!("{} {}", os, m.os_version.as_deref().unwrap_or("")),
        );
    }
    if let Some(ref k) = m.kernel_version {
        kv("Kernel", k);
    }
    if let Some(uptime) = m.uptime_seconds {
        kv("Uptime", &format_uptime(uptime));
    }

    println!();

    if let Some(cores) = m.cpu_cores {
        kv("CPU Cores", &cores.to_string());
    }
    if let Some(pct) = m.cpu_usage_percent {
        println!("  {:<20} {}", "CPU Usage".dimmed(), usage_bar(pct, 30));
    }

    println!();

    if let (Some(total), Some(used)) = (m.memory_total_bytes, m.memory_used_bytes) {
        kv(
            "Memory",
            &format!(
                "{:.1}G / {:.1}G",
                used as f64 / 1_073_741_824.0,
                total as f64 / 1_073_741_824.0
            ),
        );
    }
    if let Some(pct) = m.memory_usage_percent {
        println!("  {:<20} {}", "Memory Usage".dimmed(), usage_bar(pct, 30));
    }

    println!();

    if let (Some(total), Some(used)) = (m.disk_total_bytes, m.disk_used_bytes) {
        kv(
            "Disk",
            &format!(
                "{:.1}G / {:.1}G",
                used as f64 / 1_073_741_824.0,
                total as f64 / 1_073_741_824.0
            ),
        );
    }
    if let Some(pct) = m.disk_usage_percent {
        println!("  {:<20} {}", "Disk Usage".dimmed(), usage_bar(pct, 30));
    }

    println!();

    if let (Some(rx), Some(tx)) = (m.network_rx_bytes, m.network_tx_bytes) {
        kv("Network RX", &format!("{:.2} MB", rx as f64 / 1_048_576.0));
        kv("Network TX", &format!("{:.2} MB", tx as f64 / 1_048_576.0));
    }

    println!();
}

pub async fn run(id: &str, watch: bool) -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/agents/{}/metrics/latest", base_url(&server), id);

    if !watch {
        let pb = display::spinner("fetching agent metrics...");
        let data = get_json(&client, &url, &token).await;
        pb.finish_and_clear();
        let m: AgentMetrics = serde_json::from_value(data?)?;
        print_metrics(id, &m);
        return Ok(());
    }

    loop {
        let data = get_json(&client, &url, &token).await;
        print!("\x1B[2J\x1B[H");
        match data {
            Ok(v) => match serde_json::from_value::<AgentMetrics>(v) {
                Ok(m) => print_metrics(id, &m),
                Err(e) => display::error(&format!("parse error: {}", e)),
            },
            Err(e) => display::error(&format!("fetch error: {}", e)),
        }
        println!("  {}", "refreshing every 5s — Ctrl+C to exit".dimmed());
        tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    }
}
