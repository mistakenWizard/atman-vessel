# Phase 1 Directive: Probe Harness

## Objective

Validate the physical assumptions required by the orchestrator before building the full system.

The probe harness must answer:

- whether Gemini profile isolation is stable enough to use
- whether Codex/Gemini config and auth roots can be identified safely
- whether Windows and WSL behave differently
- whether lock recovery can detect dead sessions
- whether multi-terminal sessions can detect stale context
- whether `session.json` version checks can prevent unsafe writes

The goal is not to build orchestration yet.
The goal is to prove or reject the environment assumptions.

## Scope

Phase 1 should implement probes only.

Do not implement:

- full switching
- automatic recovery
- full state merge logic
- role routing
- global governance

## Probe Areas

### 1. Provider Profile Isolation

Validate for each provider/profile:

- config path
- auth path
- session/cache path
- environment variables that affect those paths
- whether `HOME` redirection works
- whether profile state leaks across profiles

Required Gemini checks:

- launch profile A
- authenticate normally if needed
- launch profile B with isolated root
- verify profile B does not see profile A identity
- verify profile A still works after profile B runs

Required output:

```text
.orchestrator/probe/provider-isolation.json
```

### 2. Windows vs WSL Behavior

Validate separately:

- Windows native PowerShell
- WSL/Debian shell

Check:

- path handling
- environment inheritance
- process lifetime visibility
- CLI config/auth behavior
- whether the same strategy works on both systems

Required output:

```text
.orchestrator/probe/platform-matrix.json
```

### 3. Lock Liveness And Stale Lock Recovery

Validate PID-sensitive locks.

Probe behavior:

1. create a lock with current PID
2. verify PID is live
3. create a fake/dead PID lock
4. verify the harness detects it as stale
5. verify stale lock would transition the session to `crashed`
6. verify recovery would be recommended rather than silently deleting the lock

Required output:

```text
.orchestrator/probe/lock-liveness.json
```

Required rule:

- live lock blocks mutation
- dead lock triggers `crashed -> recovering`
- dead lock should not be silently ignored

### 4. Schema Version Safety

Validate `session.json` version handling.

Probe behavior:

1. read current schema version
2. reject unknown future schema versions for write operations
3. allow read-only inspection of unknown future schema versions
4. support explicit migration command later

Required output:

```text
.orchestrator/probe/schema-version.json
```

Required rule:

- never write to an unknown or newer schema version
- fail safe into read-only mode

### 5. Multi-Terminal Snapshot/Reload Detection

Validate stale context detection.

Probe behavior:

1. session A starts with a known `session.json` timestamp
2. session B updates `session.json`
3. session A checks timestamp before continuing
4. session A detects newer state and requires reload

Required output:

```text
.orchestrator/probe/context-sync.json
```

Required rule:

- every command checks whether `session.json.last_updated` is newer than the active session snapshot
- if newer, command must warn:

```text
Context update detected. Re-read .orchestrator/HANDOFF.md before continuing.
```

## Minimum Files

Phase 1 may create:

```text
.orchestrator/
  probe/
    provider-isolation.json
    platform-matrix.json
    lock-liveness.json
    schema-version.json
    context-sync.json
    summary.md
```

No auth secrets should be written into probe outputs.

## Success Criteria

Phase 1 is successful if:

- provider paths are identified without leaking secrets
- Gemini profile isolation is proven or rejected
- Windows vs WSL differences are documented
- stale lock behavior is defined from real process behavior
- schema write-safety is validated
- stale context detection is validated
- probe results clearly say whether full MVP implementation may proceed

## Failure Criteria

Phase 1 should block full implementation if:

- profile isolation leaks identity across Gemini profiles
- CLI auth files cannot be isolated safely
- HOME redirection corrupts provider state
- stale locks cannot be detected reliably
- schema writes cannot be guarded
- multi-terminal stale state cannot be detected

## Deliverable

The final Phase 1 deliverable is:

```text
.orchestrator/probe/summary.md
```

It must include:

- pass/fail status for each probe
- environment tested
- risks discovered
- fallback recommendation
- whether Phase 2 may begin

## Implementation Recommendation

Start with a small standalone probe tool or script before building the Rust orchestrator.

Preferred path:

1. implement probes as simple scripts
2. run them on Windows and WSL
3. record results
4. only then start Rust MVP scaffolding

This avoids baking unproven environment behavior into the core architecture.
