# Phase 4 Design: Integration & Finalization

## 1. Objectives
- Port the legacy bash logic from `orch.sh` into the Rust core.
- Ensure 100% reliable identity isolation via `HOME` redirection.
- Establish `orch` as the primary, provenance-aware entry point for AI development.

## 2. Technical Design

### 2.1 Profile Bootstrapping
- **Function**: `bootstrap_profile(profile_path: &Path) -> Result<()>`
- **Logic**:
    1. Identify base Gemini config (e.g., `~/.gemini`).
    2. Check if target profile directory exists; create if not.
    3. Copy the five mandatory auth files identified in Phase 1.5:
        - `google_accounts.json`
        - `oauth_creds.json`
        - `settings.json`
        - `projects.json`
        - `state.json`
    4. **Safety**: Do not overwrite existing files in the target profile unless a reset is requested.

### 2.2 CLI Invocation (The Vessel)
- **Logic**: Use `std::process::Command` to spawn the child process.
- **Environment**:
    - Set `HOME` to the profile-specific directory.
    - Inherit all other environment variables.
- **Interaction**: Ensure `stdin`, `stdout`, and `stderr` are inherited so interactive CLI features work seamlessly.
- **State Integration**: Call `Session::save()` and update `HANDOFF.md` *before* spawning to ensure context integrity if the child process crashes.

### 2.3 orch.sh Replacement
- **Strategy**: Rename the Rust binary to `orch`.
- **Shim**: Provide a thin `orch.sh` that just calls `cargo run --` (for development) or points to the installed binary.

## 3. TDD Requirements (Test Cases)

### Test Case 1: Auth Bootstrapping Integrity
- Given an empty profile directory.
- When `bootstrap_profile` is called.
- Then the five mandatory files must exist in the target directory and match the source contents.

### Test Case 2: Environment Redirection (Dry Run)
- Given a profile root at `/path/to/profile`.
- When `orch launch` is executed (mocking the actual spawn).
- Then the resulting `Command` object must have its `HOME` environment variable set to `/path/to/profile`.

### Test Case 3: State Transition Safety
- Given a session in `Planned` state.
- When `orch launch` is called.
- Then the `session.json` on disk must be in `Running` state *before* the child process execution starts.

## 4. Implementation Strategy
- **Agent 1 (Tester)**: Writes unit tests for bootstrapping and command preparation.
- **Agent 2 (Builder)**: Implements the logic in `main.rs` to satisfy the integration requirements.
