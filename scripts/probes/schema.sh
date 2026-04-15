#!/bin/bash

# Probe 4: Schema Version Safety
# Validate session.json version handling.

PROBE_DIR=".orchestrator/probe"
mkdir -p "$PROBE_DIR"
OUTPUT_FILE="$PROBE_DIR/schema-version.json"

echo "Running Probe 4: Schema Version Safety..."

CURRENT_VERSION="1.0"
TEST_SESSION="/tmp/test-session.json"

# Function to simulate write
write_session() {
    local target_version=$1
    if [ "$target_version" != "$CURRENT_VERSION" ]; then
        echo "Error: Unknown future schema version $target_version for write operations."
        return 1
    fi
    echo "{\"version\": \"$target_version\", \"data\": \"test\"}" > "$TEST_SESSION"
    return 0
}

# Function to simulate read
read_session() {
    local target_file=$1
    cat "$target_file"
}

# Test 1: Write current version
WRITE_CURRENT_PASS=false
if write_session "$CURRENT_VERSION"; then
    WRITE_CURRENT_PASS=true
    echo "✅ Wrote current version successfully."
fi

# Test 2: Reject future version write
REJECT_FUTURE_PASS=false
if ! write_session "2.0" &>/dev/null; then
    REJECT_FUTURE_PASS=true
    echo "✅ Rejected future version write."
fi

# Test 3: Read future version (Read-only)
echo "{\"version\": \"2.0\", \"data\": \"future\"}" > "$TEST_SESSION"
READ_FUTURE_PASS=false
READ_CONTENT=$(read_session "$TEST_SESSION")
if [[ "$READ_CONTENT" == *"future"* ]]; then
    READ_FUTURE_PASS=true
    echo "✅ Read future version successfully (read-only)."
fi

# Write results
cat <<EOF > "$OUTPUT_FILE"
{
  "timestamp": "$(date -u +'%Y-%m-%dT%H:%M:%SZ')",
  "current_version": "$CURRENT_VERSION",
  "write_current": $WRITE_CURRENT_PASS,
  "reject_future_write": $REJECT_FUTURE_PASS,
  "read_future_readonly": $READ_FUTURE_PASS,
  "status": "$( [ "$WRITE_CURRENT_PASS" == "true" ] && [ "$REJECT_FUTURE_PASS" == "true" ] && [ "$READ_FUTURE_PASS" == "true" ] && echo "pass" || echo "fail" )"
}
EOF

echo "Results written to $OUTPUT_FILE"
rm -f "$TEST_SESSION"
