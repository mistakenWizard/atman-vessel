#!/bin/bash

# AtmanVessel Orchestrator Shim (v1.2.0)
# This script delegates all commands to the high-integrity Rust core.

BINARY_PATH="./src-rust/target/debug/orch"

# Build if missing
if [ ! -f "$BINARY_PATH" ]; then
    echo "🔧 First run: Building orchestrator core..."
    (cd src-rust && cargo build --quiet)
fi

# Delegate to binary
$BINARY_PATH "$@"
