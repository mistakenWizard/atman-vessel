# Phase 1: Probe Harness Summary

## Overview
This document summarizes the results of the Phase 1 environment validation probes for the AtmanVessel Orchestrator.

| Probe | Area | Status | Result |
| :--- | :--- | :--- | :--- |
| 1 | Provider Isolation | **PASS** | `HOME` redirection successfully isolates Gemini profiles. |
| 2 | Platform Matrix | **PASS** | WSL environment detected; path and env inheritance documented. |
| 3 | Lock Liveness | **PASS** | PID-sensitive locks correctly identify live vs stale processes. |
| 4 | Schema Safety | **PASS** | Write-protection for future schema versions validated. |
| 5 | Context Sync | **PASS** | Multi-terminal stale state detection confirmed via timestamp. |

## Detailed Findings

### 1. Provider Profile Isolation
- **Mechanism**: Setting `HOME` to a profile-specific directory.
- **Verification**: Profile A (bootstrapped with auth files) was authenticated; Profile B (empty root) was not. No leakage observed.
- **Risk**: Authentication requires specific files: `google_accounts.json`, `oauth_creds.json`, `settings.json`, `projects.json`, and `state.json`. `orch.sh` needs to be updated to bootstrap all of these if automatic authentication is desired.

### 2. Platform Matrix
- **Environment**: WSL (Ubuntu) on Windows.
- **Path Style**: WSL-mounted (`/mnt/e/...`).
- **Inheritance**: Windows environment variables (like `USERPROFILE`) are visible in WSL but do not seem to affect Gemini CLI's preference for `$HOME`.

### 3. Lock Liveness
- **Mechanism**: Files containing PID.
- **Validation**: `kill -0 <PID>` is a reliable way to check if a process is still running.
- **Recovery**: Stale locks can be safely identified and should trigger a `crashed` state transition.

### 4. Schema Safety
- **Mechanism**: Version field in `session.json`.
- **Validation**: System correctly rejects writes to higher versions while allowing reads.

### 5. Context Sync
- **Mechanism**: `last_updated` timestamp in `session.json`.
- **Validation**: Cross-session awareness is possible by checking the timestamp before any state mutation.

## Recommendations
- **Update `orch.sh`**: Ensure all necessary auth files are bootstrapped for new profiles.
- **Proceed to Phase 2**: The environmental assumptions are proven stable. Profile isolation is reliable, and lock/sync mechanisms are feasible.

## Environment Tested
- **OS**: Linux (WSL)
- **Gemini CLI**: 0.38.0
- **Timestamp**: 2026-04-15
