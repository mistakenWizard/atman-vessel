# Phase 7 Design: Production-Grade Hardening & Compliance

## 1. Objectives
- Implement mandatory pre-flight safety gates for all launches.
- Automate worktree management for clean session isolation.
- Enforce data classification and privacy rules at the system level.
- Formalize the "Clone & Verify" state transition.

## 2. Technical Design

### 2.1 Integrated Pre-flight Probes
- **Module**: `orch::probes`
- **Logic**: Port the five bash probes (`isolation`, `platform`, `locks`, `schema`, `sync`) into native Rust.
- **Enforcement**: `orch launch` will execute all probes. If any probe returns `Fail`, the launch sequence is aborted with a diagnostic message.

### 2.2 Automated Worktree Management
- **Flag**: `orch launch --new-worktree` and `orch takeover` (implicit).
- **Command**: Execute `git worktree add -b <branch> <path> <source_branch>` via `std::process::Command`.
- **Naming**: Worktrees will follow the pattern `.worktrees/session-<id>-<profile>`.

### 2.3 Verification Promotion
- **Command**: `orch verify`
- **Logic**: Iterates through all fields in `SessionState` and promotes any item with status `Observed` or `Asserted` to `Verified`.
- **Constraint**: `orch takeover` remains in a restricted state (No new implementation work) until `orch verify` is successfully called.

### 2.4 Governance Hardening
- **Sanitization**: `Postmortem` logic will redact any `verified_facts` or `blockers` associated with `DataClassification::Secret` before copying to the global layer.
- **Approval Gate**: Any command writing to `~/.orchestrator/system/` will require an interactive `[y/N]` prompt or an `--approve` flag.
- **Restriction**: If `Session::classification` is `Secret`, global post-mortem copying is entirely disabled.

## 3. TDD Requirements (Test Cases)

### Test Case 1: Probe-Gated Launch
- Given a system where the `platform` probe would fail (mocked).
- When `orch launch` is run.
- Then the process must exit with code 1 and not spawn any child.

### Test Case 2: Automated Worktree Creation
- Given a valid git repository.
- When `orch launch --new-worktree` is called.
- Then a new worktree must be present on the filesystem and the lock must point to its absolute path.

### Test Case 3: Status Promotion
- Given a session with a fact marked as `Asserted`.
- When `orch verify` is executed.
- Then the fact's status in `session.json` must be `Verified`.

### Test Case 4: Data Redaction
- Given a session marked as `Confidential` with one `Secret` fact.
- When `orch postmortem` generates a global report.
- Then the `Secret` fact must be replaced with `[REDACTED]` or omitted.

## 4. Implementation Strategy
- **Agent 1 (Tester)**: Writes unit tests for probe gating and redaction logic.
- **Agent 2 (Builder)**: Implements the `probes` module and the governance enforcement logic.
