# Orchestrator Master Specification

## 1. Purpose

Build a local AI orchestration system for development and knowledge work that:

- uses `Codex CLI` as the primary planner, dispatcher, and integrator
- supports multiple pre-authenticated `Antigravity CLI` profiles as specialized workers
- preserves task continuity across profile, provider, terminal, and crash boundaries
- avoids repeated login friction
- supports coding, docs, personal writing, trading research, backtesting, and system maintenance
- works across Windows, Linux, macOS, and WSL where provider CLIs support it

The system is for `session continuity`, `role specialization`, and `recoverability`.
It is not an automatic quota balancer.

## 2. Core Philosophy

The orchestrator is a `Deterministic Session Resumption` framework.

It treats AI models as disposable specialized agents while keeping durable context outside model chat history.

Three concepts define the architecture:

- `Soul`: durable, provenance-aware source of truth stored locally
- `Voice`: ephemeral handoff/recovery artifacts generated from the Soul
- `Vessel`: isolated, role-based execution environment for a provider/profile

Rules:

- the orchestrator owns canonical state
- models only propose updates
- `HANDOFF.md` and `RECOVERY.md` are derived artifacts
- filesystem, git, test output, and user input outrank model summaries
- profiles are replaceable; context is not

## 3. Compliance Boundary

The design stays conservative.

Allowed:

- pre-authenticate Antigravity profiles up front
- manually trigger switching between profiles
- automatically carry forward context through project-local state
- route work by real role or environment fit

Not allowed in this design:

- automatic failover from one consumer account to another
- proactive round-robin across equivalent consumer accounts
- randomized routing across equivalent consumer accounts
- background load balancing across consumer identities
- selecting profiles to avoid per-profile restrictions

Safe framing:

- "switch to the reviewer"
- "switch to the WSL specialist"
- "switch to the recovery agent"

Risky framing:

- "pick whichever Antigravity account should handle this so no account hits limits"

OpenAI’s terms explicitly prohibit circumventing rate limits or restrictions.
For Antigravity, the exact multi-profile workflow is not explicitly blessed or banned in the sources reviewed, so the system uses explicit user-triggered switching and avoids load balancing.

## 4. High-Level Architecture

The system has six major layers.

### 4.1 State Core

Canonical project-local state:

```text
.orchestrator/session.json
```

This is the `Soul`.

### 4.2 Handoff And Recovery Artifacts

Generated files:

```text
.orchestrator/HANDOFF.md
.orchestrator/RECOVERY.md
```

These are the `Voice`.

### 4.3 Profile Registry

Named profiles with:

- provider
- role
- launch command
- execution target
- profile root
- data classification defaults

These are the `Vessels`.

### 4.4 Worktree And Locking Layer

Ensures code-changing work is safe:

- one mutating agent per worktree
- explicit locks
- clone-and-verify takeover

### 4.5 Launcher Layer

Provider-specific launching and isolation.

This layer is experimental until validated by probes.

### 4.6 Global System Layer

Cross-repo memory and improvements:

```text
~/.orchestrator/system/
```

Only sanitized or approved material may enter this layer.

## 5. Agent Charters

Profiles should be specialized but flexible.

Each profile has:

- primary mission
- secondary capabilities
- do-not-use cases
- data handling rules

### `chief` (`Codex`)

Primary mission:

- planning
- dispatching
- integration

Responsibilities:

- break work into subtasks
- assign specialists
- maintain coherent state
- approve takeover direction
- approve global policy changes where needed

### `builder` (`Antigravity`)

Primary mission:

- implementation
- refactoring

Secondary capabilities:

- prototyping
- debugging

### `systems` (`Antigravity`)

Primary mission:

- environment and tooling
- Windows/WSL setup
- shell and PowerShell work

Secondary capabilities:

- scripts
- aliases
- local automation
- CI/developer tooling

### `reviewer` (`Antigravity`)

Primary mission:

- code review
- regression spotting
- verification

Secondary capabilities:

- risk analysis
- architecture critique
- test-gap analysis

### `writer` (`Antigravity`)

Primary mission:

- docs
- summaries
- drafting

Secondary capabilities:

- emails
- onboarding docs
- repo docs
- personal/legal drafting support

Boundary:

- legal material is drafting support only, not legal advice

