# .antigravity.md Quality Standards

## 1. Commands & Workflows (Weight: 40%)
Instructions must be derived from source-of-truth configuration files:
- **Node.js**: `package.json` scripts.
- **Rust**: `Cargo.toml` and `Makefile`.
- **Python**: `pyproject.toml` or `Makefile`.
- **Go**: `go.mod` and common `Taskfile` or `Makefile`.

**Antigravity 2.0**: All CLI commands should use `agy` instead of `gemini`.

## 2. Architecture & Mapping (Weight: 30%)
High-signal mapping of the directory structure.
- Focus on where logic lives vs. where configuration lives.
- Define boundaries between subsystems.

## 3. Idioms & Gotchas (Weight: 20%)
Capture project-specific patterns that a general model wouldn't know.
- "We use manual dependency injection instead of a framework."
- "Avoid using the `fs` module; use our internal `vfs` wrapper."
- "All dates must be UTC and stored as ISO-8601 strings."
- **Permissions**: Document if `/permissions` must be granted for specific directories in Antigravity.
- **Profiles**: Document if the project depends on specific named profiles (e.g., `chief`, `builder`).

## 4. Signal-to-Noise Ratio (Weight: 10%)
- Remove: "This project is built with React and is designed to be scalable."
- Keep: "React 18 with Concurrent Mode enabled. Avoid class components."

## Audit Checklist
1. [ ] Do all build/test commands execute successfully in the current environment?
2. [ ] Are all required environment variables listed with example values?
3. [ ] Is the branching/deployment strategy documented if non-standard?
4. [ ] Are there "dead" instructions referring to removed features?
