# Post-Mortem: MCP Core Deep Review

## 1. Task Overview
- **Task Slug**: `mcp-core-deep-review`
- **Profile**: `reviewer`
- **Status**: Success
- **Date**: 2026-05-05

## 2. Findings & Actions
The deep review of the Model Context Protocol (MCP) implementation identified several critical gaps in security, compliance, and consistency.

### 2.1 Security & Validation
- **Path Traversal Risk**: The `slug` parameter was used directly in filesystem paths without validation.
- **Action**: Implemented `validate_slug` using a strict alphanumeric/hyphen/underscore allowlist.
- **Probe Bypass**: MCP `start_task` calls were bypassing the mandatory pre-flight safety probes defined in Phase 7.
- **Action**: Refactored shared logic into `start_task_core`, ensuring all launches (CLI and MCP) run `run_probes()`.

### 2.2 JSON-RPC Compliance
- **Parse Errors**: Malformed JSON lines were silently ignored.
- **Action**: Updated `run_mcp_server` to return standard JSON-RPC `-32700` (Parse error) responses.
- **Notifications**: Notifications (requests without an `id`) were incorrectly receiving responses.
- **Action**: Updated the loop to skip response generation for notification requests.

### 2.3 Feature Completeness
- **Missing Tool**: The `freeze_session` tool was advertised in `tools/list` but lacked a handler in `tools/call`.
- **Action**: Implemented the `freeze_session` tool handler with proper provenance and session state management.
- **Handoff Regeneration**: `switch_profile` was not regenerating the `HANDOFF.md` file correctly.
- **Action**: Added handoff regeneration and profile bootstrapping to the switch logic.

### 2.4 Structural Alignment
- **Worktree Naming**: The naming convention in `create_worktree` was inconsistent with Phase 7 design docs and failing existing tests.
- **Action**: Aligned naming to `.worktrees/session-<id>-<slug>` and updated branch naming to `task-<id>-<slug>`.

## 3. Verification Results
- **Unit Tests**: 30/30 tests passed, including new test cases for slug validation and JSON-RPC compliance.
- **Compilation**: Clean build using `cargo +stable check`.

## 4. Residual Risks
- The MCP server currently runs on `stdio` and relies on the security of the parent process. If exposed to a network or untrusted pipe, further authentication mechanisms would be required.
