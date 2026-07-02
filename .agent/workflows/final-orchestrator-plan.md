# Final Orchestrator Plan

## Objective

Build a local orchestrator for development workflows that:

- uses `Codex CLI` as the main planner
- can work with multiple pre-authenticated `Antigravity CLI` profiles
- preserves context automatically across profile switches
- avoids repeated login friction
- works across Windows, Linux, macOS, and WSL

This tool is for `session continuity`, not for building an automatic quota balancer.

## Core Philosophy

The orchestrator is a `Deterministic Session Resumption` system.

It treats AI models as disposable specialized agents while keeping task continuity in project-local state.

Three concepts define the model:

- `Soul`: durable project-local context and canonical state
- `Voice`: generated handoff artifacts used to rehydrate the next session
- `Vessel`: the execution environment and profile that runs a given role

The orchestrator owns the `Soul`.
The `Voice` is derived from it.
The `Vessel` is replaceable.

## User-Accepted Boundary

The workflow should support:

- logging into all Antigravity profiles up front so there is no repeated auth headache
- manually telling the project or Codex to switch to the next Antigravity profile
- automatic carry-forward of context so the user does not need to copy context from one agent into another manually

The workflow should avoid:

- the user manually copy-pasting state between agents
- hidden or silent account switching
- automatic round-robin across equivalent consumer accounts
- randomized selection across equivalent consumer accounts
- treating many consumer profiles as one pooled backend

## Practical Compliance Position

This plan is designed to stay on the conservative side.

Important distinction:

- safer: explicit user-triggered switching with project-local state continuity
- riskier: automatic failover or proactive load balancing across equivalent consumer accounts to avoid limits

For OpenAI, the Terms of Use dated January 1, 2026 explicitly prohibit circumventing rate limits or restrictions. Since you are only using one Codex account, that part is straightforward.

For Google/Antigravity, I did not find an official Google statement that explicitly says your exact workflow is allowed or forbidden. The closest official signals are:

- Antigravity CLI terms vary based on authentication method
- Antigravity CLI officially supports `.antigravity.md`
- Google documents quotas as real usage controls
- Google prohibits interference with or circumvention of abuse protections

That means the safest interpretation is:

- pre-authenticated profiles may be acceptable as a convenience setup
- explicit human-triggered switching is more defensible than automatic switching
- automatic round-robin or failover across equivalent consumer accounts is the part most likely to create policy risk

This is a practical policy interpretation, not legal advice.

## Core Design

The system has five parts.

### 1. Canonical State

The project owns a machine-readable source of truth:

```text
.orchestrator/session.json
```

This stores:

- current goal
- constraints
- verified changes
- blockers
- files in focus
- decisions
- next actions
- active provider/profile

This file is owned by the orchestrator, not directly by any model.

This is the `Soul`.

### 2. Handoff Summary

The orchestrator generates a model-readable summary:

```text
.orchestrator/HANDOFF.md
```

This is what every new provider/profile reads at startup.

Its purpose:

- restore task context
- avoid manual copy-paste
- keep all providers aligned on the same work state

This is the `Voice`.

### 3. Profile Registry

The orchestrator tracks named profiles such as:

- `codex-main`
- `agy-a`
- `agy-b`
- `agy-c`

Each profile records:

- provider
- launch command
- config root or profile root
- execution target such as native Windows or WSL
- optional environment variables

This is the `Vessel` layer.

### 4. Worktree Ownership

Each active implementation or recovery session should own a distinct branch or worktree when code can change.

Purpose:

- prevent takeover from trampling live edits
- keep diffs attributable to a single session
- make recovery evidence easier to inspect

Recommended rules:

- one mutating session owns one worktree
- takeover creates or adopts an explicit worktree owner
- lock the worktree while a session is `running`, `recovering`, or `takeover`
- review-only or docs-only sessions may use lighter locking if they are read-mostly

### 5. State Provenance

`session.json` is canonical, but every field update should carry provenance.

State should record:

- source type such as `filesystem`, `git`, `test-output`, `model-summary`, `user`, or `recovery`
- timestamp
- originating session id
- confidence level when derived

