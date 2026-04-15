#!/bin/bash

# Master Probe Runner for AtmanVessel Orchestrator Phase 1.5

PROBE_DIR="scripts/probes"
RESULT_DIR=".orchestrator/probe"
mkdir -p "$RESULT_DIR"

echo "=== AtmanVessel Probe Harness ==="

# Run all probes in the probes directory
for probe in "$PROBE_DIR"/*.sh; do
    if [ -f "$probe" ]; then
        bash "$probe"
    fi
done

echo "=== Generating Summary ==="

# Simple summary generator
SUMMARY_FILE="$RESULT_DIR/summary.md"
cat <<EOF > "$SUMMARY_FILE"
# Phase 1: Probe Harness Summary (Reproducible)

## Overview
Results of the reproducible probe harness.

| Probe | Status | Result |
| :--- | :--- | :--- |
EOF

# Extract status from JSON results
for json in "$RESULT_DIR"/*.json; do
    NAME=$(basename "$json" .json)
    STATUS=$(grep -oP '"status": "\K[^"]+' "$json")
    if [ "$STATUS" == "pass" ]; then
        STATUS_EMOJI="**PASS**"
    else
        STATUS_EMOJI="**FAIL**"
    fi
    echo "| $NAME | $STATUS_EMOJI | See $NAME.json |" >> "$SUMMARY_FILE"
done

cat <<EOF >> "$SUMMARY_FILE"

## Detailed Findings

Refer to the individual JSON files in \`.orchestrator/probe/\` for raw data.
EOF

echo "All probes completed. Summary at $SUMMARY_FILE"
