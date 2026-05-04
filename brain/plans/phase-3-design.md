# Phase 3 Design: Deep Continuity & Recovery

## 1. Objectives
- Ensure state consistency across multiple terminals.
- Provide a safe, deterministic path for session recovery and takeover.
- Implement automated recovery artifact generation.

## 2. Technical Design

### 2.1 Multi-Terminal Sync
- **Mechanism**: The `Session` struct will track `loaded_at` (runtime only).
- **Validation**: Before every `save()`, the orchestrator will reload the `session.json` from disk and compare its `last_updated` timestamp with the `last_updated` timestamp of the current runtime session.
- **Conflict Handling**: If the disk timestamp is newer, the orchestrator will refuse to save and force a `reload()` to prevent state regression.

### 2.2 Recovery Generator
- **Method**: `Session::generate_recovery(&self) -> String`
- **Output**: `RECOVERY.md` containing:
    - Last known good state.
    - Current diff (if possible to capture).
    - PID of the failed process.
    - Actionable "Clone & Verify" instructions for the next agent.

### 2.3 Clone & Verify Takeover
- **Command**: `orch takeover --profile <name>`
- **Logic**:
    1. Identify existing lock for the profile.
    2. Verify lock is stale OR user has explicitly authorized takeover.
    3. Generate a new task-based worktree (The "Clone").
    4. Verify the state in the new worktree matches `session.json` (The "Verify").
    5. Acquire a new lock and update `active_profile` provenance.

## 3. TDD Requirements (Test Cases)

### Test Case 1: Stale State Detection
- Given a session loaded at T1.
- When `session.json` is updated on disk at T2 (where T2 > T1).
- Then `session.save()` should return an error "State on disk is newer than loaded state".

### Test Case 2: Recovery Artifact Integrity
- Given a session in a `Crashed` or `Stuck` state.
- When `session.generate_recovery()` is called.
- Then the resulting string must contain the `session_id` and the `last_updated` timestamp.

### Test Case 3: Takeover Lock Replacement
- Given an existing live lock for Profile A.
- When `orch takeover --profile A` is called.
- Then it should fail unless the lock is stale OR a `--force` flag is used.
- On success, the old lock file must be replaced with a new lock containing the current PID.

## 4. Implementation Strategy
- **Agent 1 (Tester)**: Writes unit tests in `src-rust/src/tests.rs` (or similar) using mock filesystem/state.
- **Agent 2 (Builder)**: Implements the logic in `src-rust/src/main.rs` to satisfy the tests.