Derived artifacts such as `HANDOFF.md` and `RECOVERY.md` should never overwrite canonical state without explicit merge rules.

## Required Workflow

### Normal Flow

1. Start in `Codex CLI`.
2. Codex plans the task.
3. The orchestrator updates `session.json`.
4. The orchestrator generates `HANDOFF.md`.
5. A Antigravity profile is launched with the handoff.
6. Antigravity works.
7. When paused or blocked, the orchestrator captures a summary and updates `session.json`.

### Switch Flow

1. The user tells the project or Codex to switch to another Antigravity profile.
2. The orchestrator freezes the current session.
3. The orchestrator updates `session.json`.
4. The orchestrator regenerates `HANDOFF.md`.
5. The orchestrator launches the selected Antigravity profile.
6. The new profile resumes from the generated handoff automatically.

The user should not need to manually copy context between agents.

### Freeze-Dry-Hydrate Switch

This is the preferred mental model for switching.

1. `Freeze`
   - capture proposed summary, git diff, current worktree state, logs, and verification signals
   - merge validated facts into `session.json`
2. `Dry`
   - regenerate `HANDOFF.md` from canonical state
3. `Hydrate`
   - launch the selected profile with the fresh handoff and the appropriate environment

This makes switching operationally simple without turning it into hidden failover.

## Automation Rules

### Allowed Automation

- generate and refresh `HANDOFF.md`
- update `session.json`
- launch a selected provider/profile
- capture a proposed summary from the current session
- rehydrate a new session from local project state
- use provider-native session features as adapter inputs where officially supported

### User-Triggered Only

- switching to the next Antigravity profile when limits or session issues occur
- selecting another provider/profile because of availability concerns

### Not Allowed In This Design

- automatic failover from one Antigravity consumer account to another
- proactive round-robin across equivalent Antigravity accounts
- randomized routing across equivalent Antigravity accounts
- background load balancing across consumer identities
- any design whose purpose is to avoid per-profile restrictions by spreading load

## Why This Meets Your Actual Need

Your real problem is not that you cannot ever switch.
Your real problem is:

- logging in again is annoying
- context gets lost during switching
- manual handoff is tedious

This design solves all three:

- all profiles can be authenticated ahead of time
- context is always stored locally
- switching carries context automatically
- the only manual action left is saying "switch to Antigravity-B"

## Role-Based Profile Model

The preferred model is `role-based specialization`.

Each Antigravity profile should have:

- a primary role
- optional secondary capabilities
- a real reason to exist beyond quota distribution
- a stable execution environment when relevant

Good examples:

- `Codex` for planning and architecture
- `Antigravity-WSL` for Linux/WSL shell tasks, build issues, environment-sensitive debugging
- `Antigravity-Win` for Windows-native tasks, PowerShell, filesystem/path quirks, Windows tooling
- `Antigravity-Test` for running tests, triaging failures, summarizing regressions
- `Antigravity-Review` for patch review, edge cases, and risk analysis
- `Antigravity-Ops` for maintaining agent docs, scripts, aliases, setup, and local workflow support
- future `Claude` profile for writing, refactoring review, or synthesis

Minimal MVP profile set:

- `chief` for planning, dispatching, and integration
- `builder` for implementation and refactoring
- `systems` for environment, tooling, and WSL/Windows setup
- `reviewer` for code review, regression spotting, and verification
- `writer` for docs, summaries, and drafting
- `quant` for trading research and backtesting support
- `ops` for improving the agent system itself
- `recovery` for stuck/crashed-session triage

This model is stronger if:

- roles are stable
- routing is based on task type
- the system still works without spreading load evenly

Profiles do not need to be hard-locked to one role forever.
The better model is:

- each profile has a primary role
- each profile may have secondary capabilities
- routing prefers specialization first
- the user may override manually

Example:

- `agy-wsl`: primary `shell/debug`, secondary `implementation`
- `agy-win`: primary `windows/tooling`, secondary `implementation`
- `agy-test`: primary `tests`, secondary `bug triage`
- `agy-review`: primary `review`, secondary `tests`
- `agy-ops`: primary `docs/scripts/setup`, secondary `maintenance`

