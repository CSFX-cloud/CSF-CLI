mod config;
mod display;
mod events;
mod http;
mod networks;
mod nodes;
mod registry;
mod repl;
mod system;
mod tenant;
mod user;
mod volumes;
mod workloads;

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
    Token,
    #[command(subcommand, about = "Manage storage volumes")]
    Volumes(volumes::VolumeCommands),
    #[command(subcommand, about = "View registry and agents")]
    Registry(registry::RegistryCommands),
    #[command(subcommand, about = "View cluster nodes and metrics")]
    Nodes(nodes::NodeCommands),
    #[command(subcommand, about = "Manage workloads")]
    Workloads(workloads::WorkloadCommands),
    #[command(subcommand, about = "View failover and audit events")]
    Events(events::EventCommands),
    #[command(subcommand, about = "Manage overlay networks")]
    Networks(networks::NetworkCommands),
    #[command(subcommand, about = "Manage tenant organization, users and roles")]
    Tenant(tenant::TenantCommands),
    #[command(subcommand, about = "Cluster-wide system statistics")]
    System(system::SystemCommands),
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
        Some(Commands::Token) => user::token::token().await?,
        Some(Commands::Volumes(cmd)) => volumes::run(cmd).await?,
        Some(Commands::Registry(cmd)) => registry::run(cmd).await?,
        Some(Commands::Nodes(cmd)) => nodes::run(cmd).await?,
        Some(Commands::Workloads(cmd)) => workloads::run(cmd).await?,
        Some(Commands::Events(cmd)) => events::run(cmd).await?,
        Some(Commands::Networks(cmd)) => networks::run(cmd).await?,
        Some(Commands::Tenant(cmd)) => tenant::run(cmd).await?,
        Some(Commands::System(cmd)) => system::run(cmd).await?,
    }

    Ok(())
}