### `quant` (`Antigravity`)

Primary mission:

- trading research
- market analysis
- backtesting support

Secondary capabilities:

- experiment design
- result summaries
- data preparation

Boundary:

- research support only, not autonomous trading authority

### `ops` (`Antigravity`)

Primary mission:

- improve the agent system itself

Secondary capabilities:

- prompt templates
- workflow scripts
- handoff improvements
- cross-repo standards
- usage hygiene

### `recovery` (`Antigravity`)

Primary mission:

- recover stuck, crashed, or abandoned sessions

Secondary capabilities:

- failure diagnosis
- takeover handoff preparation
- postmortem drafting

## 6. Routing Rules

Routing is based on fit, not fairness.

Examples:

- implementation task -> `builder`
- Windows, WSL, shell, path, setup issue -> `systems`
- patch review or verification -> `reviewer`
- docs, emails, summaries -> `writer`
- trading research or backtesting -> `quant`
- system prompt/script improvements -> `ops`
- stuck/crashed session -> `recovery`

Rules:

- profiles can have secondary capabilities
- profiles do not need equal utilization
- user can override routing
- the scheduler must not optimize for quota spreading

## 7. Experimental Launcher Strategy

Profile isolation is a probe-validated implementation strategy, not a guaranteed provider contract.

Candidate strategy:

- redirect `HOME` to `.orchestrator/profiles/<name>/`
- isolate provider state per profile
- bootstrap only files proven necessary by probes

Current Antigravity prototype checks may include:

- `projects.json`
- `google_accounts.json`
- CLI version parity
- auth/session visibility from a fresh profile

These checks are environment-specific until proven stable.

### Probe Harness

Before any launch, probes should validate:

1. profile path accessibility
2. expected auth/config/session files
3. CLI version compatibility
4. no auth bleed between profiles
5. Windows vs WSL behavior
6. provider-native session features

Failure policy:

- abort launch on probe failure
- do not silently run with partial isolation
- fall back to single-profile mode with explicit warning

## 8. Canonical State Model

`session.json` is canonical.

Every important field should carry field-level metadata.

```json
{
  "schema_version": "1.4.0",
  "session_id": "uuid-v4",
  "state_status": "planned|running|paused|blocked|stuck|crashed|recovering|takeover|resumed|completed|aborted",
  "active_profile": {
    "value": "builder",
    "source": "chief",
    "session_id": "s1",
    "timestamp": "ISO-8601",
    "confidence": 1.0,
    "status": "verified"
  },
  "active_worktree": {
    "value": {
      "path": "/path/to/worktree",
      "locked_by": "builder",
      "lock_acquired_at": "ISO-8601"
    },
    "source": "orchestrator",
    "session_id": "s1",
    "timestamp": "ISO-8601",
    "confidence": 1.0,
    "status": "verified"
  },
  "state": {
    "goal": {
      "value": "...",
      "source": "user",
      "session_id": "s0",
      "timestamp": "ISO-8601",
      "confidence": 1.0,
      "status": "verified"
    },
    "verified_facts": {
      "merge_strategy": "append-unique",
      "items": [
        {
          "value": "tests pass in worktree X",
          "source": "test-output",
          "session_id": "s1",
          "timestamp": "ISO-8601",
          "confidence": 1.0,
          "status": "verified"
        }
      ]
    },
    "blockers": {
      "merge_strategy": "append-unique",
      "items": [
        {
          "value": "retry metadata not persisted",
          "source": "model-summary",
          "session_id": "s1",
          "timestamp": "ISO-8601",
          "confidence": 0.5,
          "status": "asserted"
        }
      ]
    },
    "files_in_focus": {
      "value": [],
      "source": "git",
      "session_id": "s1",
      "timestamp": "ISO-8601",
      "confidence": 0.9,
      "status": "observed"
    },
    "next_actions": {
      "value": [],
      "source": "chief",
      "session_id": "s1",
      "timestamp": "ISO-8601",
      "confidence": 0.8,
      "status": "asserted"
    }
  },
  "last_verified_at": "ISO-8601"
}
```

Status levels:

- `observed`: raw filesystem, git, log, command, or test output
- `asserted`: model-derived or unverified human/model claim
- `verified`: test-confirmed, user-confirmed, or otherwise validated

