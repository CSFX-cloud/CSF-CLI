use colored::Colorize;
use rustyline::completion::{Completer, Pair};
use rustyline::error::ReadlineError;
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{CompletionType, Config, Context, Editor, Helper};
use std::borrow::Cow;

use crate::display;
use crate::events::EventCommands;
use crate::networks::NetworkCommands;
use crate::nodes::NodeCommands;
use crate::registry::RegistryCommands;
use crate::system::SystemCommands;
use crate::tenant::TenantCommands;
use crate::volumes::VolumeCommands;
use crate::workloads::WorkloadCommands;

const COMMANDS: &[(&str, &str)] = &[
    ("login", "authenticate with the server"),
    ("logout", "clear authentication token"),
    ("status", "show session and user info"),
    ("volumes list", "list all volumes"),
    ("volumes get <id>", "show volume details"),
    ("volumes create <name> --size <gb>", "create a new volume"),
    ("volumes delete <id>", "delete a volume"),
    ("volumes attach <id> --agent <agent-id>", "attach volume to agent"),
    ("volumes detach <id>", "detach volume from agent"),
    ("volumes snapshots --volume <id>", "list snapshots of a volume"),
    ("volumes snapshot-create --volume <id> --name <name>", "create a snapshot"),
    ("registry agents", "list registered agents"),
    ("registry agents-get <id>", "show agent details"),
    ("registry pre-register <name> <hostname>", "pre-register a new node"),
    ("registry deregister <id>", "deregister an agent"),
    ("registry pending", "list pending registrations"),
    ("registry pending-delete <id>", "cancel a pending registration"),
    ("registry stats", "show registry statistics"),
    ("registry tokens", "list active registration tokens"),
    ("registry bootstrap-create", "create a cluster-wide bootstrap token"),
    ("registry bootstrap-list", "list active bootstrap tokens"),
    ("registry bootstrap-revoke <id>", "revoke a bootstrap token"),
    ("nodes list", "list all nodes"),
    ("nodes get <id>", "show node details"),
    ("nodes metrics", "show cluster node metrics"),
    ("nodes metrics --watch", "live-refresh cluster node metrics"),
    ("nodes agent-metrics <id>", "show metrics for a specific agent"),
    ("nodes agent-metrics <id> --watch", "live-refresh agent metrics"),
    ("system stats", "show cluster-wide aggregated stats"),
    ("system stats --watch", "live-refresh cluster stats"),
    ("workloads list", "list all workloads"),
    ("workloads get <id>", "show workload details"),
    ("workloads create <name> <image>", "schedule a new workload"),
    ("workloads delete <id>", "delete a workload"),
    ("workloads metrics <id>", "show workload resource usage"),
    ("workloads metrics <id> --watch", "live-refresh workload metrics"),
    ("events list", "list failover and audit events"),
    ("networks list", "list all overlay networks"),
    ("networks get <id>", "show network details"),
    ("networks create <name> <cidr>", "create a new overlay network"),
    ("networks delete <id>", "delete a network"),
    ("networks policies --network <id>", "list network policies"),
    ("networks policy-create --network <id> --direction <in|out> --action <allow|deny>", "create a network policy"),
    ("networks members --network <id>", "list network members"),
    ("networks member-add --network <id> --workload <id>", "add workload to network"),
    ("networks member-remove --network <id> --workload <id>", "remove workload from network"),
    ("tenant info", "show organization details"),
    ("tenant users", "list users in the organization"),
    ("tenant user-get <id>", "show user details"),
    ("tenant user-create <username> <password> --role <role-id>", "create a new user"),
    ("tenant user-delete <id>", "delete a user"),
    ("tenant roles", "list available roles"),
    ("tenant set-role --user <id> --role <role-id>", "assign a role to a user"),
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
                display: format!("{:<56} {}", cmd, desc.dimmed()),
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
                "volumes create <name> --size <gb>",
                "volumes delete <id>",
                "volumes attach <id> --agent <agent-id>",
                "volumes detach <id>",
                "volumes snapshots --volume <id>",
                "volumes snapshot-create --volume <id> --name <name>",
            ],
        ),
        (
            "Registry",
            &[
                "registry agents",
                "registry agents-get <id>",
                "registry pre-register <name> <hostname>",
                "registry deregister <id>",
                "registry pending",
                "registry pending-delete <id>",
                "registry stats",
                "registry tokens",
                "registry bootstrap-create",
                "registry bootstrap-list",
                "registry bootstrap-revoke <id>",
            ],
        ),
        (
            "Nodes",
            &[
                "nodes list",
                "nodes get <id>",
                "nodes metrics",
                "nodes metrics --watch",
                "nodes agent-metrics <id>",
                "nodes agent-metrics <id> --watch",
            ],
        ),
        ("System", &["system stats", "system stats --watch"]),
        (
            "Workloads",
            &[
                "workloads list",
                "workloads get <id>",
                "workloads create <name> <image>",
                "workloads delete <id>",
                "workloads metrics <id>",
                "workloads metrics <id> --watch",
            ],
        ),
        ("Events", &["events list"]),
        (
            "Networks",
            &[
                "networks list",
                "networks get <id>",
                "networks create <name> <cidr>",
                "networks delete <id>",
                "networks policies --network <id>",
                "networks policy-create --network <id> --direction <in|out> --action <allow|deny>",
                "networks members --network <id>",
                "networks member-add --network <id> --workload <id>",
                "networks member-remove --network <id> --workload <id>",
            ],
        ),
        (
            "Tenant",
            &[
                "tenant info",
                "tenant users",
                "tenant user-get <id>",
                "tenant user-create <username> <password> --role <role-id>",
                "tenant user-delete <id>",
                "tenant roles",
                "tenant set-role --user <id> --role <role-id>",
            ],
        ),
        ("Shell", &["help", "exit"]),
    ];

    for (group, cmds) in groups {
        println!("  {}", group.bold().cyan());
        for cmd in *cmds {
            let base = cmd.split_whitespace().take(2).collect::<Vec<_>>().join(" ");
            if let Some((_, desc)) = COMMANDS.iter().find(|(c, _)| c.starts_with(&base)) {
                println!("    {:<56} {}", cmd.bold(), desc.dimmed());
            }
        }
    }
    println!();
}

