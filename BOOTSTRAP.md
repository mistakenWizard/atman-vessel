# AtmanVessel: Universal Minion Bootstrap

If you are an AI agent reading this, you are being integrated into the **AtmanVessel Orchestration Framework** as a **Minion**. Follow these steps to establish connectivity and assume your role.

## 1. Locate the Orchestrator
The `orch` command is globally available on this system. You do not need to navigate to a specific path. 
- Run `orch status` to verify the framework is active.
- Run `orch profiles list` to see your available identities (Vessels).
- **Authenticated Fleet**: `gayatri`, `gvklive`, `hilarioushabit`, `hollowhistory`, `hypnotichallway`, `mistakenWizard`. Use these names specifically.

## 2. Connect to the "Soul" (Project State)
AtmanVessel maintains a persistent state in `.orchestrator/session.json`. 
- **DO NOT** rely on your internal memory for project history.
- **DO** read `.orchestrator/HANDOFF.md` immediately upon arrival in any worktree.
- **DO** use `orch freeze -m "..."` to checkpoint your mental map before finishing a turn.

## 3. Tool Access (MCP)
If you are running via Gemini or Codex, the `atman-vessel` MCP server is already registered. You have access to:
- `start_task`: To spawn specialist **Minion** worktrees.
- `switch_profile`: To rotate identities and bypass rate limits.
- `freeze_session`: To persist critical findings.

## 4. Operational Mandates
- **Autonomy**: Always set `export GEMINI_CLI_TRUST_WORKSPACE=true`.
- **Isolation**: Always work inside the worktree path provided by `orch task start`.
- **Timeout**: You have a **strict 60-second limit** per turn. Work in small increments.
- **Provenance**: Use `orch verify` to promote your findings.

## 5. Merging & Cleanup
Once your task is complete:
1. Ensure your results/analysis are written to files in your worktree.
2. The user or spawning agent will run `orch task finish <slug>` to merge your work back to the parent branch.
3. The worktree will then be marked as **Stale** and removed via `orch cleanup`.
