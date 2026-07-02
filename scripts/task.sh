#!/bin/bash

# Multi-Agent Task Management Script (Linux/Bash)
# Usage: ./scripts/task.sh <start|handoff|done> <slug>

ACTION=$1
SLUG=$2

if [ -z "$ACTION" ] || [ -z "$SLUG" ]; then
    echo "Usage: $0 <start|handoff|done> <slug>"
    exit 1
fi

# Configuration
BASE_DIR=$(pwd)
# In Linux, we'll place worktrees in a sibling directory as well
WORKTREE_ROOT_DIR=".worktrees"
TASK_DIR="$WORKTREE_ROOT_DIR/atman-vessel-task-$SLUG"
BRANCH_NAME="task/$SLUG"
CURRENT_BRANCH="master"

update_board() {
    local status=$1
    local agent=$2
    local board_path="$BASE_DIR/.agent/BOARD.md"
    
    # Use sed to update the specific task line
    # Format: | `slug` | Agent | Status | Path |
    # We use a temporary file for safety
    sed -i "s/| \`$SLUG\` | .* | .* | .* |/| \`$SLUG\` | $agent | $status | ${TASK_DIR//\//\\/} |/g" "$board_path"
}

case $ACTION in
    "start")
        echo "🚀 Starting task: $SLUG"
        if [ -d "$TASK_DIR" ]; then
            echo "Found existing worktree at $TASK_DIR."
        else
            echo "Creating new worktree at $TASK_DIR from $CURRENT_BRANCH..."
            git worktree add -b "$BRANCH_NAME" "$TASK_DIR" "$CURRENT_BRANCH"
        fi
        update_board "In Progress" "Antigravity-CLI"
        echo "✅ Task initialized. Please 'cd $TASK_DIR' to begin."
        ;;
        
    "handoff")
        echo "📦 Handing off task: $SLUG"
        HANDOFF_FILE="$BASE_DIR/.agent/handoffs/$SLUG.md"
        mkdir -p "$BASE_DIR/.agent/handoffs"
        if [ ! -f "$HANDOFF_FILE" ]; then
            echo "Please fill in handoff details for $SLUG" > "$HANDOFF_FILE"
        fi
        update_board "Paused" "None"
        echo "✅ Handoff prepared at $HANDOFF_FILE"
        ;;
        
    "done")
        echo "🏁 Finishing task: $SLUG"
        update_board "Done" "None"
        echo "Please merge branch $BRANCH_NAME into $CURRENT_BRANCH and run 'git worktree remove $TASK_DIR' manually."
        ;;
        
    *)
        echo "Unknown action: $ACTION"
        exit 1
        ;;
esac
