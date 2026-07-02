# AtmanVessel Project Standards (v1.5.0)

## 1. Core Architecture: Soul & Vessel
AtmanVessel is a deterministic orchestration framework that treats AI models as disposable **Vessels** while maintaining a persistent **Soul** (Project State).

### Directory Map:
- `src-rust/`: Orchestrator Core (Rust 2024).
- `.orchestrator/`: Runtime state (session.json, locks, profiles). **DO NOT COMMIT.**
- `brain/`: Persistent intelligence (plans, post-mortems).
- `.agent/`: Coordination artifacts (BOARD.md, handoffs/).
- `scripts/`: Legacy shell probes and task shims.

### Documentation Index:
- [AGENT_GUIDE.md](AGENT_GUIDE.md): Operational standards for AI agents.
- [MCP_GUIDE.md](MCP_GUIDE.md): Technical reference for the MCP server.
- [CLI_GUIDE.md](CLI_GUIDE.md): Detailed command reference.
- [BOOTSTRAP.md](BOOTSTRAP.md): Universal guide for cross-repo integration.

---

## 2. Multi-Agent Coordination (MCP)
**CRITICAL:** This project uses an MCP-based "Manager/Specialist" model.
- **The Manager**: Coordinates work via the MCP server (`orch mcp`).
- **Specialists**: Spawned in isolated git worktrees.

### Execution Workflow:
1. **Initialize**: `orch init` (Scaffolds local state).
2. **Start Task**: `orch task start <slug>` (Creates worktree & launches agent).
3. **Worktree Isolation**: Perform all implementation/testing inside the `.worktrees/task-<slug>/` directory.

---

## 3. Engineering Standards (Rust Core)
- **Toolchain**: Rust 1.80+ (edition 2024).
- **Build**: `orch status` (Auto-builds) or `cd src-rust && cargo build`.
- **Test**: `cd src-rust && cargo test` (Mandatory before PRs).
- **Style**: Standard `cargo fmt`. Avoid unsafe code.

---

## 4. Operational "YOLO" Mode
All agent launches must be **Non-Interactive** and **Autonomous**.
- **Env Var**: `Antigravity autonomous mode=true` is required for all automated operations.
- **Global Command**: Use `orch` (installed to `~/.local/bin/orch`) for all operations.
- **Timeout**: All autonomous operations have a **strict 60-second timeout**. Agents must checkpoint state frequently.

---

## 5. Multi-Account Identity
To rotate identities and bypass rate limits, use the named fleet:
1. `orch launch --profile <name> agy -- -p "..."`
2. Authenticated Profiles: `gayatri`, `gvklive`, `hilarioushabit`, `hollowhistory`, `hypnotichallway`, `mistakenWizard`.
