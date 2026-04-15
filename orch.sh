#!/bin/bash

# Orchestrator v3 - Profile-Based Continuity Launcher
# Usage: ./orch.sh <launch|list|status> [profile] [gemini|codex] [args]

COMMAND=$1
PROFILE_NAME=$2
CLI_COMMAND=$3
ARGS=${@:4}

ORCH_ROOT="$(pwd)/.orchestrator"
PROFILES_DIR="$ORCH_ROOT/profiles"
SESSION_FILE="$ORCH_ROOT/session.json"

mkdir -p "$PROFILES_DIR"

list_profiles() {
  echo "--- Available Profiles ---"
  ls -1 "$PROFILES_DIR" 2>/dev/null || echo "No profiles created yet."
}

show_status() {
  echo "--- Orchestrator Status ---"
  if [ -f "$SESSION_FILE" ]; then
    cat "$SESSION_FILE" | grep -E "current_task|active_profile"
  else
    echo "No active session found."
  fi
}

case "$COMMAND" in
  list)
    list_profiles
    ;;
  status)
    show_status
    ;;
  launch)
    if [ -z "$PROFILE_NAME" ] || [ -z "$CLI_COMMAND" ]; then
      echo "Usage: ./orch.sh launch <profile_name> <gemini|codex> [args]"
      exit 1
    fi
    
    PROFILE_DIR="$PROFILES_DIR/$PROFILE_NAME"
    mkdir -p "$PROFILE_DIR/.gemini" "$PROFILE_DIR/.codex"
    
    # Bootstrap config from system HOME
    # WARNING: This is an EXPERIMENTAL PROTOTYPE. Future versions will use structured state management.
    BASE_GEMINI="$HOME/.gemini"
    BOOTSTRAP_FILES=("projects.json" "google_accounts.json" "oauth_creds.json" "settings.json" "state.json")
    
    for file in "${BOOTSTRAP_FILES[@]}"; do
        if [ ! -f "$PROFILE_DIR/.gemini/$file" ] && [ -f "$BASE_GEMINI/$file" ]; then
            cp "$BASE_GEMINI/$file" "$PROFILE_DIR/.gemini/"
        fi
    done

    # Update session state (DEPRECATED SED MUTATION - PHASE 1.5 ONLY)
    if [ -f "$SESSION_FILE" ]; then
        sed -i "s/\"active_profile\": \".*\"/\"active_profile\": \"$PROFILE_NAME\"/" "$SESSION_FILE"
    fi

    echo "🚀 Launching $CLI_COMMAND in profile: $PROFILE_NAME"
    echo "📂 Profile Path: $PROFILE_DIR"
    
    case "$CLI_COMMAND" in
      gemini)
        HOME="$PROFILE_DIR" gemini $ARGS
        ;;
      codex)
        HOME="$PROFILE_DIR" codex $ARGS
        ;;
      *)
        echo "Error: Unsupported CLI '$CLI_COMMAND'"
        exit 1
        ;;
    esac
    ;;
  *)
    echo "Usage: ./orch.sh <launch|list|status> [profile] [gemini|codex] [args]"
    exit 1
    ;;
esac
