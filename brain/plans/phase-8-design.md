# Phase 8 Design: System Bootstrapping & Defaults

## 1. Objectives
- Automatically populate the global system layer with default agent charters.
- Implement the `resume` command to streamline session rehydration.
- Formalize the system's "Self-Bootstrapping" capability.

## 2. Technical Design

### 2.1 Default Charter Initialization
- **Module**: `orch::defaults`
- **Roles to Implement**:
    - `chief`: Planning, dispatching, and integration.
    - `builder`: Implementation and refactoring.
    - `systems`: Environment, tooling, and setup.
    - `reviewer`: Code review and regression spotting.
    - `writer`: Documentation and drafting.
    - `quant`: Trading research and backtesting.
    - `ops`: Agent system maintenance.
    - `recovery`: Stuck/crashed session triage.
- **Logic**: During `orch init`, the orchestrator will check `~/.orchestrator/system/charters/`. If empty, it will write the default JSON definitions for these roles.

### 2.2 Resume Command
- **Command**: `orch resume [--profile <name>]`
- **Logic**:
    1. Load `session.json`.
    2. If a profile is specified, verify its lock status.
    3. If no profile is specified, use the `active_profile` from `session.json`.
    4. Transition state from `Paused` back to `Running`.
    5. Regenerate `HANDOFF.md` to ensure the model has the latest context.
    6. **Diff**: Unlike `launch`, `resume` assumes the environment is already probed and isolated.

### 2.3 System Integrity Check
- **Feature**: `orch check`
- **Logic**: Verifies that the global layer and current repository state are consistent.

## 3. TDD Requirements (Test Cases)

### Test Case 1: Charter Bootstrapping
- Given an empty global system directory.
- When `orch init --global` (or implicit init) is called.
- Then the `charters/` directory must contain exactly 8 default JSON files.

### Test Case 2: Resume Transition
- Given a session in the `Paused` state.
- When `orch resume` is executed.
- Then the `session.json` state must be `Running` and the timestamp updated.

### Test Case 3: Resume without Active Profile
- Given a new session with no active profile.
- When `orch resume` is called without a `--profile` flag.
- Then it must return an error instructing the user to use `launch` or specify a profile.

## 4. Implementation Strategy
- **Agent 1 (Tester)**: Writes unit tests for charter population and resume state transitions.
- **Agent 2 (Builder)**: Implements the defaults module and the resume command logic.
