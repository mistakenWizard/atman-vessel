# Phase 6 Design: Platform Parity & Polish

## 1. Objectives
- Refine cross-platform reliability (Windows, WSL, Linux).
- Enhance UX with better visibility and setup automation.
- Implement maintenance and scaffolding features.

## 2. Technical Design

### 2.1 Platform Awareness
- **WSL Detection**: Implement `is_wsl() -> bool` by checking `/proc/version`.
- **Process Liveness**: Refine `Lock::is_live()` for Windows using a more robust check than `tasklist` if possible, or at least ensuring the parsing is resilient.
- **Path Parity**: Ensure `bootstrap_profile` handles path translation correctly if running in a mixed environment.

### 2.2 New Commands
- **`orch init`**: Scaffolds `.orchestrator/` directory structure and initializes a default `session.json`.
- **`orch status`**: Aggregates session info, active profile, lock status, and last handoff summary.
- **`orch profiles list`**: Lists all registered profiles in the current session.
- **`orch cleanup`**: Deletes stale locks and archives handoffs older than a configurable threshold.

### 2.3 UX Improvements
- **Colorized Output**: Use `anstyle` (already part of `clap`) to make status reports more readable.
- **Implicit Init**: `launch` should automatically trigger `init` logic if the environment is uninitialized.

## 3. TDD Requirements (Test Cases)

### Test Case 1: Scaffolding Integrity
- Given an empty directory.
- When `orch init` is called.
- Then `.orchestrator/locks`, `.orchestrator/profiles`, and `.orchestrator/session.json` must be created.

### Test Case 2: Status Report Accuracy
- Given an active session with a live lock.
- When `orch status` is run.
- Then it must correctly identify the active profile and show the live PID.

### Test Case 3: WSL Detection
- Given a system where `/proc/version` contains "Microsoft".
- When `is_wsl()` is called.
- Then it must return `true`.

### Test Case 4: Cleanup Safety
- Given a stale lock (Dead PID) and a live lock (Current PID).
- When `orch cleanup` is run.
- Then the stale lock must be removed and the live lock must remain.

## 4. Implementation Strategy
- **Agent 1 (Tester)**: Writes unit tests for init, status, and cleanup logic.
- **Agent 2 (Builder)**: Implements the new commands and platform-specific refinements.
