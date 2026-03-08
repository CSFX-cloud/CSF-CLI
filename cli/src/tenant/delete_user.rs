use crate::{display, http};

pub async fn run(id: &str) -> Result<(), Box<dyn std::error::Error>> {
    let (client, server, token) = http::auth()?;
    let url = format!("{}/organization/users/{}", http::base_url(&server), id);

    let pb = display::spinner("deleting user...");
    http::delete_req(&client, &url, &token).await?;
    pb.finish_and_clear();

    display::success(&format!("user deleted: {}", id));
    println!();
    Ok(())
}
