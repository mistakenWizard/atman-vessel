# AtmanVessel Project Standards (Task-Centric)

## Core Mandate: Deterministic Continuity
Every action taken in this repository must prioritize **Task Continuity** and **Context Integrity**.

### Operational Flow:
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
