# Phase 5 Design: Global System & Compliance

## 1. Objectives
- Establish a durable global configuration layer outside individual repositories.
- Formalize agent roles via the "Charter" system.
- Implement structured incident analysis and improvement loops.
- Prepare the system for cryptographic provenance verification.

## 2. Technical Design

### 2.1 Global System Layer
- **Base Path**: `~/.orchestrator/system/` (using `dirs` crate for cross-platform expansion).
- **Structure**:
    - `charters/`: YAML/JSON definitions of agent roles.
    - `postmortems/`: Aggregated analysis of failed sessions.
    - `config.json`: Global settings (default classification, retention).
- **Mechanism**: The `orch` binary will prioritize repo-local `.orchestrator/` but fall back to or aggregate with global settings.

### 2.2 Agent Charter Architecture
- **Struct**: `Charter { name, primary_role, secondary_capabilities, constraints }`
- **Command**: `orch charter list` / `orch charter show <name>`
- **Logic**: Used to suggest the best profile for a task during `launch` or `switch`.

### 2.3 Post-Mortem Implementation
- **Command**: `orch postmortem`
- **Logic**:
    1. Scan `.orchestrator/crash/` (if it exists) or use the last aborted session.
    2. Collate verified facts vs. narrative blockers.
    3. Generate a structured report in `brain/post-mortems/` and optionally copy it to global `~/.orchestrator/system/postmortems/`.

### 2.4 Cryptographic Readiness
- **Command**: `orch sign --session <id>` (Stub)
- **Model**: Add a `signature: Option<String>` field to the `Provenance` struct to support future multi-key verification.

## 3. TDD Requirements (Test Cases)

### Test Case 1: Global Path Resolution
- Given a system with `~/.orchestrator/system/config.json` existing.
- When `orch` is run.
- Then it should correctly identify the global path for settings.

### Test Case 2: Charter Retrieval
- Given a charter file `builder.json` in the global charters directory.
- When `orch charter show builder` is executed.
- Then it should return the correct role and capabilities.

### Test Case 3: Signature Field Presence
- When a `Provenance` object is serialized to JSON.
- Then it must optionally include a `signature` field without breaking legacy parses.

## 4. Implementation Strategy
- **Agent 1 (Tester)**: Writes unit tests for global path expansion and charter loading.
- **Agent 2 (Builder)**: Implements the `Charter` struct and the global layer logic.
