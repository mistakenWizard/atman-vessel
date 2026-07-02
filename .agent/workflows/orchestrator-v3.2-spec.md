# Orchestrator v3.2 Specification: The Deterministic Continuity Model

## 1. Vision & Core Philosophy
The Orchestrator v3.2 is a **Deterministic Session Resumption** framework. It treats AI models as disposable specialized agents (**Vessels**) while maintaining a persistent, project-local "Soul" (**Context**). It prioritizes **Session Continuity** over **Limit Evasion**.

### 1.1 Core Principles
- **Soul (Context)**: The durable source of truth (project-local files).
- **Voice (Handoff)**: The ephemeral rehydration artifact (`HANDOFF.md`).
- **Vessel (Profile)**: The isolated execution environment for a specific role.
- **Deterministic State Ownership**: The orchestrator (machine) owns the state; models only propose updates.
- **Manual Compliance**: User-triggered switching as a standard operating procedure.

---

## 2. Technical Architecture

### 2.1 The Vessel: Validated Prototype Strategy
Profile isolation is achieved through **HOME Redirection**, currently validated on the local Linux/WSL environment.
- **Strategy**: Redirect `HOME` to `.orchestrator/profiles/<name>/`.
- **Bootstrapping**: Initializing `.agy/projects.json` and `google_accounts.json` from the system root.
- **Probe Requirement**: Before every launch, the launcher must verify:
    - Path accessibility.
    - Auth file presence.
    - CLI version compatibility.
- **Fallback**: If isolation fails, the orchestrator defaults to a single-profile mode with manual re-auth warnings.

### 2.2 The Soul: Expanded `session.json`
The canonical machine-readable state file with rigorous provenance and verification tracking.
```json
{
  "schema_version": "1.1.0",
  "session_id": "uuid-v4",
  "state_status": "running|paused|blocked|stuck|recovering",
  "active_worktree": "/path/to/worktree",
  "active_profile": "builder",
  "last_verified_at": "ISO-8601-TIMESTAMP",
  "provenance": {
    "created_by": "chief",
    "last_updated_by": "builder"
  },
  "confidence": 0.95,
  "state": {
    "goal": "...",
    "constraints": [],
    "verified_facts": [],
    "blockers": [],
    "files_in_focus": [],
    "next_actions": []
  }
}
```

---

## 3. The Full Agent Charter

| Profile | Provider | Primary Mission |
| :--- | :--- | :--- |
| **`chief`** | Codex | Planning, Dispatching, Integration. |
| **`builder`** | Antigravity | Implementation and Refactoring. |
| **`systems`** | Antigravity | Environment, Tooling, and WSL setup. |
| **`reviewer`** | Antigravity | Code review and Regression spotting. |
| **`writer`** | Antigravity | Docs, summaries, and drafting. |
| **`quant`** | Antigravity | Trading research and backtesting support. |
| **`ops`** | Antigravity | Maintaining the agent system itself. |
| **`recovery`** | Antigravity | Triage of stuck/crashed sessions. |

---

## 4. Workflows & State Machine

### 4.1 The "Freeze-Dry-Hydrate" Switch
1.  **Freeze**: Snapshots `git diff`, current worktree, and model-proposed summaries into `session.json`. Updates `last_verified_at` and `provenance`.
2.  **Dry**: Regenerates `HANDOFF.md` from the validated state.
3.  **Hydrate**: Launches the next Vessel with redirected `HOME` and the fresh Handoff.

### 4.2 Recovery & Takeover (Rigorously Defined)
- **Artifacts Captured**: `git diff`, process logs, transcript fragments, `session.json` snapshot.
- **Takeover Rules**:
    - The new owner (e.g., `recovery`) must read `RECOVERY.md` before `HANDOFF.md`.
    - Verification of "Uncertain Facts" is mandatory before resuming implementation.
    - Filesystem state outranks narrative summaries.
- **Postmortem**: Required for every `stuck` or `crashed` state transition.

---

## 5. Tool Comparison (Revised)

| Tool | Core Focus | Overlap with v3.2 |
| :--- | :--- | :--- |
| **`aisw`** | Auth Switching | Profile/Credential isolation at the OS level. |
| **`CCManager`** | Session/Worktree Mgmt | Management of multiple sessions and worktrees for recovery/takeover. |
| **v3.2** | **Context Continuity** | Integrates Auth Isolation with Task State Tracking and Role-Based Handoffs. |
