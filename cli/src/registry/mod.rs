pub mod agents;
pub mod bootstrap;
pub mod deregister;
pub mod pending;
pub mod pre_register;
pub mod stats;
pub mod tokens;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum RegistryCommands {
    Agents,
    #[command(name = "agents-get")]
    AgentsGet { id: String },
    Pending,
    #[command(name = "pending-delete")]
    PendingDelete { id: String },
    #[command(name = "pre-register")]
    PreRegister {
        name: String,
        hostname: String,
        #[arg(long)]
        os: Option<String>,
        #[arg(long)]
        arch: Option<String>,
        #[arg(long)]
        ttl: Option<i64>,
    },
    Deregister { id: String },
    Stats,
    Tokens,
    #[command(name = "bootstrap-create")]
    BootstrapCreate {
        #[arg(long)]
        description: Option<String>,
        #[arg(long)]
        ttl: Option<i64>,
        #[arg(long, name = "max-uses")]
        max_uses: Option<i32>,
    },
    #[command(name = "bootstrap-list")]
    BootstrapList,
    #[command(name = "bootstrap-revoke")]
    BootstrapRevoke { id: String },
}

pub async fn run(cmd: RegistryCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        RegistryCommands::Agents => agents::list().await,
        RegistryCommands::AgentsGet { id } => agents::get(&id).await,
        RegistryCommands::Pending => pending::run().await,
        RegistryCommands::PendingDelete { id } => deregister::pending(&id).await,
        RegistryCommands::PreRegister { name, hostname, os, arch, ttl } => {
            pre_register::run(name, hostname, os, arch, ttl).await
        }
        RegistryCommands::Deregister { id } => deregister::agent(&id).await,
        RegistryCommands::Stats => stats::run().await,
        RegistryCommands::Tokens => tokens::run().await,
        RegistryCommands::BootstrapCreate { description, ttl, max_uses } => {
            bootstrap::create(description, ttl, max_uses).await
        }
        RegistryCommands::BootstrapList => bootstrap::list().await,
        RegistryCommands::BootstrapRevoke { id } => bootstrap::revoke(&id).await,
    }
}
