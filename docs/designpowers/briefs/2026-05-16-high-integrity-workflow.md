# Design Brief: High-Integrity Minion Workflow (v1.0)

## Problem Statement
Minions (AI agents) are currently operating in a "dumb" mode characterized by premature coding, improper delegation, poor worktree hygiene, and a lack of documented planning. This leads to broken branches, cluttered codebases, and significant friction for the human operator who has to repeatedly intervene and clean up.

## Users
- **The Operator (Creative Director):** Needs a clean, predictable, and high-quality delivery from the swarm.
- **The Minions:** Need structured protocols (TDD, Design Discovery) to prevent "hallucinating" implementation paths and to ensure isolation.

## Design Direction
Transform the `orch` core into a **Workflow Orchestrator** that enforces a sequence of **Discovery -> Plan -> Implement (TDD) -> Verify -> Cleanup**. 
- **Discovery First:** No coding until a brief exists.
- **TDD Enforcement:** Implementation requires a test case first.
- **Auto-Cleanup:** `orch task finish` must handle worktree removal and branch merging atomically.
- **Structured Handoffs:** Use pre-defined templates for `orch launch` and `orch task start` to ensure high-signal prompts.

## Constraints
- Must work within the existing Rust `orch` core.
- Must respect the strict 60-second timeout.
- Must maintain cross-repo compatibility (AtmanVessel standard).

## Existing Design System
- AtmanVessel `AGENT_GUIDE.md` and `AGENT_STANDARDS.md`.

## Success Criteria
- 0 untracked files after `orch task finish`.
- 100% of features have a documented design brief and plan before code changes.
- Automated TDD cycles for all logic changes.
- Detectable reduction in "operator cleanup" messages.

## Out of Scope
- Replacing the Rust orchestrator (this is an upgrade, not a rewrite).
- Managing non-AI tasks.
