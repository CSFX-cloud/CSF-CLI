pub mod releases;
pub mod stats;
pub mod update;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum SystemCommands {
    Stats {
        #[arg(long, short)]
        watch: bool,
    },
    Update {
        version: String,
    },
    UpdateWatch,
    UpdatePause,
    UpdateResume,
    CheckUpdate {
        #[arg(long, help = "include pre-release versions")]
        pre: bool,
    },
}

pub async fn run(cmd: SystemCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        SystemCommands::Stats { watch } => stats::run(watch).await,
        SystemCommands::Update { version } => update::run(version).await,
        SystemCommands::UpdateWatch => update::run_status().await,
        SystemCommands::UpdatePause => update::run_pause().await,
        SystemCommands::UpdateResume => update::run_resume().await,
        SystemCommands::CheckUpdate { pre } => releases::run(pre).await,
    }
}
