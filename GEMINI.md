# AtmanVessel Project Standards (v1.6.0)

## 1. Core Architecture: Soul & Minion
AtmanVessel is a deterministic orchestration framework that treats AI models as disposable **Minions** (Vessels) while maintaining a persistent **Soul** (Project State).

### Directory Map:
- `src-rust/`: Orchestrator Core (Rust 2024).
- `.orchestrator/`: Runtime state (session.json, locks, profiles). **DO NOT COMMIT.**
- `brain/`: Persistent intelligence (plans, post-mortems).
- `.agent/`: Coordination artifacts (BOARD.md, handoffs/).
- `scripts/`: Legacy shell probes and task shims.

### Documentation Index:
- [AGENT_GUIDE.md](AGENT_GUIDE.md): Operational standards for Minions.
- [MCP_GUIDE.md](MCP_GUIDE.md): Technical reference for the MCP server.
- [CLI_GUIDE.md](CLI_GUIDE.md): Detailed command reference.
- [BOOTSTRAP.md](BOOTSTRAP.md): Universal guide for cross-repo integration.

---

## 2. Multi-Minion Coordination (MCP)
**CRITICAL:** This project uses an MCP-based "Manager/Specialist" model.
- **The Manager**: Coordinates work via the MCP server (`orch mcp`).
- **Minions**: Spawned in isolated git worktrees.

### Execution Workflow:
1. **Initialize**: `orch init` (Scaffolds local state).
2. **Start Task**: `orch task start <slug>` (Creates worktree & launches minion).
3. **Worktree Isolation**: Perform all implementation/testing inside the `.worktrees/task-<slug>/` directory.
4. **Finish & Merge**: `orch task finish <slug>` (Commits and merges results back to the parent branch).
5. **Resource Cleanup**: `orch cleanup` (Automatically removes stale worktrees).

---

## 3. Engineering Standards (Rust Core)
- **Toolchain**: Rust 1.80+ (edition 2024).
- **Build**: `orch status` (Auto-builds) or `cd src-rust && cargo build`.
- **Test**: `cd src-rust && cargo test` (Mandatory before PRs).
- **Style**: Standard `cargo fmt`. Avoid unsafe code.

---

## 4. Operational "YOLO" Mode
All minion launches must be **Non-Interactive** and **Autonomous**.
- **Env Var**: `GEMINI_CLI_TRUST_WORKSPACE=true` is required for all automated operations.
- **Global Command**: Use `orch` (installed to `~/.local/bin/orch`) for all operations.
- **Timeout**: All autonomous operations have a **strict 60-second timeout**. Minions must checkpoint state frequently.

---

## 5. Multi-Account Identity
To rotate identities and bypass rate limits, use the named fleet:
1. `orch launch --profile <name> gemini -- -p "..."`
2. Authenticated Profiles: `gayatri`, `gvklive`, `hilarioushabit`, `hollowhistory`, `hypnotichallway`, `mistakenWizard`.
