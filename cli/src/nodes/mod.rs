pub mod agent_metrics;
pub mod get;
pub mod list;
pub mod metrics;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum NodeCommands {
    List,
    Get { id: String },
    Metrics {
        #[arg(long, short)]
        watch: bool,
    },
    AgentMetrics {
        id: String,
        #[arg(long, short)]
        watch: bool,
    },
}

pub async fn run(cmd: NodeCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        NodeCommands::List => list::run().await,
        NodeCommands::Get { id } => get::run(&id).await,
        NodeCommands::Metrics { watch } => metrics::run(watch).await,
        NodeCommands::AgentMetrics { id, watch } => agent_metrics::run(&id, watch).await,
    }
}
