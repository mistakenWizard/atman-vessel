#!/bin/bash

# AtmanVessel Orchestrator Shim (v1.2.1)
# This script delegates all commands to the high-integrity Rust core.

# Resolve the absolute path of the script's directory
SCRIPT_DIR="$(cd "$(dirname "$(readlink -f "${BASH_SOURCE[0]}")")" && pwd)"
BINARY_PATH="$SCRIPT_DIR/src-rust/target/debug/orch"

# Build if missing
if [ ! -f "$BINARY_PATH" ]; then
    echo "🔧 First run: Building orchestrator core..." >&2
    (cd "$SCRIPT_DIR/src-rust" && cargo build --quiet)
fi

# Delegate to binary
"$BINARY_PATH" "$@"
