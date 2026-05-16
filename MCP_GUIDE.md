# MCP Server Integration Guide

AtmanVessel provides a JSON-RPC 2.0 MCP server to allow agents to control the orchestration framework using standard tools.

## 1. Registration
The AtmanVessel MCP server should be registered in your Gemini/Codex CLI configuration.

### Auto-Setup (Recommended)
Run the following from the project root:
```bash
gemini mcp add atman-vessel $(pwd)/orch.sh mcp
```

### Manual Configuration
Add the following to your `settings.json` (usually in `~/.gemini/settings.json` or project-local `.gemini/settings.json`):

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

- **`start_task`**: Creates a worktree and launches a specialist **Minion**. Now automatically scaffolds a plan in the centralized investigations layer.
  - Params: `slug` (string), `profile` (string).
- **`finish_task`**: Commits and merges results from a Minion worktree. Now performs a centralized brief check and automatic worktree cleanup.
  - Params: `slug` (string).
- **`switch_profile`**: Changes the active vessel for the current session.
  - Params: `profile` (string).
- **`freeze_session`**: Records a major milestone and pauses the state.
  - Params: `message` (string).
- **`get_status`**: Returns the current session, active profile, and lock status.

## 3. Security
The MCP server respects the `AGENT_STANDARDS.md`. It will only allow tool execution if the session is correctly locked by the requesting profile.
