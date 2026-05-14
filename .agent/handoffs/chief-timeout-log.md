# Orch Chief Timeout Repro

Date: 2026-05-14
Task: `chief-timeout-log`

## Repro

From both `/mnt/e/atman-vessel` and `/mnt/e/rust-algo`:

```bash
orch task start <slug>
```

Observed behavior:

1. Pre-flight probes pass.
2. Worktree is created successfully.
3. Output reaches:

```text
✅ Worktree created at "<path>"
🚀 Launching agent chief...
```

4. Process exits after about 60 seconds with:

```text
Error: CLI process timed out after 60 seconds
```

## Impact

- Task worktree creation succeeds.
- Chief agent launch does not complete.
- Downstream users are forced to work manually inside the created Orch worktree.
- This breaks the intended "chief dispatch" workflow and creates ambiguity around task completion semantics.

## Reproduced Paths

- `/mnt/e/atman-vessel/.worktrees/session-7ab09d01-c9cc-4e62-b098-96c6d8abc217-chief-timeout-log`
- `/mnt/e/rust-algo/.worktrees/session-2a9f1425-afce-4951-917b-5a97dff20b46-replacement-option-candle-replay`
- `/mnt/e/rust-algo/.worktrees/session-2a9f1425-afce-4951-917b-5a97dff20b46-option-contract-resolution-feasibility`

This same pattern also occurred earlier for:

- `replacement-data-inventory`
- `stale-gate-120s-config`
- `replacement-full-replay-frame`

## Notes

- This is not a worktree-creation failure.
- It appears to be specifically in the post-create chief-launch phase.
- A useful next debugging step is to inspect whatever subprocess or IPC boundary is responsible for `Launching agent chief...`.
