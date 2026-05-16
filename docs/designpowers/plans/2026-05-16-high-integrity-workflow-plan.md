# Design Plan: High-Integrity Minion Workflow

> **For agentic workers:** REQUIRED: Use designpowers:designpowers-critique to review completed work against this plan.

**Goal:** Transform the AtmanVessel minion ecosystem from a "dumb" state to a high-integrity, specialized swarm using standardized workflow skills.

**Design Direction:** [High-Integrity Workflow Brief](docs/designpowers/briefs/2026-05-16-high-integrity-workflow.md)

**Personas:** The Creative Director (Human Operator)

---

## Task 1: Update Agent Standards & Guides

**Files:** `AGENT_GUIDE.md`, `AGENT_STANDARDS.md`, `GEMINI.md`

- [ ] Step 1: Add "Discovery First" mandate to `AGENT_GUIDE.md`.
- [ ] Step 2: Update `AGENT_STANDARDS.md` to include TDD and Designpowers as mandatory skills for all tasks.
- [ ] Step 3: Refine the "YOLO" mode instructions in `GEMINI.md` to prioritize planning over speed.

**Accessibility check:** Ensure standards are written in plain, unambiguous language for AI interpretation.

**Verification:** Read the updated files and verify the new mandates are clear.

---

## Task 2: Enhance Orchestrator Core (Rust)

**Files:** `src-rust/src/main.rs`, `src-rust/src/orchestrator.rs`

- [ ] Step 1: Update `orch task start` to automatically create a `brain/plans/<slug>-plan.md` template.
- [ ] Step 2: Modify `orch task finish` to automatically run `git worktree remove` and check for untracked files.
- [ ] Step 3: Implement a "Sanity Check" before `finish` that verifies if a brief exists in `docs/designpowers/briefs/`.

**Accessibility check:** Use descriptive error messages when a task fails the "Integrity Check."

**Verification:** Run `cargo test` and manually test the `start/finish` loop in a temp worktree.

---

## Task 3: Implement Template-Based Delegation

**Files:** `scripts/task_template.md` (new), `AGENT_GUIDE.md`

- [ ] Step 1: Create a high-signal prompt template for delegation that forces context, problem, and success criteria.
- [ ] Step 2: Update `orch launch` instructions in `AGENT_GUIDE.md` to use this template.

**Accessibility check:** Ensure the template includes "Ability Spectrum" as a required field.

**Verification:** Simulate a delegation using the new template.

---

## Task 4: Standardize Investigation Logs

**Files:** `GEMINI.md`, `AGENT_GUIDE.md`

- [ ] Step 1: Enforce the use of `/mnt/e/investigations/` for all session summaries.
- [ ] Step 2: Create a `daily/YYYY-MM-DD/` auto-logger shim in `orch`.

**Accessibility check:** Ensure logs are structured for easy cross-referencing by other minions.

**Verification:** Verify that a new log is created correctly.