This keeps the profiles useful without turning them into interchangeable worker slots.

## Why Round-Robin Is Still Risky

Round-robin sounds harmless if framed as "let each agent do some work."

But if the profiles are equivalent consumer identities, the technical effect is:

- spread usage across profiles
- reduce the chance any one profile hits a limit
- increase the total uninterrupted usage available to the workflow

That is too close to quota-avoidance behavior.

Randomized selection does not fix this.
If the accounts are equivalent and the point is to spread usage, random routing is still a load-balancing strategy.

So the design should avoid:

- round-robin across equivalent Antigravity consumer profiles
- randomized selection across equivalent Antigravity consumer profiles
- any scheduler that optimizes for spreading usage across identical consumer identities

## Existing Tools Worth Looking At

These tools may reduce how much you need to build from scratch.

### `aisw`

Why it is relevant:

- switching and profile convenience
- useful reference for credential/profile isolation workflows

Why it is not enough on its own:

- it does not solve project-local task continuity or recovery ownership

### CCManager

GitHub: `https://github.com/kbwo/ccmanager`

Why it is relevant:

- manages multiple AI coding assistant sessions
- supports Codex CLI, Antigravity CLI, Claude Code, and others
- built around multi-session workflows and worktrees

Why it matters more than a simple switcher:

- worktree/session management is directly relevant to takeover and recovery safety

### CodeAgentSwarm

Website: `https://www.codeagentswarm.com/`

Why it is relevant:

- multi-terminal workspace for coding agents
- focused on visibility and coordination

### CC-Switch CLI

GitHub: `https://github.com/SaladDay/cc-switch-cli`

Why it is relevant:

- cross-platform switching and configuration management for AI coding tools

### Atmos AI CLI Providers

Website: `https://atmos.tools/changelog/ai-cli-providers`

Why it is relevant:

- useful architectural reference for treating installed CLIs as interchangeable providers

None of these exactly matches the conservative design here, but `CCManager` is the closest tool to evaluate before building your own.

## Tool Comparison Summary

Use this shorthand when comparing the plan to existing tools.

| Tool | Core Focus | Main Overlap | Main Gap |
| :--- | :--- | :--- | :--- |
| `aisw` | Switching and profile convenience | profile and credential isolation ideas | little or no task-state continuity |
| `CCManager` | session and worktree management | multi-session handling, worktree safety, takeover-adjacent workflow | not centered on canonical project-local state and handoff semantics |
| `final-orchestrator-plan` | deterministic continuity | integrates state, handoff, switching, recovery, and role-based routing | depends on proving provider-isolation assumptions before implementation |

## Recommended File Layout

```text
.orchestrator/
  config.toml
  session.json
  state-history/
  locks/
  HANDOFF.md
  decisions.md
  handoffs/
  transcripts/
  logs/
```

## Suggested `session.json`

```json
{
  "schema_version": 1,
  "session_id": "",
  "goal": "",
  "constraints": [],
  "verified_changes": [],
  "open_blockers": [],
  "files_in_focus": [],
  "decisions": [],
  "next_actions": [],
  "active_provider": "",
  "active_profile": "",
  "active_worktree": "",
  "last_updated": "",
  "provenance": {}
}
```

Suggested provenance shape:

```json
{
  "goal": { "source": "user", "session_id": "s1", "confidence": 1.0 },
  "verified_changes": { "source": "git", "session_id": "s4", "confidence": 0.9 },
  "open_blockers": { "source": "model-summary", "session_id": "s4", "confidence": 0.5 }
}
```

## v1 Scope

Build only this first:

- `orch init`
- `orch status`
- `orch profiles list`
- `orch launch <provider> --profile <name>`
- `orch freeze`
- `orch handoff generate`
- `orch resume --profile <name>`
- `orch switch --profile <name>`

The `switch` command should:

- freeze the current session
- refresh state
- regenerate handoff
- launch the selected profile
- preserve or transfer explicit worktree ownership

That is the key convenience feature you actually want.

## Validation Before Full Build

