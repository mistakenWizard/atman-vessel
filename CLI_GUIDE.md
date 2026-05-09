# CLI Usage Reference

The `orch` command is the primary entry point for managing the AtmanVessel framework.

## 1. Global Setup
If not already done, symlink the orchestrator to your path:
```bash
ln -sf $(pwd)/orch.sh ~/.local/bin/orch
```

## 2. Profile Management
Manage the "Souls" (authenticated identities) of your fleet.

- `orch profiles list`: View all authenticated profiles.
- `orch launch --profile <name> --interactive gemini -- "test"`: Authenticate a new profile via Google OAuth.

## 3. Task Orchestration
The primary workflow for agentic development.

- `orch task start <slug> --profile <agent>`:
  1. Creates a git worktree at `.worktrees/task-<slug>`.
  2. Initializes a local session.
  3. Bootstraps the agent's profile (OAuth tokens).
  4. Launches the agent in autonomous mode.

- `orch status`: View active task, profile, and session integrity.
- `orch cleanup`: Remove stale locks and finished worktrees.

## 4. State & Recovery
- `orch freeze -m "message"`: Checkpoint the current state.
- `orch recover`: Generate `RECOVERY.md` after a crash to reconstruct the mental map.
- `orch takeover --profile <name>`: Forcefully acquire the session lock if an agent hangs.

## 5. Development
- `orch verify`: Promote "Asserted" findings from agents to "Verified" project facts.
- `orch postmortem`: Generate a summary of the session's actions, tool usage, and costs.
