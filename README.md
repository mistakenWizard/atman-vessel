# AtmanVessel: The Deterministic AI Orchestrator

## Mission
AtmanVessel is a local AI orchestration framework designed for **Deterministic Session Resumption**. It treats AI models as disposable specialized agents (**Minions**) while maintaining a persistent, project-local "Soul" (Context).

## Key Features
- **Multi-Account Rotation**: Effortlessly switch between 6+ Antigravity identities to bypass rate limits.
- **Identity Isolation**: Process-level environment redirection prevents authentication "bleeding."
- **MCP Tool Support**: A built-in Model Context Protocol server for agent-to-agent delegation.
- **Provenance Aware**: Every state update is tracked with source and timestamp metadata.
- **Global Integration**: One-command access via the global `orch` CLI.
- **Hardened Safety**: Strict 60-second autonomous timeout to prevent runaway token usage.

## Documentation Suite
- **[.antigravity.md](.antigravity.md)**: Core project standards and technical specifications.
- **[AGENT_GUIDE.md](AGENT_GUIDE.md)**: How Minions should operate within the framework.
- **[BOOTSTRAP.md](BOOTSTRAP.md)**: Universal guide for integrating AtmanVessel into any repository.
- **[CLI_GUIDE.md](CLI_GUIDE.md)**: Comprehensive command reference for the `orch` CLI.
- **[MCP_GUIDE.md](MCP_GUIDE.md)**: Technical reference for connecting agents via MCP.

## Getting Started
To install the orchestrator globally on your system:
```bash
ln -sf $(pwd)/orch.sh ~/.local/bin/orch
orch init
```
