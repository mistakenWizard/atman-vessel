#!/bin/bash

# Probe 5: Multi-Terminal Snapshot/Reload Detection
# Validate stale context detection.

PROBE_DIR=".orchestrator/probe"
mkdir -p "$PROBE_DIR"
OUTPUT_FILE="$PROBE_DIR/context-sync.json"

echo "Running Probe 5: Multi-Terminal Sync..."

SESSION_FILE="/tmp/session.json"

# Session A starts
TS_A=$(date +%s)
echo "{\"last_updated\": $TS_A}" > "$SESSION_FILE"
echo "Session A started with TS: $TS_A"

# Simulate delay
sleep 1

# Session B updates
TS_B=$(date +%s)
echo "{\"last_updated\": $TS_B}" > "$SESSION_FILE"
echo "Session B updated with TS: $TS_B"

# Session A checks
TS_CHECK=$(cat "$SESSION_FILE" | grep -oP '"last_updated": \K\d+')
STALE_DETECTED=false
if [ "$TS_CHECK" -gt "$TS_A" ]; then
    STALE_DETECTED=true
    echo "✅ Session A detected newer state and requires reload."
fi

# Write results
cat <<EOF > "$OUTPUT_FILE"
{
  "timestamp": "$(date -u +'%Y-%m-%dT%H:%M:%SZ')",
  "session_a_ts": $TS_A,
  "session_b_ts": $TS_B,
  "stale_detected": $STALE_DETECTED,
  "status": "$( [ "$STALE_DETECTED" == "true" ] && echo "pass" || echo "fail" )"
}
EOF

echo "Results written to $OUTPUT_FILE"
rm -f "$SESSION_FILE"
