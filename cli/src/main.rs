mod config;
mod display;
mod http;
mod nodes;
mod registry;
mod repl;
mod user;
mod volumes;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "csf")]
#[command(about = "Cloud Service Foundry CLI")]
#[command(version)]
#[command(disable_help_subcommand = true)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Login,
    Logout,
    Status,
    #[command(subcommand, about = "Manage storage volumes")]
    Volumes(volumes::VolumeCommands),
    #[command(subcommand, about = "View registry and agents")]
    Registry(registry::RegistryCommands),
    #[command(subcommand, about = "View cluster nodes and metrics")]
    Nodes(nodes::NodeCommands),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    display::banner();

    let cli = Cli::parse();

    match cli.command {
        None => repl::run().await?,
        Some(Commands::Login) => user::login::login().await?,
        Some(Commands::Logout) => user::logout::logout().await?,
        Some(Commands::Status) => user::status::status().await?,
        Some(Commands::Volumes(cmd)) => volumes::run(cmd).await?,
        Some(Commands::Registry(cmd)) => registry::run(cmd).await?,
        Some(Commands::Nodes(cmd)) => nodes::run(cmd).await?,
    }

    Ok(())
}