## 9. State Merge Rules

Merge priority:

1. user input
2. filesystem state
3. git state
4. test output / command output
5. verified recovery findings
6. model summaries

Rules:

- model summaries never directly overwrite canonical state
- derived artifacts are not canonical sources
- uncertain facts remain `asserted` or `observed`
- arrays append with provenance and deduplication
- `verified` facts outrank `asserted` facts
- conflict resolution should preserve both conflicting claims until resolved

## 10. File Layout

### Repo-Local

```text
.orchestrator/
  config.toml
  session.json
  HANDOFF.md
  RECOVERY.md
  decisions.md
  state-history/
  sessions/
  handoffs/
  crash/
  locks/
  transcripts/
  logs/
  profiles/
```

### Global System

```text
~/.orchestrator/system/
  agents/
  charters/
  routing/
  prompts/
  postmortems/
  scripts/
  usage/
```

## 11. Worktree And Mutation Safety

### Single-Mutator Rule

Only one profile may hold the mutation lock for a worktree.

### Lock Storage

Locks live under:

```text
.orchestrator/locks/
```

Lock metadata should include:

- worktree path
- session id
- profile
- PID when available
- acquired timestamp

### Takeover Policy

Default:

- create a fresh worktree with `git worktree add`

Manual override:

- user may grant `adopt` permission to move a lock to another profile

### Clone And Verify Rule

When a takeover starts in a fresh worktree:

1. new owner reads `RECOVERY.md`
2. new owner inspects diff, logs, and relevant files
3. new owner runs verification commands where applicable
4. uncertain facts must be promoted to `verified` before new mutations

## 12. Workflow Model

### Normal Flow

1. `chief` plans the task
2. orchestrator updates `session.json`
3. orchestrator generates `HANDOFF.md`
4. selected specialist launches
5. work proceeds
6. progress is frozen or completed
7. validated state merges back into canonical form

### Freeze-Dry-Hydrate Switch

1. `Freeze`
   - capture diff, logs, transcript fragments, worktree state, verification signals, and proposed summary
2. `Dry`
   - regenerate `HANDOFF.md` from canonical state
3. `Hydrate`
   - launch the selected profile with the fresh handoff and validated environment

## 13. Recovery Model

Recovery handles:

- model loops
- CLI crashes
- closed terminals
- abandoned sessions
- partial work without clean handoff

Recovery reconstructs from durable artifacts, not lost chat history.

### Recovery Inputs

- `session.json`
- `HANDOFF.md`
- git diff
- changed files
- transcripts and logs
- recent test results
- crash artifacts
- provider-native saved sessions where available

### Recovery Workflow

1. mark session as `stuck`, `crashed`, or `aborted`
2. freeze recoverable evidence
3. assign `recovery`
4. generate `RECOVERY.md`
5. identify confirmed and uncertain facts
6. choose takeover agent if needed
7. merge findings with provenance-aware rules
8. regenerate `HANDOFF.md`
9. resume under takeover
10. write postmortem when significant

### Recovery State Machine

States:

- `planned`
- `running`
- `paused`
- `blocked`
- `stuck`
- `crashed`
- `aborted`
- `recovering`
- `takeover`
- `resumed`
- `completed`

Transitions:

- `planned -> running`
- `running -> paused`
- `paused -> running`
- `running -> blocked`
- `running -> stuck`
- `running -> crashed`
- `blocked -> recovering`
- `stuck -> recovering`
- `crashed -> recovering`
- `recovering -> takeover`
- `takeover -> resumed`
- `resumed -> running`
- `running -> completed`
- `recovering -> aborted`

### `RECOVERY.md` Template

```md
# Recovery Summary

Session ID: <id>
Previous Owner: <agent/profile>
Failure State: <blocked|stuck|crashed>

## Confirmed Facts
- ...

## Uncertain Facts
- ...

## Likely Root Cause
- ...

## Evidence
- changed files:
- recent commands:
- test outputs:
- logs/transcripts:

## Recommended Takeover Agent
- ...

## Next Verification Steps
1. ...
2. ...
3. ...

## Durable Improvements
- ...
```

## 14. Command Contracts

### `launch`

Owner:

- orchestrator/user

Effects:

- run launcher probe
- create/bootstrap isolation directory only if probe allows
- create/select session
- lock worktree if mutation is expected

