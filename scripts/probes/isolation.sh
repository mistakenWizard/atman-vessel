#!/bin/bash

# Probe 1: Provider Profile Isolation (Hardened)
# Verify two distinct authenticated profiles can coexist and switch safely.

PROBE_DIR=".orchestrator/probe"
mkdir -p "$PROBE_DIR"
OUTPUT_FILE="$PROBE_DIR/provider-isolation.json"

echo "Running Probe 1: Provider Profile Isolation (Hardened)..."

# Create two profile directories
PROFILE_A=$(mktemp -d -p /tmp agy-profile-A.XXXXXX)
PROFILE_B=$(mktemp -d -p /tmp agy-profile-B.XXXXXX)

# Cleanup function
cleanup() {
    rm -rf "$PROFILE_A" "$PROFILE_B"
}
trap cleanup EXIT

# 1. Bootstrap Profile A with REAL system config
echo "Bootstrapping Profile A with real system config..."
mkdir -p "$PROFILE_A/.agy"
cp "$HOME/.agy/google_accounts.json" "$PROFILE_A/.agy/"
cp "$HOME/.agy/oauth_creds.json" "$PROFILE_A/.agy/"
cp "$HOME/.agy/settings.json" "$PROFILE_A/.agy/"
cp "$HOME/.agy/projects.json" "$PROFILE_A/.agy/"
cp "$HOME/.agy/state.json" "$PROFILE_A/.agy/"

# 2. Bootstrap Profile B with SIMULATED different identity
echo "Bootstrapping Profile B with simulated identity B..."
mkdir -p "$PROFILE_B/.agy"
cp "$PROFILE_A/.agy/"* "$PROFILE_B/.agy/"
# Modify Profile B's identity in the copied file (simulating a second account)
# We assume google_accounts.json has an email we can swap
if command -v jq &> /dev/null; then
    jq '.selectedAccount = "simulated-b@example.com"' "$PROFILE_B/.agy/google_accounts.json" > "$PROFILE_B/.agy/google_accounts.json.tmp" && mv "$PROFILE_B/.agy/google_accounts.json.tmp" "$PROFILE_B/.agy/google_accounts.json"
else
    sed -i 's/"email": ".*"/"email": "simulated-b@example.com"/' "$PROFILE_B/.agy/google_accounts.json"
fi

# 3. Verify A
echo "Checking Profile A identity..."
IDENTITY_A=$(HOME="$PROFILE_A" agy --prompt "Say only 'authenticated_a' if you can reach the model." --approval-mode plan 2>&1)

# 4. Verify B
echo "Checking Profile B identity..."
# Since B has a modified google_accounts.json, it might fail auth or show the new identity if the CLI supports it.
# If the CLI is sensitive to the account email, it should at least not show A's identity.
IDENTITY_B=$(HOME="$PROFILE_B" agy --prompt "Say only 'authenticated_b' if you can reach the model." --approval-mode plan 2>&1)

# 5. Switch back to A and verify
echo "Switching back to Profile A..."
IDENTITY_A_RECHECK=$(HOME="$PROFILE_A" agy --prompt "Say only 'still_authenticated_a'" --approval-mode plan 2>&1)

# 6. Verify no file contamination (Profile A files should not have Profile B's simulated email)
CONTAMINATION_DETECTED=false
if grep -q "simulated-b@example.com" "$PROFILE_A/.agy/google_accounts.json"; then
    CONTAMINATION_DETECTED=true
    echo "❌ CONTAMINATION: Profile A files modified by Profile B actions!"
fi

# Result logic
STATUS="pass"
if [[ "$IDENTITY_A_RECHECK" != *"still_authenticated_a"* ]]; then
    STATUS="fail"
    echo "❌ Profile A corrupted or lost auth during switch!"
fi
if [ "$CONTAMINATION_DETECTED" == "true" ]; then
    STATUS="fail"
fi

# Write results
cat <<EOF > "$OUTPUT_FILE"
{
  "timestamp": "$(date -u +'%Y-%m-%dT%H:%M:%SZ')",
  "profiles": {
    "A": {
      "initial_auth": $( [[ "$IDENTITY_A" == *"authenticated_a"* ]] && echo true || echo false ),
      "final_auth": $( [[ "$IDENTITY_A_RECHECK" == *"still_authenticated_a"* ]] && echo true || echo false )
    },
    "B": {
      "auth_attempt": $( [[ "$IDENTITY_B" == *"authenticated_b"* ]] && echo true || echo false )
    }
  },
  "contamination_detected": $CONTAMINATION_DETECTED,
  "status": "$STATUS"
}
EOF

echo "Results written to $OUTPUT_FILE"
