# Post-Mortem: Gemini to Antigravity CLI Migration (v2.0)

## 1. Summary
The AtmanVessel project was migrated from Gemini CLI to Antigravity CLI (v2.0) to meet the June 18, 2026 platform deadline. This migration addressed breaking changes in binary naming, configuration paths, and workspace context standards.

## 2. Technical Findings
- **Binary**: `gemini` $\rightarrow$ `agy`.
- **Context File**: `GEMINI.md` $\rightarrow$ `.antigravity.md`.
- **Auth Compatibility**: **Incompatible.** Gemini's `oauth_creds.json` cannot be used with Antigravity due to a Client ID change and Go-based protocol shifts.
- **Environment**: `GEMINI_CLI_HOME` $\rightarrow$ `ANTIGRAVITY_CLI_HOME`.
- **Trust Model**: Deprecated `GEMINI_CLI_TRUST_WORKSPACE` in favor of settings-based permissions managed via `/permissions`.

## 3. Changes Implemented
- **Rust Core**: Updated `src-rust/src/main.rs` to target `agy`, redirect environment variables, and refactor profile bootstrapping to the `antigravity-cli/` subdirectory.
- **Documentation**: Global update of `README.md`, `CLI_GUIDE.md`, `AGENT_GUIDE.md`, `BOOTSTRAP.md`, and `MCP_GUIDE.md`.
- **Skills**: Migrated `gemini-md-improver` to `antigravity-md-improver-skill/antigravity-md-improver`.
- **Source Control**: All changes committed to branch `feat/antigravity-migration`.

## 4. Unresolved Issues / Next Steps
- **Manual Authentication**: All 6+ fleet identities must be manually re-authenticated using `orch launch --profile <name> --interactive agy -- "hello"`.
- **Permission Grants**: Users must grant filesystem permissions via the `agy` CLI for autonomous directory access.
