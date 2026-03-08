use crate::{display, http};

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = http::auth()?;
    let url = format!("{}/organization", http::base_url(&server));

    let pb = display::spinner("fetching organization...");
    let org = http::get_json(&client, &url, &token).await?;
    pb.finish_and_clear();

    display::section("Organization");
    display::kv("ID", org["id"].as_str().unwrap_or("-"));
    display::kv("Name", org["name"].as_str().unwrap_or("-"));
    display::kv("Description", org["description"].as_str().unwrap_or("-"));
    display::kv("Created", org["created_at"].as_str().unwrap_or("-"));
    println!();
    Ok(())
}
