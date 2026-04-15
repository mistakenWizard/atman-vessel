# Codex CLI Agent Standards (Task-Centric)

## Multi-Agent Coordination 
**CRITICAL:** This project is shared with other agents. You MUST use Task-Based Worktrees.

### Execution Flow:
1. **Claim Task:** Run `task start <slug>` in the root directory.
2. **Switch Context:** `cd` to the path printed (e.g., `E:\rust-algo-task-xyz`).
3. **Verify:** Run `git branch` to ensure you are on `task/<slug>`.
4. **Work:** Perform all implementation and testing ONLY in this worktree.
5. **Pause:** If stopping, write to `.agent/handoffs/<slug>.md` and run `task handoff <slug>`.

**Never** work in the root `E:\rust-algo` directory.
**Always** check `D:\claude-setup\knowledge\` for global standards before starting.

### Verified Windows Workflow
- Preferred task command: `& .\scripts\task.ps1 start <slug>`
- Preferred verification command: `& .\scripts\cargo-check-windows.ps1`
- If `task` shell aliases are unavailable, use the PowerShell script directly.
- Rust worktrees may need access to repo-local `.tdlib` for compile/test flows.

## Command Execution Standards (STRICT)
1. **Prefer concise commands:** Use short commands when practical, but use repo-native PowerShell scripts when they are the authoritative workflow.
2. **Missing Utilities:** Install missing Unix tools via `winget` or `choco`.
3. **PowerShell is valid here:** Shared repo automation such as `scripts/task.ps1` and `scripts/cargo-check-windows.ps1` is preferred over ad hoc command substitutions.

## Tech Stack
- **Language:** Rust
- **Runtime:** Tokio
- **Broker:** Dhan API
- **Communication:** Telegram (tdlib-rs)

## Key Entry Points
- `src/main.rs` contains the root trading runtime and monitor loop.
- `src/option_selling/system.rs` contains the option-selling runtime and monitor loop.
- `src/trading/config.rs` and `src/option_selling/config.rs` contain the main runtime configuration surfaces.
