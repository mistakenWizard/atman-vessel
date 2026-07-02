---
name: agy-md-improver
description: Audits and improves project instruction files (.antigravity.md) to ensure optimal AI context and execution accuracy. Use when project instructions are stale, missing key workflows, or lack actionable technical standards.
---

# Antigravity MD Improver

This skill evaluates and hardens project instruction files (`.antigravity.md`) to maintain high-signal context for AI agents.

## Core Mandates

1. **Project-Specific Only**: Never add generic "best practices." Every instruction must be derived from the actual codebase.
2. **Actionable Commands**: Ensure every build, test, and deploy command is accurate and copy-pasteable.
3. **Density Over Verbosity**: Favor concise lists and code blocks. Delete fluff and obvious explanations.
4. **Report First**: Always provide a Quality Audit report before proposing or applying changes.

## Workflow

### 1. Discovery
Search for all instruction files in the repository:
- Root `./.antigravity.md`
- Subdirectory `./src/**/.antigravity.md`
- Context files like `README.md` or `CONTRIBUTING.md` that contain workflows.

### 2. Quality Audit
Evaluate files against these criteria:
- **Build/Test/Deploy (High)**: Accurate commands for the current tech stack.
- **Architecture (High)**: Mapping of key directories and system boundaries.
- **Gotchas/Idioms (Medium)**: Project-specific quirks (e.g., "Always use X instead of Y").
- **Currency (High)**: Alignment with `package.json`, `Cargo.toml`, `requirements.txt`, etc.

**Grading Scale**:
- **A (90-100)**: Fully actionable, concise, and perfectly reflects the current state.
- **C (70-80)**: Missing 1-2 key workflows or contains slightly stale info.
- **F (<60)**: Missing, generic, or wildly inaccurate.

### 3. Reporting
Output a `.antigravity.md Quality Report`:
- **File**: [Path]
- **Score**: [0-100]
- **Findings**: [List specific gaps found via codebase search]
- **Proposed Additions**: [Surgical diff or quoted lines]

### 4. Implementation
Update files only after user approval. Maintain the existing file style and structure.

## Quality Standards Reference

For detailed rubric and examples, see [references/standards.md](references/standards.md).

## Common Flags to Detect
- Documented commands that don't exist in config files.
- Missing setup for required environment variables.
- Outdated test runner instructions (e.g., `jest` mentioned but `vitest` installed).
- Complex logic in the code that is not documented as a "Gotcha."