Before committing to the full orchestrator, verify:

1. whether Antigravity truly supports stable per-profile isolation in the way you intend
2. whether Codex supports the same cleanly
3. whether Windows native and WSL behave consistently
4. where each CLI actually stores auth and config
5. whether multiple pre-authenticated Antigravity profiles can coexist without conflict

If those checks fail, change the launcher model before building more.

Add a proof harness before full implementation:

- inspect where each CLI actually stores config, auth, and chat/session state
- launch isolated probes on Windows and WSL
- verify whether two pre-authenticated Antigravity profiles can coexist without corrupting each other
- verify whether provider-native resume/checkpoint features can be used instead of custom hacks
- fail closed if the launcher contract is not stable

## Final Recommendation

Build a local orchestrator that:

- pre-authenticates multiple Antigravity profiles
- keeps all context in project-local files
- lets Codex remain the main planner
- allows the user to say "switch to the next Antigravity profile"
- automatically carries forward context without manual copy-paste

Do not build:

- automatic failover
- round-robin across equivalent Antigravity consumer accounts
- randomized routing across equivalent Antigravity consumer accounts
- proactive load balancing across consumer identities

That gives you the workflow relief you actually care about while staying materially safer than a multi-account load balancer.

## Agent Charter Architecture

If the system spans coding, writing, research, trading, and self-improvement, the right model is a small set of durable agent charters rather than a pool of equivalent Antigravity profiles.

### Recommended Agent Set

#### `codex-chief`

Primary mission:

- planner
- dispatcher
- integrator

Responsibilities:

- break work into subtasks
- assign the right specialist
- maintain handoff quality
- keep task state coherent

#### `agy-builder`

Primary mission:

- implementation across languages and platforms

Secondary capabilities:

- refactors
- debugging
- prototyping

#### `agy-systems`

Primary mission:

- environment and tooling work

Secondary capabilities:

- Windows and WSL setup
- shell scripts
- aliases
- local automation
- CI and developer tooling support

#### `agy-reviewer`

Primary mission:

- code review
- edge-case analysis
- regression spotting

Secondary capabilities:

- architecture critique
- test-gap analysis
- patch review

#### `agy-writer`

Primary mission:

- docs
- emails
- summaries
- personal and legal drafting support

Secondary capabilities:

- repo documentation
- onboarding docs
- structured notes

Important boundary:

- useful for drafting and organizing legal material
- not a substitute for legal advice

#### `agy-quant`

Primary mission:

- trading idea exploration
- market research
- backtesting support

Secondary capabilities:

- experiment design
- result summaries
- data preparation

Important boundary:

- useful for research and analysis
- should not be treated as autonomous trading authority

#### `agy-ops`

Primary mission:

- improve the agent system itself over time

Secondary capabilities:

- maintain agent docs
- improve prompt templates
- improve handoff structure
- maintain helper scripts and aliases
- monitor token and usage patterns
- improve cross-repo workflows

#### `agy-recovery`

Primary mission:

- recover stalled, crashed, or abandoned agent sessions

Secondary capabilities:

- diagnose why an agent got stuck
- reconstruct likely task state
- prepare takeover handoffs
- produce system improvements so the same failure is less likely again

### Design Rule

Each agent should have:

- a primary role
- secondary capabilities
- clear do-not-use cases

Profiles should be broad enough to stay useful, but not so interchangeable that they become a disguised load-balancing pool.

## Cross-Repo System Layer

The system should maintain two layers of state.

### Repo-Local State

For task continuity inside a specific repository:

```text
.orchestrator/
  session.json
  HANDOFF.md
  decisions.md
```

### Global System State

For improving the entire agent ecosystem across repositories and task types:

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

This global layer is where `agy-ops` should work.

Sensitive domains should not be copied into the global system layer by default.

## Recovery Mode

Recovery mode is required.

Problem:

- a Antigravity session may crash
- a terminal may be closed
- the model may get stuck
- partial progress may exist without a clean handoff

The orchestrator needs a recovery path that gets the system back on track without relying on lost chat history.

There should also be a dedicated recovery specialist:

