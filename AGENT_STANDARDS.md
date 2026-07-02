# Multi-Agent Coordination Standards (Task-Centric)

This repository is maintained concurrently by multiple AI agents (Antigravity, Antigravity, Codex, Claude). 

To ensure isolation and prevent conflicts, we use **Task-Based Worktrees**. This allows multiple agents—even of the same type—to work on different tasks simultaneously without file locks or build interference.

---

## 1. Task-Based Worktree Isolation

**DO NOT** modify code in the main `E:\rust-algo` directory.
**DO NOT** share a worktree between two different tasks.

### The Workflow:
1. **Identify Task:** Read `.agent/BOARD.md` to find a task or add a new one.
2. **Claim & Initialize:** Use the `task start <slug>` command. 
   - This creates a unique worktree at `../rust-algo-task-<slug>`.
   - This creates a dedicated branch `task/<slug>` off of `release/integrated-trading-platform`.
3. **Switch Context:** Immediately move your operation to that worktree.
4. **Execute:** Run builds and tests only within that isolated directory.
5. **Merge Back:** Once the task is complete, the branch `task/<slug>` MUST be merged back into `release/integrated-trading-platform`.

---

## 2. State Tracking (`.agent/BOARD.md`)

The board is the single source of truth for task ownership and worktree locations.

| Field | Description |
| :--- | :--- |
| **Task** | Short description of the goal. |
| **Agent** | The agent currently owning the task (e.g., `Claude-1`, `Antigravity-Alpha`). |
| **Status** | `Backlog`, `In Progress`, `Paused`, `Done`. |
| **Worktree** | The absolute path to the task's worktree. |

---

## 3. Handoffs & Resilience

If you stop working on a task before it is merged:
1. **Document:** Update `.agent/handoffs/<task-slug>.md` with current status and next steps.
2. **Commit:** Use `[PAUSE]` or `[HANDOFF]` in the commit message.
3. **Release:** Update the board to `Paused`. The worktree remains on disk for the next agent to resume.

---

## 4. Cleanup

Once a task is `Done` and the branch is merged into `main`:
1. Delete the worktree using `git worktree remove <path>`.
2. Remove the task entry from the active board.

## 5. Command Execution Standards (STRICT)
1. **Unix Commands Only:** You MUST use Linux/Unix commands strictly (e.g., `ls`, `grep`, `cat`, `rm`, `touch`, `cp`, `mv`). Ignore and do not use verbose PowerShell cmdlets (e.g., `Get-ChildItem`, `Select-String`).
2. **Missing Utilities:** If a standard Unix command is unavailable, you MUST attempt to install the corresponding utility using `winget` or `choco` before proceeding.
3. **PowerShell as Last Resort:** Only if no suitable Unix tool or package exists for the task should you employ a PowerShell command.