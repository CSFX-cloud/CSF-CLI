pub mod stats;

use clap::Subcommand;

#[derive(Subcommand)]
pub enum SystemCommands {
    Stats {
        #[arg(long, short)]
        watch: bool,
    },
}

pub async fn run(cmd: SystemCommands) -> Result<(), Box<dyn std::error::Error>> {
    match cmd {
        SystemCommands::Stats { watch } => stats::run(watch).await,
    }
}
