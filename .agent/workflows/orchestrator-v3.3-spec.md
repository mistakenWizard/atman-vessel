# Orchestrator v3.3 Specification: The Operational Rigor Model

## 1. Vision & Core Philosophy
The Orchestrator v3.3 is a **Deterministic Session Resumption** framework. It treats AI models as disposable specialized agents (**Vessels**) while maintaining a persistent, project-local "Soul" (**Context**). It prioritizes **Session Continuity** over **Limit Evasion**.

### 1.1 Core Principles
- **Soul (Context)**: The durable source of truth (project-local files).
- **Voice (Handoff)**: The ephemeral rehydration artifact (`HANDOFF.md`).
- **Vessel (Profile)**: The isolated execution environment for a specific role.
- **Deterministic State Ownership**: The orchestrator (machine) owns the state; models only propose updates.
- **Manual Compliance**: User-triggered switching as a standard operating procedure.

---

## 2. Technical Architecture

### 2.1 Experimental Launcher Strategy
Profile isolation is achieved through **HOME Redirection**, currently treated as an experimental prototype for Linux/WSL.
- **Validation Harness**: Before any launch, the `orch.sh` must execute a **Probe Harness**:
    1. Verify exact presence of `projects.json` and `google_accounts.json`.
    2. Check CLI version parity across profiles.
    3. Assert no "auth bleed" by listing sessions in a fresh profile.
- **Strategy**: Redirect `HOME` to `.orchestrator/profiles/<name>/`.
- **Fallback**: If the probe fails, the launcher aborts to avoid configuration corruption.

### 2.2 The Soul: Field-Level Provenance `session.json`
Every field in the state object is wrapped in metadata to ensure merge correctness.
```json
{
  "schema_version": "1.3.0",
  "session_id": "uuid-v4",
  "state_status": "running|paused|blocked|stuck|recovering",
  "active_worktree": {
    "path": "/path/to/worktree",
    "locked_by": "builder",
    "lock_acquired_at": "ISO-8601"
  },
  "state": {
    "goal": { "value": "...", "source": "user", "timestamp": "..." },
    "verified_facts": [
      { "value": "Tests pass in worktree X", "source": "test-output", "confidence": 1.0 }
    ],
    "next_actions": { "value": [], "source": "chief", "timestamp": "..." }
  }
}
```

---

## 3. Worktree & Mutation Safety
- **Single-Mutator Rule**: Only one profile may hold the lock on a specific worktree at a time.
- **Takeover Policy**: 
    - **Default**: New profile creates a new worktree (`git worktree add`).
    - **Manual**: User may grant "Adopt" permission to a takeover profile, which moves the lock.
- **Lock Storage**: Locks are stored in `.orchestrator/locks/` and are PID-sensitive.

---

## 4. Command Contracts

| Command | Input | Primary Side Effect | Permission |
| :--- | :--- | :--- | :--- |
| **`launch`** | Profile, Role | Creates/Bootstraps isolation dir. | User |
| **`freeze`** | Session ID | Writes `diff`, `logs`, and `session.json`. | Any |
| **`switch`** | Target Profile | Regenerates `HANDOFF.md` & redirects `HOME`. | User |
| **`recover`** | Crash Artifacts | Generates `RECOVERY.md` (read-only). | `recovery` |
| **`takeover`** | `RECOVERY.md` | Updates `session.json` & reassigns Worktree. | `chief` + User |
| **`postmortem`**| `RECOVERY.md` | Writes to `brain/post-mortems/`. | `ops` |

---

## 5. Privacy, Data & Governance

### 5.1 Data Classification
- **Level 1 (Public)**: Code, general documentation. (Stored in repo).
- **Level 2 (Internal)**: Backtest results, strategy logic. (Encrypted at rest).
- **Level 3 (Secret)**: Auth tokens, API keys. (Isolated in `.orchestrator/profiles/`, never committed).

### 5.2 Approval Gates
- **Global Policy**: Any change proposed by `ops` or `recovery` to the `~/.orchestrator/system/` layer requires **Explicit User Approval**.
- **Retention**: `transcripts/` and `logs/` have a 30-day sliding window before auto-archiving.

---

## 6. Tool Comparison (Revised)

| Tool | Core Focus | Overlap with v3.3 |
| :--- | :--- | :--- |
| **`aisw`** | Auth Switching | Profile/Credential isolation at the OS level. |
| **`CCManager`** | Worktree Mgmt | Management of multiple sessions and worktrees. |
| **v3.3** | **Context Integrity** | Unifies Auth Isolation with **Provenance-Aware** task state. |