- `agy-recovery` for failure analysis and takeover support
- `codex-chief` remains the final coordinator when recovery changes task direction

### Recovery Inputs

Recovery should reconstruct task state from:

- the last saved `session.json`
- the current `HANDOFF.md`
- recent file modifications
- git diff
- recent test results if available
- any captured transcript fragments or logs

### Recovery Workflow

1. Detect or declare that the current worker session is lost or unreliable.
2. Freeze the broken session as `aborted` or `stale`.
3. Inspect repo state:
   - changed files
   - git diff
   - uncommitted work
   - recent command/test outputs if available
4. Generate a `RECOVERY.md` summary containing:
   - likely completed work
   - uncertain work
   - blockers
   - next verification steps
5. Assign `agy-recovery` to diagnose the failure mode.
6. If needed, assign a takeover specialist such as `agy-builder`, `agy-systems`, or `agy-reviewer`.
7. Regenerate `session.json` and `HANDOFF.md` from the recovered picture.
8. Resume work in a new session.
9. Record a postmortem so the system improves over time.

### Takeover Flow

The orchestrator should support explicit takeover when one agent cannot continue.

Examples:

- `agy-builder` gets stuck, so `agy-reviewer` or `agy-systems` takes over
- `agy-test` crashes, so `agy-builder` continues after reviewing failures
- `agy-writer` stalls, so `codex-chief` or another writing-capable profile resumes

Takeover should follow this pattern:

1. mark the original session as `stuck`, `crashed`, or `aborted`
2. collect all recoverable evidence
3. generate `RECOVERY.md`
4. choose the best takeover agent
5. generate a new `HANDOFF.md`
6. resume under the new owner

This avoids losing work just because a specific agent failed.

### Failure Analysis

Recovery should not only restore work. It should also explain failure.

`agy-recovery` should produce a structured diagnosis:

- what was the agent trying to do
- where it got stuck
- whether the problem was prompt quality, missing context, environment issues, tool failure, task ambiguity, or model weakness
- what evidence supports the conclusion
- what should change next time

Suggested failure categories:

- missing or stale handoff
- ambiguous task definition
- insufficient repo context
- environment mismatch
- tool or shell failure
- model loop or reasoning stall
- verification gap
- role mismatch

### Recovery Postmortem

Every significant stuck/crash event should optionally write a postmortem:

```text
~/.orchestrator/system/postmortems/
```

Suggested contents:

- incident summary
- affected agent
- task type
- root cause
- recovery steps used
- what worked
- what failed
- durable improvements to make

### System Improvement Loop

Recovery should feed a long-term improvement loop.

`agy-recovery` and `agy-ops` should work together:

- `agy-recovery` explains the immediate failure
- `agy-ops` converts that into durable system improvements

Examples of improvements:

- better handoff templates
- stronger role definitions
- safer routing rules
- improved helper scripts
- clearer repo bootstrapping
- better verification checkpoints
- token/log capture improvements

This makes the system more resilient over time instead of repeatedly failing in the same way.

Global changes proposed by `agy-recovery` or `agy-ops` should require approval by `codex-chief` or the user before they change:

- routing rules
- agent charters
- prompts
- global scripts
- retention policy

### Recovery State Machine

Recovery should be modeled explicitly.

Recommended session states:

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

Recommended transitions:

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

State meanings:

- `blocked`: the agent is responsive but cannot continue because of a known obstacle
- `stuck`: the agent is alive but looping, confused, or not making progress
- `crashed`: the session is unavailable or untrustworthy
- `recovering`: the system is reconstructing state and should avoid normal task progress
- `takeover`: a replacement agent is validating recovery artifacts before continuing

### Recovery File Ownership

Artifact ownership should be explicit.

`session.json`

- owner: orchestrator
- role: canonical machine-readable task state

`HANDOFF.md`

- owner: orchestrator
- role: generated task summary for active or next worker

`RECOVERY.md`

- owner: orchestrator
- content proposed by: `agy-recovery`
- role: recovery reconstruction and takeover guidance

`postmortem.md`

