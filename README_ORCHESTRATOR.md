# AtmanVessel Orchestrator (v1.2.0)

Deterministic Session Resumption and Context Integrity for Multi-Agent Development.

## Core Commands

### 1. Project Initialization
```bash
./orch.sh init
```
Scaffolds the `.orchestrator/` directory and initializes the canonical `session.json`.

### 2. Launching a Session
```bash
./orch.sh launch --profile builder --new-worktree
```
- Runs pre-flight safety probes.
- Bootstraps Gemini/Codex auth files.
- Automatically creates a new isolated git worktree.
- Launches the target CLI with `HOME` redirection.

### 3. Context Management
- **Freeze**: `echo "Finished task X" | ./orch.sh freeze --message -`
- **Verify**: `./orch.sh verify` (Promotes asserted facts to verified status).
- **Status**: `./orch.sh status` (Detailed session and lock diagnostics).

### 4. Recovery & Takeover
- **Recover**: `./orch.sh recover` (Generates `RECOVERY.md` from crashed state).
- **Takeover**: `./orch.sh takeover --profile reviewer` (Safe worktree adoption).

### 5. Governance
- **Classification**: `./orch.sh governance set-class --level confidential`
- **Post-Mortem**: `./orch.sh postmortem` (Generates sanitized global reports).
- **Charters**: `./orch.sh charter list` (View available agent roles).

## Safety Principles
- **Deep Provenance**: Every state update records who, when, and how.
- **Single-Mutator Rule**: PID-based locks prevent concurrent mutation of the same worktree.
- **Clone & Verify**: Takeover requires explicit verification before new edits are permitted.