### `freeze`

Owner:

- orchestrator/user

Effects:

- capture diff, logs, transcript fragments, and proposed model summary
- persist artifacts
- merge validated facts into `session.json`

Rules:

- models may propose content only
- orchestrator performs canonical writes

### `switch`

Owner:

- orchestrator/user

Effects:

- freeze current session
- regenerate `HANDOFF.md`
- handle worktree lock according to clone/adopt policy
- launch selected replacement profile

### `recover`

Owner:

- `recovery` profile under orchestrator control

Effects:

- generate `RECOVERY.md`
- classify facts as confirmed/uncertain
- propose merge updates

Rules:

- `recover` is read-only against canonical state until orchestrator merge

### `takeover`

Owner:

- `chief` + user

Effects:

- select takeover profile
- create fresh worktree or adopt lock with permission
- enforce clone-and-verify
- regenerate `HANDOFF.md`
- move state toward `resumed`

### `postmortem`

Owner:

- `ops` under approval gate

Effects:

- write local incident report
- produce sanitized global lessons where approved
- propose improvements

Storage:

- local details: repo-local `brain/post-mortems/` or `.orchestrator/postmortems/`
- sanitized global lessons: `~/.orchestrator/system/postmortems/`

## 15. Data Classification, Privacy, And Governance

### Classes

- `public`
- `internal`
- `confidential`
- `secret`
- `personal`
- `legal-draft`
- `trading-research`
- `system-internal`

### Defaults

- code and general repo docs stay repo-local
- personal writing stays local or encrypted
- legal drafting defaults to no retention or local encryption
- trading research stays repo-local or encrypted
- secrets never enter handoffs, transcripts, crash artifacts, or postmortems
- global postmortems must be sanitized
- transcripts/logs use retention windows

### Approval Gates

Explicit user or `chief` approval is required before changing global:

- routing rules
- agent charters
- prompt templates
- scripts
- retention policy
- cross-repo memory

## 16. Native Provider Features

Use official provider features where helpful.

For Antigravity CLI, evaluate:

- `.antigravity.md`
- `/chat save`
- `/chat resume`
- `/chat share`
- headless mode

Provider-native state may inform recovery and handoff generation, but must not replace canonical orchestrator state.

## 17. Existing Tools And Positioning

### `aisw`

Focus:

- profile/account switching

Useful for:

- credential/profile isolation ideas

Gap:

- not designed around canonical task state, recovery, or provenance-aware handoffs

### `CCManager`

Focus:

- session and worktree management

Useful for:

- multi-session workflows
- worktree-oriented safety

Gap:

- not centered on project-local provenance-aware state and derived handoff semantics

### This Orchestrator

Focus:

- context integrity
- deterministic recovery
- role-based profile orchestration

Differentiator:

- unifies profile launching, provenance-aware state, generated handoffs, worktree safety, recovery, and governance

## 18. MVP Implementation Order

### Phase 1: Probe Harness

- inspect CLI config/auth/session locations
- validate Antigravity profile isolation on WSL/Linux
- test Windows behavior separately
- test no-auth-bleed assumptions
- define fallback behavior

### Phase 2: State Core

- implement `session.json`
- field metadata
- merge rules
- `HANDOFF.md` generation
- `RECOVERY.md` generation

### Phase 3: Worktree Safety

- lock files
- single-mutator rule
- clone-and-verify takeover
- adopt override

### Phase 4: Command Layer

- `launch`
- `freeze`
- `switch`
- `recover`
- `takeover`
- `postmortem`

### Phase 5: Governance

- data classification
- retention
- approval gates
- global system layer

## 19. Build Recommendation

Use Rust if building the durable version.

Recommended Rust modules:

- `cli`
- `config`
- `probe`
- `profiles`
- `state`
- `handoff`
- `recovery`
- `worktree`
- `locks`
- `governance`
- `providers`

Do not build full automation first.
Build the probe harness first.

## 20. Final Position

This is the canonical implementation and operating spec.

The system should be:

- local
- explicit
- role-based
- provenance-aware
- worktree-safe
- recovery-first
- privacy-conscious
- conservative about provider/account limits

Implementation should begin only after the launcher probe proves the isolation model is safe enough for the target environment.
