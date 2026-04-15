# Orchestrator v3.1 Specification: The Unified Soul Model

## 1. Vision & Core Philosophy
The Orchestrator v3.1 is a **Deterministic Session Resumption** framework. It treats AI models as disposable specialized agents (Vessels) while maintaining a persistent, project-local "Soul" (Context). It prioritizes **Session Continuity** over **Limit Evasion**.

### 1.1 Key Principles
- **Role-Based Specialization**: Profiles are functional (`chief`, `builder`, `auditor`) rather than just numerical (`acc_1`, `acc_2`).
- **Deterministic State**: The orchestrator (machine) owns the state (`session.json`); models only propose updates.
- **Physical Isolation**: Robust `HOME` redirection and configuration bootstrapping prevent CLI crashes and auth "bleeding."
- **Manual Compliance**: All profile switches are user-triggered, ensuring alignment with Terms of Service.

---

## 2. Technical Architecture

### 2.1 The Vessel (`orch.sh`)
The "Vessel" is the execution environment. It uses **HOME Redirection** to isolate each profile's configuration.
- **Path**: `.orchestrator/profiles/<profile_name>/`
- **Bootstrapping**: Automatically copies `projects.json` and `google_accounts.json` from the system `HOME` to the profile `HOME` to ensure stable CLI initialization.
- **Command Set**: `launch`, `list`, `status`, `switch`, `freeze`, `recover`.

### 2.2 The Soul (`session.json`)
The canonical machine-readable state file. It is the single source of truth for the task.
```json
{
  "task_id": "...",
  "active_profile": "builder",
  "state": {
    "goal": "...",
    "verified_facts": [],
    "blockers": [],
    "files_in_focus": [],
    "next_actions": []
  },
  "history": [
    { "timestamp": "...", "profile": "chief", "event": "plan_created" }
  ]
}
```

### 2.3 The Voice (`HANDOFF.md`)
A model-readable Markdown summary generated dynamically from `session.json`. It is the "Mental Map" for rehydrating intuition at the start of every session.

---

## 3. Profiles & Roles

| Profile | Provider | Primary Role | Secondary Role |
| :--- | :--- | :--- | :--- |
| **`chief`** | Codex | Planning, Design, Integration | Code Review |
| **`builder`** | Gemini | Implementation, Refactoring | Prototyping |
| **`auditor`** | Gemini | Testing, Verification, Triage | Bug Hunting |
| **`researcher`** | Gemini | Documentation, Analysis | Post-mortems |

---

## 4. Workflows

### 4.1 The "Freeze-Dry-Hydrate" Switch
When moving from one profile to another:
1.  **Freeze**: Captures current `git diff`, recent command outputs, and the model's summary into `session.json`.
2.  **Dry**: Generates a fresh `HANDOFF.md` from the updated state.
3.  **Hydrate**: Launches the new profile with a prompt: *"Read `.orchestrator/HANDOFF.md` and resume from 'Next Atomic Steps'."*

### 4.2 Recovery Mode
In the event of a CLI crash or model loop:
1.  The orchestrator snapshots the filesystem and git status.
2.  It generates a `RECOVERY.md` detailing confirmed vs. uncertain progress.
3.  The user selects a takeover profile (e.g., `researcher`) to triage the failure.

---

## 5. Compliance Alignment
- **No Automated Failover**: Switches occur only upon user request or manual confirmation.
- **No Load Balancing**: The system does not attempt to spread tokens evenly across profiles to bypass quotas.
- **Legitimate Identities**: Uses standard CLI authentication flows and persists them in local, secure profile directories.
