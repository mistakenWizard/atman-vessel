#!/bin/bash

# Probe 2: Platform Matrix (WSL/Windows Hardened)
# Compare path handling and environment inheritance.

PROBE_DIR=".orchestrator/probe"
mkdir -p "$PROBE_DIR"
OUTPUT_FILE="$PROBE_DIR/platform-matrix.json"

echo "Running Probe 2: Platform Matrix..."

IS_WSL=false
if grep -qEi "(Microsoft|WSL)" /proc/version &> /dev/null; then
    IS_WSL=true
fi

# Path handling check
PATH_STYLE="unix"
if [[ "$PWD" == "/mnt/"* ]]; then
    PATH_STYLE="wsl-mounted"
fi

# Environment inheritance check
# Check for Windows-style paths in environment (since USERPROFILE might be missing in some WSL configs)
HAS_WIN_VARS=false
if printenv | grep -q "/mnt/c/Users"; then
    HAS_WIN_VARS=true
fi

# CLI config behavior (checking if agy is available as a command)
GEMINI_PATH=$(which agy)

# Write results
cat <<EOF > "$OUTPUT_FILE"
{
  "timestamp": "$(date -u +'%Y-%m-%dT%H:%M:%SZ')",
  "platform": {
    "os": "$(uname -s)",
    "is_wsl": $IS_WSL,
    "path_style": "$PATH_STYLE"
  },
  "environment": {
    "has_windows_vars": $HAS_WIN_VARS,
    "gemini_path": "$GEMINI_PATH"
  },
  "status": "pass"
}
EOF

echo "Results written to $OUTPUT_FILE"