- owner: orchestrator or `agy-ops`
- content proposed by: `agy-recovery`
- role: durable learning after incidents

Only approved conclusions should flow from postmortems into global system policy.

### Crash Artifact Layout

Recommended layout:

```text
.orchestrator/crash/<session-id>/
  metadata.json
  summary.txt
  commands.log
  tests.log
  diff.patch
  notes.md
```

This keeps failed-session evidence isolated and inspectable.

Sensitive crash artifacts should support redaction or encryption when they involve:

- legal drafts
- personal writing
- trading notes
- credentials or environment secrets

### Recovery Commands

Add these commands to the design:

- `orch recover`
- `orch recover --from-crash`
- `orch recover --profile <name>`
- `orch takeover --profile <name>`
- `orch postmortem`

Expected behavior:

- inspect repo state
- synthesize the likely current task position
- produce `RECOVERY.md`
- update canonical state
- offer a clean resume point
- optionally trigger root-cause analysis and postmortem generation

Command intent:

`orch freeze`

- pause the active session cleanly
- capture a proposed summary
- update `session.json`

`orch switch --profile <name>`

- freeze the current session
- refresh `session.json`
- regenerate `HANDOFF.md`
- launch the selected replacement profile

`orch recover`

- mark the session as `recovering`
- inspect git state, changed files, logs, transcripts, and test outputs
- generate `RECOVERY.md`
- merge findings into canonical state using provenance-aware rules

`orch recover --from-crash`

- snapshot crash evidence
- reconstruct state from filesystem and process artifacts
- assign `agy-recovery` for diagnosis
- prefer verified filesystem and git facts over narrative summaries

`orch takeover --profile <name>`

- select the new owner
- mark the session as `takeover`
- generate a fresh `HANDOFF.md`
- launch the replacement profile
- move state toward `resumed`

`orch postmortem`

- record root cause
- store durable lessons
- create follow-up work for `agy-ops`
- require approval before modifying global policy or automation

### Recovery Artifacts

Recommended files:

```text
.orchestrator/
  RECOVERY.md
  crash/
  transcripts/
  logs/
```

### Recovery Principles

- assume chat history may be gone
- trust filesystem and git state more than memory
- clearly mark uncertain conclusions
- prefer verification steps over speculative summaries

This is especially important for:

- coding tasks with partial edits
- test/debug sessions
- trading research with unfinished analysis
- docs where partial drafts may be mistaken for final output

### Takeover Rules

When another agent takes over:

- the new owner must read `RECOVERY.md` before `HANDOFF.md`
- the new owner must verify uncertain facts before continuing substantial work
- the old agent's unfinished reasoning is not treated as ground truth
- filesystem state, git state, and verification output outrank narrative summaries

### Agent Update Permissions During Recovery

`codex-chief`

- may approve takeover direction
- may update planning and routing artifacts
- should not directly overwrite canonical recovery evidence without reason

`agy-recovery`

- may update `RECOVERY.md`
- may classify failure cause
- may propose postmortem content
- should not silently resume unrelated implementation work

`agy-ops`

- may update global charters, prompts, scripts, and workflow improvements
- may convert incidents into durable system changes

takeover specialist such as `agy-builder`, `agy-systems`, or `agy-reviewer`

- may resume task work after recovery artifacts are accepted
- should not rewrite recovery history

## Data Classification And Retention

Not all work should be retained the same way.

Recommended classes:

- `repo-public`
- `repo-private`
- `personal`
- `legal-draft`
- `trading-research`
- `system-internal`

Recommended defaults:

- repo-local state stays with the repo
- personal, legal, and trading material remain repo-local or encrypted
- global postmortems should redact sensitive content by default
- transcripts should have retention windows
- secrets should never be stored in handoffs, transcripts, or crash artifacts

The orchestrator should allow per-task retention settings.

## Native Provider Features

Use official provider features when they reduce custom complexity.

For Antigravity CLI, the adapter should evaluate official features such as:

- `.antigravity.md`
- `/chat save`
- `/chat resume`
- `/chat share`
- headless mode

These should be treated as inputs to the orchestrator, not as replacements for canonical project state.

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
