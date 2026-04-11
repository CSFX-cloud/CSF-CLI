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
    display::kv("desired version", resp["desired_version"].as_str().unwrap_or("-"));
    display::kv("resolved rev", resp["desired_flake_rev"].as_str().unwrap_or("-"));

    let build_status = resp["build_status"].as_str().unwrap_or("-");
    let build_color = match build_status {
        "ready" => colored::Color::Green,
        "building" => colored::Color::Yellow,
        "failed" => colored::Color::Red,
        _ => colored::Color::White,
    };
    display::kv_colored("build status", build_status, build_color);

    let result = resp["last_result"].as_str().unwrap_or("-");
    let result_color = match result {
        "success" => colored::Color::Green,
        "failed" => colored::Color::Red,
        _ => colored::Color::White,
    };
    display::kv_colored("last result", result, result_color);

    let paused = resp["paused"].as_bool().unwrap_or(false);
    display::kv_colored(
        "paused",
        if paused { "yes" } else { "no" },
        if paused { colored::Color::Yellow } else { colored::Color::Green },
    );

    Ok(())
}

pub async fn run_pause() -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/system/update/pause", base_url(&server));

    let resp = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        crate::display::error(&format!("request failed: {}", resp.status()));
        std::process::exit(1);
    }

    display::success("updates paused");
    Ok(())
}

pub async fn run_resume() -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/system/update/resume", base_url(&server));

    let resp = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .send()
        .await?;

    if !resp.status().is_success() {
        crate::display::error(&format!("request failed: {}", resp.status()));
        std::process::exit(1);
    }

    display::success("updates resumed");
    Ok(())
}
