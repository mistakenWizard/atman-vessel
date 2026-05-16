# Design State: High-Integrity Minion Workflow

## Brief Summary
- **Problem:** "Dumb" minions creating messy codebases and skipping planning.
- **Primary Persona:** The Creative Director (Human Operator) who requires high-integrity deliverables.
- **Success Metric:** 0 untracked files after task completion and 100% plan-to-code adherence.

## Design Principles
1. **Planning is Mandatory:** No implementation without a brief and a plan.
2. **Isolation is Absolute:** All work happens in isolated worktrees.
3. **Verification is Evidence-Based:** TDD cycles and automated verification before merging.
4. **Hygiene is Automatic:** The system cleans up its own mess.

## Decisions Log
| ID | Decision | Rationale | Status |
|:---|:---|:---|:---|
| D001 | Enforce Discovery | Prevents premature coding and context drift. | Implemented |
| D002 | Auto-Cleanup on Finish | Fixes the "untracked files" and "stale worktrees" friction. | Implemented |
| D003 | Template-Based Handoffs | Solves the "dumb prompt" and "delegation" issues. | Implemented |
| D004 | Audit Layer Logging | Ensures cross-branch visibility via investigations folder. | Implemented |

## Open Questions
- None. (TDD enforcement is currently through documentation and start/finish loop awareness).

## Artefact Index
- Brief: [docs/designpowers/briefs/2026-05-16-high-integrity-workflow.md](docs/designpowers/briefs/2026-05-16-high-integrity-workflow.md)
- Plan: [docs/designpowers/plans/2026-05-16-high-integrity-workflow-plan.md](docs/designpowers/plans/2026-05-16-high-integrity-workflow-plan.md)
- Critique: [docs/designpowers/critiques/2026-05-16-high-integrity-implementation-critique.md](docs/designpowers/critiques/2026-05-16-high-integrity-implementation-critique.md)

## Handoff Chain
- **design-discovery** -> **writing-design-plans**: Initializing the high-integrity workflow design.
- **designpowers-critique** -> **Done**: All high-integrity mandates and core upgrades are live.
