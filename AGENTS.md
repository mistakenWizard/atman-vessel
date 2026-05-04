# AtmanVessel Codex Guide

## Repo Purpose
AtmanVessel is a local AI orchestration framework for deterministic session resumption and context continuity across isolated model profiles.

## Key Paths
- `src-rust/`: Rust orchestrator core
- `src-rust/src/main.rs`: main CLI implementation
- `src-rust/src/tests.rs`: Rust test suite
- `orch.sh`: shell shim that builds and runs the Rust binary
- `.orchestrator/`: runtime state, locks, handoff, recovery, and archives; do not commit runtime artifacts
- `.agent/workflows/`: design and specification documents
- `brain/`: plans and postmortems

## Working Rules
- Do not re-track generated or runtime artifacts under `.orchestrator/`, `src-rust/.orchestrator/`, or `src-rust/target/`
- Preserve user or other-agent changes; do not revert unrelated work
- Expect the repo to be dirty while generated runtime and build artifacts are being removed from Git tracking

## Verified Commands
- Initialize orchestrator state: `./orch.sh init`
- Launch a profiled session: `./orch.sh launch --profile builder --new-worktree`
- WSL verification: `wsl bash -lc 'cd /mnt/e/atman-vessel/src-rust && cargo fmt && cargo test'`

## Environment Notes
- Prefer WSL for Rust verification unless the Windows-native linker path is fixed
- Windows-native `cargo test` can fail if `link.exe` resolves to Git's Unix `link.exe` instead of the MSVC linker

## Additional Notes
- `README_ORCHESTRATOR.md` is the quickest operator-facing command reference
- `./orch.sh verify` promotes only `verified_facts`; it does not auto-verify blockers or next actions
- Current `git status` may show many staged deletions because generated, runtime, and auth-related artifacts are being removed from Git tracking

## Current Focus
- Rust CLI behavior
- state management and provenance
- lock behavior and takeover safety
- recovery flows
- repo hygiene around generated and auth-related files
