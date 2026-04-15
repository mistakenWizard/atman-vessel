# Orchestrator v3.4 Specification: The Production-Ready Model

## 1. Vision & Core Philosophy
The Orchestrator v3.4 is a **Deterministic Session Resumption** framework. It treats AI models as disposable specialized agents (**Vessels**) while maintaining a persistent, project-local "Soul" (**Context**). It prioritizes **Session Continuity** over **Limit Evasion**.

### 1.1 Core Principles
- **Soul (Context)**: Durable, provenance-aware source of truth (project-local).
- **Voice (Handoff)**: Ephemeral rehydration artifact (`HANDOFF.md`).
- **Vessel (Profile)**: Isolated, role-based execution environment.
- **Deterministic State Ownership**: The orchestrator (machine) owns the state; models only propose updates.
- **Manual Compliance**: User-triggered switching as a standard operating procedure.

---

## 2. Technical Architecture

### 2.1 Experimental Launcher Strategy
Profile isolation is achieved through **HOME Redirection**, currently treated as an experimental prototype for Linux/WSL.
- **Probe Harness**: Before any launch, `orch.sh` must:
    1. Verify `projects.json` and `google_accounts.json` integrity.
    2. Assert CLI version parity.
    3. Confirm no "auth bleed" via isolated session-list check.
- **Strategy**: Redirect `HOME` to `.orchestrator/profiles/<name>/`.
- **Fallback**: Launcher aborts on any probe failure.

### 2.2 The Soul: Deep Field-Level Provenance `session.json`
Every field is wrapped in metadata to ensure merge correctness and prevent state corruption.
```json
{
  "schema_version": "1.4.0",
  "session_id": "uuid-v4",
  "state_status": "running|paused|blocked|stuck|recovering",
  "active_worktree": {
    "path": "/path/to/worktree",
    "locked_by": "builder",
    "lock_acquired_at": "ISO-8601"
  },
  "state": {
    "goal": { 
      "value": "...", 
      "source": "user", 
      "session_id": "s0", 
      "timestamp": "...", 
      "confidence": 1.0, 
      "status": "verified" 
    },
    "verified_facts": {
      "merge_strategy": "append-unique",
      "items": [
        { "value": "...", "source": "test-output", "session_id": "s1", "confidence": 1.0, "status": "verified" }
      ]
    },
    "next_actions": { 
      "value": [], 
      "source": "chief", 
      "session_id": "s1", 
      "timestamp": "...", 
      "confidence": 0.8, 
      "status": "asserted" 
    }
  }
}
```
**Status Levels**: `observed` (raw output), `asserted` (model-derived), `verified` (test/user-confirmed).

---

## 3. Worktree & Takeover Safety

### 3.1 Single-Mutator Rule
Only one profile may hold the lock on a specific worktree. Locks are stored in `.orchestrator/locks/`.

### 3.2 The "Clone & Verify" Takeover Rule
1. **Isolation**: New profile creates a new worktree (`git worktree add`).
2. **Verification Phase**: The takeover agent **must** execute a verification suite (tests, diff inspection) to confirm the prior state.
3. **Activation**: New mutations are **prohibited** until all relevant state fields are promoted to `verified` status in the new worktree.

---

## 4. Command Contracts

| Command | Owner | Side Effect |
| :--- | :--- | :--- |
| **`launch`** | Orchestrator/User | Creates/Bootstraps isolation dir & Redirects `HOME`. |
| **`freeze`** | Orchestrator/User | Writes `diff`, `logs`, and `session.json`. Models only **propose** content. |
| **`switch`** | Orchestrator/User | Regenerates `HANDOFF.md` and transitions Worktree Lock. |
| **`recover`** | `recovery` (Profile) | Generates `RECOVERY.md` (Read-Only). |
| **`takeover`**| `chief` + User | Promotes `observed` state to `verified` after Clone & Verify. |
| **`postmortem`**| `ops` (Profile) | Local: `brain/post-mortems/`. Global: `~/.orchestrator/system/postmortems/` (Sanitized). |

---

## 5. Privacy, Data & Governance

### 5.1 Domain-Aware Classification
- **Code/Implementation**: Repo-local. (Public/Internal).
- **Trading/Research**: Repo-local or encrypted. (Internal/Confidential).
- **Personal/Writing**: Isolated in `.orchestrator/profiles/`, never committed. (Confidential).
- **Legal/Drafting**: No-retention policy or local encryption. (Secret).

### 5.2 Approval Gates
- **Global Policy**: Any change to `~/.orchestrator/system/` (routing, prompts, scripts) requires **Explicit User Approval**.
- **Redaction**: Global postmortems must be sanitized of all Domain-Level Confidential/Secret data.

---

## 6. Tool Comparison Summary

| Tool | Focus | Differentiator |
| :--- | :--- | :--- |
| **`aisw`** | Auth Switching | Credential isolation. |
| **`CCManager`** | Worktree Mgmt | Session and worktree handling. |
| **v3.4** | **Context Integrity** | Unifies Auth Isolation with **Deep Provenance** and **Clone & Verify** takeover. |
