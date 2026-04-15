#!/bin/bash

# Probe 3: Lock Liveness And Stale Lock Recovery
# Validate PID-sensitive locks.

PROBE_DIR=".orchestrator/probe"
mkdir -p "$PROBE_DIR"
OUTPUT_FILE="$PROBE_DIR/lock-liveness.json"

echo "Running Probe 3: Lock Liveness..."

LOCK_FILE="/tmp/orchestrator-test.lock"

# Function to check if a PID is live
is_live() {
    kill -0 "$1" 2>/dev/null
}

# Test 1: Live Lock
MY_PID=$$
echo "$MY_PID" > "$LOCK_FILE"
LIVE_DETECTED=false
if is_live "$(cat "$LOCK_FILE")"; then
    LIVE_DETECTED=true
    echo "✅ Live PID lock detected as live."
fi

# Test 2: Dead Lock
DEAD_PID=999999
while is_live "$DEAD_PID"; do
    DEAD_PID=$((DEAD_PID - 1))
done

echo "$DEAD_PID" > "$LOCK_FILE"
STALE_DETECTED=false
if ! is_live "$(cat "$LOCK_FILE")"; then
    STALE_DETECTED=true
    echo "✅ Stale PID lock detected as stale."
fi

# Write results
cat <<EOF > "$OUTPUT_FILE"
{
  "timestamp": "$(date -u +'%Y-%m-%dT%H:%M:%SZ')",
  "live_lock": {
    "pid": $MY_PID,
    "detected_live": $LIVE_DETECTED
  },
  "stale_lock": {
    "pid": $DEAD_PID,
    "detected_stale": $STALE_DETECTED
  },
  "status": "$( [ "$LIVE_DETECTED" == "true" ] && [ "$STALE_DETECTED" == "true" ] && echo "pass" || echo "fail" )"
}
EOF

echo "Results written to $OUTPUT_FILE"
rm -f "$LOCK_FILE"
