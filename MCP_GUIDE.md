# MCP Server Integration Guide

AtmanVessel provides a JSON-RPC 2.0 MCP server to allow agents to control the orchestration framework using standard tools.

## 1. Registration
The AtmanVessel MCP server should be registered in your Antigravity/Codex CLI configuration.

### Auto-Setup (Recommended)
Run the following from the project root:
```bash
agy mcp add atman-vessel $(pwd)/orch.sh mcp
```

### Manual Configuration
Add the following to your `settings.json` (usually in `~/.agy/settings.json` or project-local `.agy/settings.json`):

```json
{
  "mcpServers": {
    "atman-vessel": {
      "command": "/path/to/atman-vessel/orch.sh",
      "args": ["mcp"],
      "transport": "stdio"
    }
  }
}
```

## 2. Available Tools
Once connected, the following tools become available to the agent:

- **`start_task`**: Creates a worktree and launches a specialist.
  - Params: `slug` (string), `profile` (string).
- **`switch_profile`**: Changes the active vessel for the current session.
  - Params: `profile` (string).
- **`freeze_session`**: Records a major milestone and pauses the state.
  - Params: `message` (string).
- **`get_status`**: Returns the current session, active profile, and lock status.

## 3. Security
The MCP server respects the `AGENT_STANDARDS.md`. It will only allow tool execution if the session is correctly locked by the requesting profile.
