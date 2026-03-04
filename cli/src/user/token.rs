use crate::config::load_config;

pub async fn token() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config().ok_or("Not logged in")?;
    let token = config.token.ok_or("No token stored")?;
    println!("{}", token);
    Ok(())
}
