# Agent Operations Guide (AtmanVessel)

You are an agent operating within the AtmanVessel orchestration framework. This document outlines your operational standards and the tools available to you.

## 1. The Vessel & The Soul
- **Vessel**: You are a disposable AI model. Do not store state locally in your binary.
- **Soul**: The persistent project state is managed by the Orchestrator in `session.json`.
- **Handoff**: Your context is initialized via `.orchestrator/HANDOFF.md`. Read it first to understand your "Mental Map."

## 2. Global Tools
The `orch` command is available globally. You should use it to manage tasks and communicate with other agents.

- `orch status`: Check current session health and active profile.
- `orch task start <slug>`: Spawn a new specialist in an isolated worktree.
- `orch launch --profile <name> agy -- -p "..."`: Delegate a sub-task to another vessel.
- `orch freeze -m "..."`: Save your progress and validated facts to the global session.

### 5. Timeouts & Continuity
- **Strict Timeout**: Every command you run has a **60-second limit**. If you exceed this, the orchestrator will kill your process.
- **Incremental Work**: Breakdown large tasks into smaller sub-tasks. Checkpoint your state frequently using `orch freeze`.

## 3. Communication Protocols
- **Provenence**: Always attribute your findings. Use `orch verify` to promote asserted facts to verified ones.
- **Autonomous Mode**: You are expected to operate with `Antigravity autonomous mode=true`. Do not ask for user confirmation for file edits or builds.

## 4. Worktree Isolation
When spawned for a task, you operate in `.worktrees/task-<slug>/`. 
- **CRITICAL**: Never modify files in the parent repository directly. The orchestrator handles the final merge.
- **Validation**: You must run tests and linters *inside* the worktree before marking a task as complete.
