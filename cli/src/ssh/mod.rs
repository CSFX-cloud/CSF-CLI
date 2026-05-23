pub mod connect;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum SshCommands {
    Connect {
        #[arg(help = "Node name or UUID")]
        node: String,
        #[arg(long, short, help = "SSH port", default_value = "22")]
        port: u16,
    },
}

pub async fn run(cmd: SshCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        SshCommands::Connect { node, port } => connect::run(node, port).await,
    }
}
