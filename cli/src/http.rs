use crate::config::load_config;
use serde_json::Value;

pub fn auth() -> Result<(reqwest::Client, String, String), Box<dyn std::error::Error>> {
    let config = load_config().ok_or("not authenticated, run: csf login")?;
    let token = config.token.ok_or("not authenticated, run: csf login")?;
    Ok((reqwest::Client::new(), config.server, token))
}

pub fn base_url(server: &str) -> String {
    format!("{}/api", server.trim_end_matches('/'))
}

pub async fn get_json(
    client: &reqwest::Client,
    url: &str,
    token: &str,
) -> Result<Value, Box<dyn std::error::Error>> {
    let resp = client
        .get(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("accept", "application/json")
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await?;
        eprintln!("error status={} body={}", status, body);
        std::process::exit(1);
    }

    Ok(resp.json().await?)
}

pub async fn post_json(
    client: &reqwest::Client,
    url: &str,
    token: &str,
    body: &Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let resp = client
        .post(url)
        .header("Authorization", format!("Bearer {}", token))
        .header("Content-Type", "application/json")
        .json(body)
        .send()
        .await?;

    if !resp.status().is_success() {
        let status = resp.status();
        let err = resp.text().await?;
        eprintln!("error status={} body={}", status, err);
        std::process::exit(1);
    }

    Ok(resp.json().await?)
}
