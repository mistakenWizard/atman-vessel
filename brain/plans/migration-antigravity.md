# Migration Plan: Antigravity CLI to Antigravity CLI (v2.0)

## Overview
Google is transitioning Antigravity CLI to the Go-based **Antigravity CLI** on the Antigravity 2.0 platform. This migration introduces breaking changes to the configuration structure, extension system, and workspace standards.

**Deadline:** June 18, 2026 (Consumer services for Pro/Ultra/Free end).

## 1. Core Platform Changes
| Feature | Gemini CLI (Old) | Antigravity CLI (New) |
| :--- | :--- | :--- |
| **Binary** | `gemini` | `agy` (Antigravity CLI) |
| **Global Config** | `~/.gemini/` | `~/.gemini/antigravity-cli/` |
| **Workspace Context** | `GEMINI.md` | `.antigravity.md` |
| **Skills Directory** | `.gemini/skills/` | `.agents/skills/` |
| **MCP Config** | `settings.json` | `mcp_config.json` |
| **Extensions** | Extensions | Plugins |
| **Trust Mode** | `GEMINI_CLI_TRUST_WORKSPACE` | Managed via `settings.json` / `/permissions` |

## 2. Authentication Compatibility (CRITICAL)
**Finding:** Gemini CLI authentication JSON files (`oauth_creds.json`, `google_accounts.json`) are **NOT** compatible with Antigravity CLI.

**Technical Reasons:**
- **OAuth Client ID Change:** Antigravity CLI uses a new Client ID (`1071006060591-...`). Tokens issued for Gemini CLI are rejected by the Antigravity backend.
- **Protocol Shift:** The transition from Node.js to Go introduced changes in how tokens are cached and encrypted.
- **Config Path:** Antigravity requires credentials to be located in the `antigravity-cli/` subdirectory.

**Action Required:** Every profile must be re-authenticated manually using the `--interactive` flag.

## 2. Orchestrator Implementation (Rust)
### 2.1. CLI Invocation
- Update `prepare_launch_command` in `src-rust/src/main.rs` to use `agy`.
- Set `ANTIGRAVITY_CLI_HOME` to the profile path.
- Add support for the new permission model.

### 2.2. Profile Bootstrapping
- Update `bootstrap_profile` to include `mcp_config.json`.
- Adjust file copy logic for the new `antigravity-cli` subdirectory.

### 2.3. Plugin Migration
- For existing profiles, run `agy plugin import agy` to migrate legacy extensions to the new plugin architecture.

## 3. Workspace Migration
### 3.1. Context Renaming
- Rename root `.antigravity.md` to `.antigravity.md`.
- Rename any subdirectory `.antigravity.md` files.

### 3.2. Skill Migration
- Move `.agy/skills/` to `.agents/skills/`.
- Update `dist-skills/` if they are used as workspace-level skills.

## 4. Documentation Audit
- Update `README.md`, `CLI_GUIDE.md`, `AGENT_GUIDE.md`, and `BOOTSTRAP.md`.
- Replace all "Antigravity CLI" occurrences with "Antigravity CLI".
- Replace `orch launch ... agy` with `orch launch ... agy`.

## 5. Execution Strategy
1. **Phase 1: Dual-Support (Optional)**: Update Rust core to support both `agy` and `agy`.
2. **Phase 2: Transition**: Rename context files and move skills.
3. **Phase 3: Cleanup**: Remove `agy` references and strictly use `agy`.
