use colored::Colorize;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{CompletionType, Config, Context, Editor, Helper};
use std::borrow::Cow;

use crate::display;
use crate::nodes::NodeCommands;
use crate::registry::RegistryCommands;
use crate::volumes::VolumeCommands;

const COMMANDS: &[(&str, &str)] = &[
    ("login", "authenticate with the server"),
    ("logout", "clear authentication token"),
    ("status", "show session and user info"),
    ("volumes list", "list all volumes"),
    ("volumes get <id>", "show volume details"),
    ("volumes snapshots", "list all snapshots"),
    ("volumes nodes", "list cluster nodes (volume manager)"),
    ("registry agents", "list registered agents"),
    ("registry agents-get <id>", "show agent details"),
    ("registry pending", "list pending agents"),
    ("registry stats", "show registry statistics"),
    ("registry tokens", "list registration tokens"),
    ("nodes list", "list all nodes"),
    ("nodes metrics", "show system metrics"),
    ("help", "show available commands"),
    ("exit", "exit the shell"),
];

struct CsfHelper;

impl Completer for CsfHelper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let prefix = &line[..pos];
        let candidates: Vec<Pair> = COMMANDS
            .iter()
            .filter(|(cmd, _)| cmd.starts_with(prefix))
            .map(|(cmd, desc)| Pair {
                display: format!("{:<36} {}", cmd, desc.dimmed()),
                replacement: cmd.to_string(),
            })
            .collect();
        Ok((0, candidates))
    }
}

impl Hinter for CsfHelper {
    type Hint = String;

    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<String> {
        if pos < line.len() {
            return None;
        }
        COMMANDS
            .iter()
            .find(|(cmd, _)| cmd.starts_with(line) && *cmd != line)
            .map(|(cmd, _)| cmd[line.len()..].to_string())
    }
}

impl Highlighter for CsfHelper {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Cow::Owned(hint.dimmed().to_string())
    }
}

impl Validator for CsfHelper {}

impl Helper for CsfHelper {}

fn print_help() {
    display::section("Commands");

    let groups: &[(&str, &[&str])] = &[
        ("Auth", &["login", "logout", "status"]),
        (
            "Volumes",
            &[
                "volumes list",
                "volumes get <id>",
                "volumes snapshots",
                "volumes nodes",
            ],
        ),
        (
            "Registry",
            &[
                "registry agents",
                "registry agents-get <id>",
                "registry pending",
                "registry stats",
                "registry tokens",
            ],
        ),
        ("Nodes", &["nodes list", "nodes metrics"]),
        ("Shell", &["help", "exit"]),
    ];

    for (group, cmds) in groups {
        println!("  {}", group.bold().cyan());
        for cmd in *cmds {
            let base = cmd.split_whitespace().take(2).collect::<Vec<_>>().join(" ");
            if let Some((_, desc)) = COMMANDS.iter().find(|(c, _)| c.starts_with(&base)) {
                println!("    {:<36} {}", cmd.bold(), desc.dimmed());
            }
        }
    }
    println!();
}

async fn dispatch(parts: &[&str]) -> Result<(), Box<dyn std::error::Error>> {
    match parts {
        ["login"] => crate::user::login::login().await?,
        ["logout"] => crate::user::logout::logout().await?,
        ["status"] => crate::user::status::status().await?,

        ["volumes", "list"] => crate::volumes::run(VolumeCommands::List).await?,
        ["volumes", "get", id] => {
            crate::volumes::run(VolumeCommands::Get { id: id.to_string() }).await?
        }
        ["volumes", "snapshots"] => crate::volumes::run(VolumeCommands::Snapshots).await?,
        ["volumes", "nodes"] => crate::volumes::run(VolumeCommands::Nodes).await?,

        ["registry", "agents"] => crate::registry::run(RegistryCommands::Agents).await?,
        ["registry", "agents-get", id] => {
            crate::registry::run(RegistryCommands::AgentsGet { id: id.to_string() }).await?
        }
        ["registry", "pending"] => crate::registry::run(RegistryCommands::Pending).await?,
        ["registry", "stats"] => crate::registry::run(RegistryCommands::Stats).await?,
        ["registry", "tokens"] => crate::registry::run(RegistryCommands::Tokens).await?,

        ["nodes", "list"] => crate::nodes::run(NodeCommands::List).await?,
        ["nodes", "metrics"] => crate::nodes::run(NodeCommands::Metrics).await?,

        ["help"] | ["?"] => print_help(),

        [cmd, ..] => {
            display::error(&format!("unknown command: {}", cmd));
            display::info("type 'help' to see available commands");
        }
        [] => {}
    }
    Ok(())
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .completion_type(CompletionType::List)
        .build();

    let mut rl: Editor<CsfHelper, _> = Editor::with_config(config)?;
    rl.set_helper(Some(CsfHelper));

    let history_path = dirs::home_dir().map(|mut p| {
        p.push(".csf");
        let _ = std::fs::create_dir_all(&p);
        p.push("history");
        p
    });

    if let Some(ref path) = history_path {
        let _ = rl.load_history(path);
    }

    println!(
        "  {}",
        "Tab for completion, arrow keys for history, exit to quit".dimmed()
    );
    println!();

    loop {
        let prompt = format!("{} ", "csf>".cyan().bold());

        match rl.readline(&prompt) {
            Ok(line) => {
                let trimmed = line.trim().to_string();
                if trimmed.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(trimmed.as_str());

                if matches!(trimmed.as_str(), "exit" | "quit" | "q") {
                    break;
                }

                let parts: Vec<&str> = trimmed.split_whitespace().collect();
                if let Err(e) = dispatch(&parts).await {
                    display::error(&e.to_string());
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                println!();
                break;
            }
            Err(e) => {
                display::error(&format!("readline error: {}", e));
                break;
            }
        }
    }

    if let Some(ref path) = history_path {
        let _ = rl.save_history(path);
    }

    Ok(())
}
