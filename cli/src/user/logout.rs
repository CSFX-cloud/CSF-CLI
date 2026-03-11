use crate::config::{get_config_path, get_history_path, load_config};
use crate::display;

pub async fn logout() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_config();

    match config {
        Some(cfg) => {
            display::success("logged out");
            display::kv("Server", &cfg.server);
            let config_path = get_config_path();
            if config_path.exists() {
                std::fs::remove_file(&config_path)?;
            }
            let history_path = get_history_path();
            if history_path.exists() {
                std::fs::remove_file(&history_path)?;
            }
        }
        None => {
            display::warn("not logged in");
        }
    }

    Ok(())
}
