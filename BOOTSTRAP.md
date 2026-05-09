# AtmanVessel: Universal Agent Bootstrap

If you are an AI agent reading this, you are being integrated into the **AtmanVessel Orchestration Framework**. Follow these steps to establish connectivity and assume your role as a Vessel.

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
- `start_task`: To spawn specialist sub-agents.
- `switch_profile`: To rotate identities and bypass rate limits.
- `freeze_session`: To persist critical findings.

## 4. Operational Mandates
- **Autonomy**: Always set `export GEMINI_CLI_TRUST_WORKSPACE=true`.
- **Isolation**: Always work inside the worktree path provided by `orch task start`.
- **Provenance**: Use `orch verify` to promote your findings.

## 5. Quick Start for New Repos
If you are in a new repository and need to bring it under AtmanVessel control:
1. Run `orch init` to scaffold the `.orchestrator` directory.
2. Run `orch task start <slug>` to create your first isolated environment.

**Reference the local `AGENT_GUIDE.md`, `MCP_GUIDE.md`, and `CLI_GUIDE.md` for deep technical details.**
