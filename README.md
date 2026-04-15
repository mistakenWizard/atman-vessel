# AtmanVessel: The Deterministic AI Orchestrator

## Mission
AtmanVessel is a local AI orchestration framework designed for **Deterministic Session Resumption**. It treats AI models as disposable specialized agents (**Vessels**) while maintaining a persistent, project-local "Soul" (**Context**).

## Core Concepts
- **Soul**: Durable, provenance-aware canonical state (`session.json`).
- **Voice**: Ephemeral rehydration artifacts (`HANDOFF.md`, `RECOVERY.md`).
- **Vessel**: Isolated execution environments (Profiles) for specific roles.

## Project Structure
- `.agent/workflows/`: Strategic and operational plans.
- `.orchestrator/`: Canonical state, locks, and generated artifacts.
- `brain/`: Planning, post-mortems, and long-term memory.
- `orch.sh`: The experimental launcher and probe harness.

## Standards
- **Manual Compliance**: User-triggered switching only.
- **Isolation First**: No auth-bleed between profiles.
- **Provenance Aware**: Every state field must have a source and timestamp.
- **Verification Driven**: Clone-and-verify takeover policy.

## Getting Started
See `.agent/workflows/orchestrator-master-spec.md` for the full technical specification.
