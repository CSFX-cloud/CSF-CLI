# CSFX-CLI

Command-line interface for Cloud Service Foundry.

## Requirements

- Rust toolchain (stable) — [install via rustup](https://rustup.rs)

## Installation

```bash
git clone https://github.com/cs-foundry/CSFX-CLI.git
cd CSFX-CLI
cargo install --path cli
```

The `csfx` binary is installed to `~/.cargo/bin/csfx`. Ensure `~/.cargo/bin` is in your `PATH`.

## Update

```bash
git pull
cargo install --path cli --force
```
@
## Usage

```
csfx                         # interactive REPL
csfx login                   # authenticate against CSFX-Core
csfx logout                  # remove stored credentials
csfx status                  # show current session info
csfx token                   # print stored JWT token (useful for scripts)

csfx volumes list
csfx volumes get <id>
csfx volumes snapshots
csfx volumes nodes

csfx registry agents
csfx registry agents-get <id>
csfx registry pre-register <name> <hostname> [--os <os>] [--arch <arch>] [--ttl <hours>]
csfx registry pending
csfx registry pending-delete <id>
csfx registry deregister <id>
csfx registry stats
csfx registry tokens

csfx nodes list
csfx nodes metrics
```

## Configuration

Config is stored at `~/.csfx/config.json`:

```json
{
  "server": "http://localhost:8000",
  "token": "<jwt>"
}
```

## Scripting

```bash
export JWT=$(csfx token)
GATEWAY_URL=http://localhost:8000 ./scripts/test-mtls.sh
```
