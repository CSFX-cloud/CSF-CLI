use crate::display;
use crate::http::{auth, base_url, delete_req};

pub async fn agent(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = auth()?;
    let url = format!("{}/registry/admin/agents/{}", base_url(&server), id);

    let pb = display::spinner("deregistering agent...");
    let result = delete_req(&client, &url, &token).await;
    pb.finish_and_clear();

    result?;
    display::success(&format!("agent {} deregistered", id));

    Ok(())
}
