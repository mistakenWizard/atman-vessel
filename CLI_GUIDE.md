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

- `orch task start <slug> --profile <minion>`:
  1. Creates a git worktree at `.worktrees/session-<id>-<slug>`.
  2. Initializes a local session.
  3. **Scaffolds a Centralized Plan** at `/mnt/e/investigations/projects/<repo>/plans/`.
  4. Captures the current branch as `parent_branch`.
  5. Bootstraps the minion's profile (OAuth tokens).
  6. Launches the minion in autonomous mode.

- `orch task finish <slug>`:
  1. **Integrity Check**: Verifies that a centralized design brief exists.
  2. **Hygiene Check**: Scans for untracked files in the worktree.
  3. Commits all changes in the task worktree.
  4. Merges the task branch back into the `parent_branch`.
  5. **Auto-Cleanup**: Automatically removes the worktree and audit-logs to investigations.
  6. Marks the task as **Done**.

- `orch task list`: View all active and stale tasks.
- `orch status`: View active task, profile, and session integrity.
- `orch cleanup`: Remove stale locks and **Stale** worktrees.

## 4. State & Recovery
- `orch freeze -m "message"`: Checkpoint the current state.
- `orch recover`: Generate `RECOVERY.md` after a crash to reconstruct the mental map.
- `orch takeover --profile <name>`: Forcefully acquire the session lock if a minion hangs.

## 5. Development
- `orch verify`: Promote "Asserted" findings from minions to "Verified" project facts.
- `orch postmortem`: Generate a summary of the session's actions, tool usage, and costs.
