# Post-Mortem: Phase 1 & 1.5 (Probe Harness & Hardening)

## Status: SUCCESS

### Accomplishments
- **Physical Assumption Validation**: Proved that `HOME` redirection is sufficient for Antigravity profile isolation.
- **Reproducibility**: Created a reusable probe harness in `scripts/probes/` and `scripts/probe.sh`.
- **Platform Matrix**: Reconciled Windows/WSL environment variable detection.
- **Bootstrapping**: Identified and implemented full auth file bootstrapping for Antigravity in `orch.sh`.
- **MVP Scaffolding**: Initialized `src-rust` with robust JSON models for `Session` and `Profile`.

### Lessons Learned
- Antigravity CLI authentication is sensitive to a specific set of 5 files; missing any one (like `oauth_creds.json`) breaks the isolated profile's ability to reach the model.
- Simulation of second identity is sufficient for filesystem and logic validation, but real multi-account testing remains a future requirement for production-grade assurance.
- `sed` is dangerous for structured JSON state and must be replaced by the Rust implementation as soon as possible.

### Decision
Proceeding to Phase 2: MVP Implementation.
