# AtmanVessel Project Standards (Task-Centric)

## Multi-Agent Coordination
**CRITICAL:** This project is shared with other agents. You MUST use Task-Based Worktrees.

### Execution Flow:
1. **Claim Task**: Run `task start <slug>` in the root directory.
2. **Switch Context**: `cd` to the task-specific directory (e.g., `E:\atman-vessel-task-xyz`).
3. **Work**: Perform implementation and testing in the isolated worktree.
4. **Pause/Handoff**: Write to `.agent/handoffs/<slug>.md` and run `task handoff <slug>`.

---

## Core Mandate: Deterministic Continuity
Every action taken in this repository must prioritize **Task Continuity** and **Context Integrity**.

### Operational Standards:
1. **State Persistence**: All task progress must be mirrored in `.orchestrator/session.json`.
2. **Provenance**: Every update to the soul must include `source`, `session_id`, and `timestamp`.
3. **Isolation**: Always use `orch.sh` for profile-level execution to ensure identity isolation.
4. **Handoffs**: Before switching vessels, update `HANDOFF.md` with the "Mental Map" of the task.

## Recovery Protocol
In the event of a crash or stall:
1. Snapshot current git diff and filesystem status.
2. Generate `RECOVERY.md`.
3. Assign a takeover profile using the "Clone & Verify" rule.

## Prototyping Guardrails
- **HOME Redirection**: Treat profile isolation as an experimental prototype requiring a probe harness.
- **Bootstrapping**: Manual config copies must be verified for CLI version compatibility.

## Planning
All plans, post-mortems, and architectural decisions must be stored in the `brain/` directory.
