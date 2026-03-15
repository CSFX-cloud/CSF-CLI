use crate::display;
use crate::http::{auth, base_url, get_json, post_json};
use serde_json::json;

pub async fn run(version: String) -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/system/update", base_url(&server));

    let resp = post_json(&client, &url, &token, &json!({ "version": version })).await?;

    display::kv("status", resp["status"].as_str().unwrap_or("-"));
    display::kv("version", resp["version"].as_str().unwrap_or("-"));

    Ok(())
}

pub async fn run_status() -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/system/update/status", base_url(&server));

    let resp = get_json(&client, &url, &token).await?;

    display::section("Control Plane Update Status");
    display::kv("current", resp["current_version"].as_str().unwrap_or("-"));
    display::kv("desired", resp["desired_version"].as_str().unwrap_or("-"));

    let result = resp["last_result"].as_str().unwrap_or("-");
    let color = match result {
        "success" => colored::Color::Green,
        "failed" => colored::Color::Red,
        "in_progress" => colored::Color::Yellow,
        _ => colored::Color::White,
    };
    display::kv_colored("last result", result, color);

    Ok(())
}