fn parse_flag<'a>(parts: &[&'a str], flag: &str) -> Option<&'a str> {
    parts
        .windows(2)
        .find(|w| w[0] == flag)
        .map(|w| w[1])
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
        ["volumes", "create", name, "--size", size] => {
            crate::volumes::run(VolumeCommands::Create {
                name: name.to_string(),
                size: size.parse().unwrap_or(10),
                pool: None,
            })
            .await?
        }
        ["volumes", "create", name, "--size", size, "--pool", pool] => {
            crate::volumes::run(VolumeCommands::Create {
                name: name.to_string(),
                size: size.parse().unwrap_or(10),
                pool: Some(pool.to_string()),
            })
            .await?
        }
        ["volumes", "delete", id] => {
            crate::volumes::run(VolumeCommands::Delete { id: id.to_string() }).await?
        }
        ["volumes", "attach", id, "--agent", agent] => {
            crate::volumes::run(VolumeCommands::Attach {
                id: id.to_string(),
                agent: agent.to_string(),
                workload: None,
            })
            .await?
        }
        ["volumes", "attach", id, "--agent", agent, "--workload", wid] => {
            crate::volumes::run(VolumeCommands::Attach {
                id: id.to_string(),
                agent: agent.to_string(),
                workload: Some(wid.to_string()),
            })
            .await?
        }
        ["volumes", "detach", id] => {
            crate::volumes::run(VolumeCommands::Detach { id: id.to_string() }).await?
        }
        ["volumes", "snapshots", "--volume", vid] => {
            crate::volumes::run(VolumeCommands::Snapshots { volume: vid.to_string() }).await?
        }
        ["volumes", "snapshot-create", "--volume", vid, "--name", name] => {
            crate::volumes::run(VolumeCommands::SnapshotCreate {
                volume: vid.to_string(),
                name: name.to_string(),
            })
            .await?
        }

        ["registry", "agents"] => crate::registry::run(RegistryCommands::Agents).await?,
        ["registry", "agents-get", id] => {
            crate::registry::run(RegistryCommands::AgentsGet { id: id.to_string() }).await?
        }
        ["registry", "pre-register", name, hostname] => {
            crate::registry::run(RegistryCommands::PreRegister {
                name: name.to_string(),
                hostname: hostname.to_string(),
                os: None,
                arch: None,
                ttl: None,
            })
            .await?
        }
        ["registry", "pre-register", name, hostname, "--os", os] => {
            crate::registry::run(RegistryCommands::PreRegister {
                name: name.to_string(),
                hostname: hostname.to_string(),
                os: Some(os.to_string()),
                arch: None,
                ttl: None,
            })
            .await?
        }
        ["registry", "deregister", id] => {
            crate::registry::run(RegistryCommands::Deregister { id: id.to_string() }).await?
        }
        ["registry", "pending"] => crate::registry::run(RegistryCommands::Pending).await?,
        ["registry", "pending-delete", id] => {
            crate::registry::run(RegistryCommands::PendingDelete { id: id.to_string() }).await?
        }
        ["registry", "stats"] => crate::registry::run(RegistryCommands::Stats).await?,
        ["registry", "tokens"] => crate::registry::run(RegistryCommands::Tokens).await?,
        ["registry", "bootstrap-create"] => {
            crate::registry::run(RegistryCommands::BootstrapCreate {
                description: None,
                ttl: None,
                max_uses: None,
            })
            .await?
        }
        ["registry", "bootstrap-list"] => {
            crate::registry::run(RegistryCommands::BootstrapList).await?
        }
        ["registry", "bootstrap-revoke", id] => {
            crate::registry::run(RegistryCommands::BootstrapRevoke { id: id.to_string() }).await?
        }

        ["nodes", "list"] => crate::nodes::run(NodeCommands::List).await?,
        ["nodes", "get", id] => {
            crate::nodes::run(NodeCommands::Get { id: id.to_string() }).await?
        }
        ["nodes", "metrics"] => {
            crate::nodes::run(NodeCommands::Metrics { watch: false }).await?
        }
        ["nodes", "metrics", "--watch"] => {
            crate::nodes::run(NodeCommands::Metrics { watch: true }).await?
        }
        ["nodes", "agent-metrics", id] => {
            crate::nodes::run(NodeCommands::AgentMetrics { id: id.to_string(), watch: false }).await?
        }
        ["nodes", "agent-metrics", id, "--watch"] => {
            crate::nodes::run(NodeCommands::AgentMetrics { id: id.to_string(), watch: true }).await?
        }

        ["system", "stats"] => {
            crate::system::run(SystemCommands::Stats { watch: false }).await?
        }
        ["system", "stats", "--watch"] => {
            crate::system::run(SystemCommands::Stats { watch: true }).await?
        }

        ["workloads", "metrics", id] => {
            crate::workloads::run(WorkloadCommands::Metrics { id: id.to_string(), watch: false }).await?
        }
        ["workloads", "metrics", id, "--watch"] => {
            crate::workloads::run(WorkloadCommands::Metrics { id: id.to_string(), watch: true }).await?
        }
        ["workloads", "list"] => crate::workloads::run(WorkloadCommands::List).await?,
        ["workloads", "create", name, image] => {
            crate::workloads::run(WorkloadCommands::Create {
                name: name.to_string(),
                image: image.to_string(),
                cpu: 500,
                memory: 536870912,
                disk: 10737418240,
            })
            .await?
        }
        ["workloads", "create", name, image, "--cpu", cpu] => {
            crate::workloads::run(WorkloadCommands::Create {
                name: name.to_string(),
                image: image.to_string(),
                cpu: cpu.parse().unwrap_or(500),
                memory: 536870912,
                disk: 10737418240,
            })
            .await?
        }
        ["workloads", "get", id] => {
            crate::workloads::run(WorkloadCommands::Get { id: id.to_string() }).await?
        }
        ["workloads", "delete", id] => {
            crate::workloads::run(WorkloadCommands::Delete { id: id.to_string() }).await?
        }

        ["events", "list"] => crate::events::run(EventCommands::List).await?,

        ["networks", "list"] => crate::networks::run(NetworkCommands::List).await?,
        ["networks", "get", id] => {
            crate::networks::run(NetworkCommands::Get { id: id.to_string() }).await?
        }
        ["networks", "delete", id] => {
            crate::networks::run(NetworkCommands::Delete { id: id.to_string() }).await?
        }
        ["networks", "create", name, cidr] => {
            crate::networks::run(NetworkCommands::Create {
                name: name.to_string(),
                cidr: cidr.to_string(),
                overlay: "wireguard".to_string(),
            })
            .await?
        }
        ["networks", "create", name, cidr, "--overlay", overlay] => {
            crate::networks::run(NetworkCommands::Create {
                name: name.to_string(),
                cidr: cidr.to_string(),
                overlay: overlay.to_string(),
            })
            .await?
        }
        parts if parts.first() == Some(&"networks") && parts.get(1) == Some(&"policies") => {
            let network = parse_flag(parts, "--network").unwrap_or("").to_string();
            crate::networks::run(NetworkCommands::Policies { network }).await?
        }
        parts if parts.first() == Some(&"networks") && parts.get(1) == Some(&"policy-create") => {
            let network = parse_flag(parts, "--network").unwrap_or("").to_string();
            let direction = parse_flag(parts, "--direction").unwrap_or("ingress").to_string();
            let action = parse_flag(parts, "--action").unwrap_or("allow").to_string();
            let priority = parse_flag(parts, "--priority").and_then(|v| v.parse().ok()).unwrap_or(100);
            let source = parse_flag(parts, "--source").map(|s| s.to_string());
            let destination = parse_flag(parts, "--destination").map(|s| s.to_string());
            let port = parse_flag(parts, "--port").and_then(|v| v.parse().ok());
            let protocol = parse_flag(parts, "--protocol").map(|s| s.to_string());
            crate::networks::run(NetworkCommands::PolicyCreate {
                network, direction, action, priority, source, destination, port, protocol,
            })
            .await?
        }
        parts if parts.first() == Some(&"networks") && parts.get(1) == Some(&"members") => {
            let network = parse_flag(parts, "--network").unwrap_or("").to_string();
            crate::networks::run(NetworkCommands::Members { network }).await?
        }
        parts if parts.first() == Some(&"networks") && parts.get(1) == Some(&"member-add") => {
            let network = parse_flag(parts, "--network").unwrap_or("").to_string();
            let workload = parse_flag(parts, "--workload").unwrap_or("").to_string();
            crate::networks::run(NetworkCommands::MemberAdd { network, workload }).await?
        }
        parts if parts.first() == Some(&"networks") && parts.get(1) == Some(&"member-remove") => {
            let network = parse_flag(parts, "--network").unwrap_or("").to_string();
            let workload = parse_flag(parts, "--workload").unwrap_or("").to_string();
            crate::networks::run(NetworkCommands::MemberRemove { network, workload }).await?
        }

        ["tenant", "info"] => crate::tenant::run(TenantCommands::Info).await?,
        ["tenant", "users"] => crate::tenant::run(TenantCommands::Users).await?,
        ["tenant", "user-get", id] => {
            crate::tenant::run(TenantCommands::UserGet { id: id.to_string() }).await?
        }
        ["tenant", "user-delete", id] => {
            crate::tenant::run(TenantCommands::UserDelete { id: id.to_string() }).await?
        }
        ["tenant", "roles"] => crate::tenant::run(TenantCommands::Roles).await?,
        parts if parts.first() == Some(&"tenant") && parts.get(1) == Some(&"user-create") => {
            if parts.len() < 4 {
                display::error("usage: tenant user-create <username> <password> --role <role-id>");
            } else {
                let username = parts[2].to_string();
                let password = parts[3].to_string();
                let role = parse_flag(parts, "--role").unwrap_or("").to_string();
                let email = parse_flag(parts, "--email").map(|s| s.to_string());
                let force = parse_flag(parts, "--force-password-change")
                    .map(|v| v == "true")
                    .unwrap_or(false);
                crate::tenant::run(TenantCommands::UserCreate {
                    username,
                    password,
                    role,
                    email,
                    force_password_change: force,
                })
                .await?
            }
        }
        parts if parts.first() == Some(&"tenant") && parts.get(1) == Some(&"set-role") => {
            let user = parse_flag(parts, "--user").unwrap_or("").to_string();
            let role = parse_flag(parts, "--role").unwrap_or("").to_string();
            crate::tenant::run(TenantCommands::SetRole { user, role }).await?
        }

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
