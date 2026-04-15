# Multi-Agent Orchestration Framework v2.1: Identity Isolation Model

## 1. Executive Summary
This framework manages **five (5) Gemini CLI accounts** and **one (1) Codex CLI account** by isolating each identity at the process level and externalizing task state into a project-local "Soul" (Context). This eliminates authentication collisions and prevents context amnesia during account rotation.

---

## 2. The "Vessel": Identity Isolation (Environment-Based)
Instead of swapping shared config folders, each account is launched in an isolated "Identity Bubble" using environment variable redirection. This allows multiple accounts to remain logged in and active simultaneously in different terminals.

### 2.1 Windows (PowerShell) Implementation
Add this to your `$PROFILE` or a dedicated `scripts/launch-identity.ps1`:

```powershell
function Launch-Gemini($Id) {
    $ProfilePath = "C:\Users\ganes\.gemini_profiles\acc_$Id"
    if (!(Test-Path $ProfilePath)) { New-Item -ItemType Directory -Path $ProfilePath -Force }
    
    # Isolate the process environment
    $env:USERPROFILE = $ProfilePath
    $env:HOME = $ProfilePath
    $env:LOCALAPPDATA = "$ProfilePath\AppData\Local"
    $env:APPDATA = "$ProfilePath\AppData\Roaming"
    
    Write-Host "🚀 Launching Gemini Identity #$Id..." -ForegroundColor Cyan
    # Start a new isolated shell or run a command
    powershell.exe -NoExit
}
```

### 2.2 WSL / Linux Implementation
```bash
launch_gemini() {
    export HOME="$HOME/.gemini_profiles/acc_$1"
    mkdir -p "$HOME"
    echo "🚀 Launching Gemini Identity #$1..."
    bash
}
```

---

## 3. The "Soul": Structured Context (HANDOFF.md)
The source of truth for the task is a project-local `HANDOFF.md`. This file is the "Mental Map" that allows a new Gemini account to "rehydrate" its intuition instantly.

### 3.1 Schema Definition
```markdown
# CURRENT TASK: [Short Name]
**Status:** [In-Progress | Blocked | Awaiting Review]
**Active Identity:** [Gemini-Acc-N]

## 1. Mental Map (Context)
- **Files in Focus:** List paths currently being modified.
- **Verified Logic:** What has been implemented and tested successfully?
- **Current Blocker:** What is the immediate technical hurdle?

## 2. Design Decisions
- **Decision:** [e.g., Use JSON for storage]
- **Rationale:** [e.g., Keeps context portable across WSL]

## 3. Next Atomic Steps (Directive for Account N+1)
1. [Actionable step 1]
2. [Actionable step 2]
3. [Verification command to run]
```

---

## 4. The "Orchestrator" Workflow

### Step 1: Planning (Codex)
Use Codex as the "Control Tower." Codex manages the `HANDOFF.md`.
*   **Action:** Codex analyzes the task and writes the initial `HANDOFF.md`.

### Step 2: Execution (Gemini - Identity N)
*   **Action:** Launch a terminal using `Launch-Gemini $N`.
*   **Bootup:** Paste: *"Context: Read `HANDOFF.md`. Resume from 'Next Atomic Steps'. Ignore session history."*

### Step 3: Rotation (Limit Hit)
*   **Action:** When Identity N is limited, instruct it: *"Update the 'Status' and 'Next Steps' in `HANDOFF.md` with current progress. Stop."*
*   **Handover:** Move to the terminal for `Identity N+1` and repeat the Bootup.

---

## 5. Compliance & Security
*   **ToS Alignment:** This is a sequential, manual workflow using legitimate consumer subscriptions. No automated quota pooling or auth-bypassing is performed.
*   **Security:** Authentication tokens remain in isolated `%USERPROFILE%\.gemini_profiles\` directories and are never committed to version control.
