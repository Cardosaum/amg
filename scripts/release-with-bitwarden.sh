#!/usr/bin/env bash
# release-with-bitwarden.sh
# Helper script to run release-plz commands with tokens retrieved from Bitwarden CLI
#
# Usage:
#   ./scripts/release-with-bitwarden.sh release-pr
#   ./scripts/release-with-bitwarden.sh release
#
# Environment Variables:
#   BW_GITHUB_TOKEN_ITEM - Custom name for GitHub token item (default: amg-release-github-token)
#   BW_CARGO_TOKEN_ITEM - Custom name for Cargo token item (default: amg-release-cargo-token)
#   BW_PASSWORD - Master password for non-interactive unlock (discouraged; prefer exporting BW_SESSION)
#   BW_DEBUG - Set to 1 to enable debug output
#
# This script:
# 1. Checks if Bitwarden CLI is installed
# 2. Checks if user is logged in to Bitwarden
# 3. Unlocks the Bitwarden vault (prompts for password if needed)
# 4. Retrieves GITHUB_TOKEN from Bitwarden vault
# 5. Optionally retrieves CARGO_REGISTRY_TOKEN
# 6. Runs release-plz with the retrieved tokens
# 7. Locks the vault after use

set -euo pipefail

# Colors for output
readonly BLUE='\033[0;34m'
readonly GREEN='\033[0;32m'
readonly YELLOW='\033[0;33m'
readonly RED='\033[0;31m'
readonly NC='\033[0m' # No Color

# Bitwarden vault item names (customize these to match your vault)
readonly GITHUB_TOKEN_ITEM="${BW_GITHUB_TOKEN_ITEM:-amg-release-github-token}"
readonly CARGO_TOKEN_ITEM="${BW_CARGO_TOKEN_ITEM:-amg-release-cargo-token}"

# Script directory
readonly SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
readonly PROJECT_ROOT="$(cd "${SCRIPT_DIR}/.." && pwd)"

# Debug mode (set BW_DEBUG=1 to enable)
readonly DEBUG="${BW_DEBUG:-0}"

# Logging functions
log_debug() {
    if [ "$DEBUG" = "1" ]; then
        echo -e "${BLUE}[DEBUG]${NC} $*" >&2
    fi
}

log_info() {
    echo -e "${BLUE}$*${NC}" >&2
}

log_success() {
    echo -e "${GREEN}✓${NC} $*" >&2
}

log_warning() {
    echo -e "${YELLOW}⚠${NC} $*" >&2
}

log_error() {
    echo -e "${RED}Error:${NC} $*" >&2
}

# Check if Bitwarden CLI is installed
check_bw_installed() {
    if ! command -v bw &> /dev/null; then
        log_error "Bitwarden CLI (bw) is not installed."
        echo -e "${YELLOW}Install it with:${NC}" >&2
        echo "  brew install bitwarden-cli  # macOS" >&2
        echo "  # See: https://bitwarden.com/help/cli/" >&2
        exit 1
    fi
    
    # Check if jq is installed (needed for JSON parsing)
    if ! command -v jq &> /dev/null; then
        log_error "jq is not installed (required for JSON parsing)."
        echo -e "${YELLOW}Install it with:${NC}" >&2
        echo "  brew install jq  # macOS" >&2
        echo "  sudo apt-get install jq  # Debian/Ubuntu" >&2
        exit 1
    fi
    
    log_debug "Bitwarden CLI and jq are installed"
}

# Get Bitwarden status
get_bw_status() {
    local status_json
    status_json=$(bw status 2>/dev/null || echo '{"status":"error"}')
    
    if [ "$status_json" = '{"status":"error"}' ]; then
        log_error "Failed to get Bitwarden status"
        return 1
    fi
    
    echo "$status_json"
}

# Check if user is logged in to Bitwarden
check_bw_logged_in() {
    local status_json status
    
    status_json=$(get_bw_status) || return 1
    status=$(echo "$status_json" | jq -r '.status' 2>/dev/null || echo "error")
    
    log_debug "Bitwarden status: $status"
    
    # Status can be: "unauthenticated", "authenticated", "locked", "unlocked"
    case "$status" in
        "unauthenticated"|"error"|"")
            log_error "Not logged in to Bitwarden."
            echo -e "${YELLOW}Please log in first:${NC}" >&2
            echo "  bw login" >&2
            echo "" >&2
            echo -e "${BLUE}For automation, consider using API keys:${NC}" >&2
            echo "  export BW_CLIENTID=your_client_id" >&2
            echo "  export BW_CLIENTSECRET=your_client_secret" >&2
            echo "  bw login --apikey" >&2
            exit 1
            ;;
        "authenticated"|"locked"|"unlocked")
            log_debug "User is logged in (status: $status)"
            return 0
            ;;
        *)
            log_warning "Unknown Bitwarden status: $status"
            # Try to continue anyway
            return 0
            ;;
    esac
}

# Unlock Bitwarden vault and get session key
unlock_vault() {
    local session_key status_json status
    
    # First, check if BW_SESSION is already set (user might have unlocked manually)
    if [ -n "${BW_SESSION:-}" ] && [ ${#BW_SESSION} -gt 10 ]; then
        log_debug "Found existing BW_SESSION environment variable"
        # Verify the session is still valid
        if bw list folders --session "$BW_SESSION" &>/dev/null 2>&1; then
            log_debug "Using existing BW_SESSION from environment"
            echo "$BW_SESSION"
            return 0
        else
            log_debug "BW_SESSION exists but appears invalid, will unlock again"
        fi
    fi
    
    status_json=$(get_bw_status) || return 1
    status=$(echo "$status_json" | jq -r '.status' 2>/dev/null || echo "")
    
    log_debug "Vault status before unlock: $status"
    
    # If already unlocked, try to get existing session
    if [ "$status" = "unlocked" ]; then
        log_debug "Vault is already unlocked, checking for existing session"
        # Try to get session from unlock --check (only works when unlocked)
        session_key=$(bw unlock --check 2>/dev/null || echo "")
        
        if [ -n "$session_key" ] && [ "$session_key" != "null" ] && [ ${#session_key} -gt 10 ]; then
            log_debug "Using existing unlocked session from --check"
            echo "$session_key"
            return 0
        fi
    fi
    
    # Vault is locked or session expired, need to unlock
    log_info "Unlocking Bitwarden vault..."
    
    # Try non-interactive unlock if password is provided via environment variable
    if [ -n "${BW_PASSWORD:-}" ]; then
        log_debug "Attempting non-interactive unlock using BW_PASSWORD"
        session_key=$(bw unlock --passwordenv BW_PASSWORD --raw 2>&1)
        local unlock_exit_code=$?
        
        if [ $unlock_exit_code -eq 0 ] && [ -n "$session_key" ] && [ "$session_key" != "null" ] && [ ${#session_key} -gt 10 ]; then
            log_debug "Non-interactive unlock successful"
            echo "$session_key"
            return 0
        else
            log_warning "Non-interactive unlock failed, falling back to interactive"
            log_debug "Unlock output: ${session_key:0:100}"
        fi
    fi
    
    # Check if we're in an interactive terminal (for password prompt)
    # When running from Makefile, stdin/stdout might be redirected, so we need to check carefully
    local is_interactive=true
    
    # Check if stdin is connected to a terminal
    # Makefile with @ might redirect, but we can still be interactive
    if [ ! -t 0 ]; then
        # Stdin is not a terminal - check if it's because of Makefile
        if [ -n "${MAKELEVEL:-}" ]; then
            # Running from Makefile - check if we can still read from terminal
            # Try to detect if we're in a real terminal session
            if [ -t 1 ] && [ -c /dev/tty ]; then
                # stdout is terminal and /dev/tty exists - likely interactive
                is_interactive=true
                log_debug "Detected interactive terminal via /dev/tty"
            else
                is_interactive=false
            fi
        else
            is_interactive=false
        fi
    fi
    
    log_debug "Terminal check: is_interactive=$is_interactive, stdin_tty=$([ -t 0 ] && echo yes || echo no), stdout_tty=$([ -t 1 ] && echo yes || echo no)"
    
    if [ "$is_interactive" = "false" ] && [ -z "${BW_PASSWORD:-}" ]; then
        log_error "Not in an interactive terminal and BW_PASSWORD not set. Cannot prompt for password."
        echo "" >&2
        echo -e "${YELLOW}Quick Solution (Recommended):${NC}" >&2
        echo "  export BW_SESSION=\$(bw unlock --raw)" >&2
        echo "  make release-pr-bw" >&2
        echo "" >&2
        echo -e "${YELLOW}Other Options:${NC}" >&2
        echo "  1. Set BW_PASSWORD environment variable:" >&2
        echo "     export BW_PASSWORD=your_password" >&2
        echo "     make release-pr-bw" >&2
        echo "" >&2
        echo "  2. Use API keys for automation:" >&2
        echo "     export BW_CLIENTID=your_client_id" >&2
        echo "     export BW_CLIENTSECRET=your_client_secret" >&2
        echo "     bw login --apikey" >&2
        echo "     export BW_SESSION=\$(bw unlock --raw)" >&2
        echo "" >&2
        exit 1
    fi
    
    # Interactive unlock (will prompt for password if interactive)
    if [ "$is_interactive" = "true" ]; then
        log_info "Please enter your Bitwarden master password:"
        # Use a timeout to prevent infinite hanging, but allow enough time for user input
        # Read from /dev/tty if available to ensure we can prompt even from Makefile
        if [ -c /dev/tty ]; then
            # Use /dev/tty to ensure we can read from terminal even if stdin is redirected
            if command -v timeout &> /dev/null || command -v gtimeout &> /dev/null; then
                local timeout_cmd="timeout"
                [ -z "$(command -v timeout)" ] && timeout_cmd="gtimeout"
                # Allow 120 seconds for user to enter password
                # Read password prompt from /dev/tty
                session_key=$($timeout_cmd 120 bw unlock --raw < /dev/tty 2>&1)
                local unlock_exit_code=$?
            else
                # No timeout, read from /dev/tty
                session_key=$(bw unlock --raw < /dev/tty 2>&1)
                local unlock_exit_code=$?
            fi
        else
            # /dev/tty not available, try normal stdin
            if command -v timeout &> /dev/null || command -v gtimeout &> /dev/null; then
                local timeout_cmd="timeout"
                [ -z "$(command -v timeout)" ] && timeout_cmd="gtimeout"
                session_key=$($timeout_cmd 120 bw unlock --raw 2>&1)
                local unlock_exit_code=$?
            else
                session_key=$(bw unlock --raw 2>&1)
                local unlock_exit_code=$?
            fi
        fi
        
        # Check if timeout occurred
        if [ $unlock_exit_code -eq 124 ] || [ $unlock_exit_code -eq 143 ]; then
            log_error "Unlock timed out (waited 120 seconds for password input)"
            echo -e "${YELLOW}Try unlocking manually and exporting the session:${NC}" >&2
            echo "  export BW_SESSION=\$(bw unlock --raw)" >&2
            echo "  make release-pr-bw" >&2
            exit 1
        fi
    else
        # Non-interactive: should have been handled by BW_PASSWORD check above
        log_error "Unexpected non-interactive state without BW_PASSWORD"
        exit 1
    fi
    
    # Check if unlock failed or returned an error message
    if [ $unlock_exit_code -ne 0 ] || [ -z "$session_key" ] || [ "$session_key" = "null" ] || \
       [[ "$session_key" == *"error"* ]] || [[ "$session_key" == *"Error"* ]] || \
       [ ${#session_key} -le 10 ]; then
        log_error "Failed to unlock Bitwarden vault"
        if [[ "$session_key" == *"Invalid master password"* ]] || [[ "$session_key" == *"invalid password"* ]]; then
            echo -e "${YELLOW}Invalid master password. Please try again.${NC}" >&2
        else
            echo -e "${YELLOW}Make sure you're logged in and enter the correct master password.${NC}" >&2
            if [ "$DEBUG" = "1" ]; then
                log_debug "Unlock output: ${session_key:0:200}"
            fi
        fi
        exit 1
    fi
    
    log_debug "Vault unlocked successfully"
    echo "$session_key"
}

# Run bw command with timeout and proper error handling
# Usage: bw_cmd <timeout_seconds> bw <args...>
bw_cmd() {
    local timeout_seconds="${1:-10}"

    # Require at least: <timeout_seconds> <command>
    if [ "$#" -lt 2 ]; then
        log_error "bw_cmd: missing command"
        return 2
    fi
    shift

    local cmd_output
    local exit_code

    # Use timeout if available, otherwise just run the command
    if command -v timeout &> /dev/null || command -v gtimeout &> /dev/null; then
        local timeout_cmd="timeout"
        [ -z "$(command -v timeout)" ] && timeout_cmd="gtimeout"
        # Execute without invoking a shell (prevents command injection)
        cmd_output=$($timeout_cmd "$timeout_seconds" "$@" 2>&1)
        exit_code=$?

        # Check if timeout occurred
        if [ $exit_code -eq 124 ] || [ $exit_code -eq 143 ]; then
            log_debug "Command timed out after ${timeout_seconds}s"
            return 124
        fi
    else
        # Fallback: run without timeout (not ideal but better than nothing)
        cmd_output=$("$@" 2>&1)
        exit_code=$?
    fi

    # Check for common error patterns that indicate vault needs unlocking
    if [[ "$cmd_output" == *"Master password"* ]] || \
       [[ "$cmd_output" == *"Vault is locked"* ]] || \
       [[ "$cmd_output" == *"You are not logged in"* ]] || \
       [[ "$cmd_output" == *"Not authenticated"* ]] || \
       [[ "$cmd_output" == *"Invalid session"* ]]; then
        log_debug "Command requires authentication: ${cmd_output:0:100}"
        return 1
    fi

    echo "$cmd_output"
    return $exit_code
}

# Retrieve token from Bitwarden vault with timeout
get_token() {
    local item_name="$1"
    local session_key="$2"
    local token item_json
    local timeout_seconds=15
    
    log_debug "Retrieving token for item: $item_name"
    
    # Try direct password command first (simplest and fastest)
    log_debug "Trying direct password command..."
    token=$(bw_cmd "$timeout_seconds" bw get password "$item_name" --session "$session_key")
    local pw_exit_code=$?
    
    if [ $pw_exit_code -eq 0 ] && [ -n "$token" ] && [ "$token" != "null" ] && \
       [[ ! "$token" == *"error"* ]] && [[ ! "$token" == *"Error"* ]] && \
       [[ ! "$token" == *"not found"* ]] && [[ ! "$token" == *"Master password"* ]] && \
       [ ${#token} -gt 5 ]; then
        log_debug "Successfully retrieved via password command"
        echo "$token"
        return 0
    fi
    
    log_debug "Password command failed (exit: $pw_exit_code), trying get item..."
    
    # Get the item as JSON with timeout
    item_json=$(bw_cmd "$timeout_seconds" bw get item "$item_name" --session "$session_key")
    local exit_code=$?
    
    if [ $exit_code -ne 0 ] || [ -z "$item_json" ] || [ "$item_json" = "null" ] || [[ "$item_json" == *"error"* ]]; then
        log_debug "Failed to get item (exit code: $exit_code)"
        if [ "$DEBUG" = "1" ]; then
            log_debug "Item output preview: ${item_json:0:200}"
        fi
        echo ""
        return 1
    fi
    
    # Validate it's valid JSON
    if ! echo "$item_json" | jq empty 2>/dev/null; then
        log_debug "Invalid JSON returned from bw get item"
        echo ""
        return 1
    fi
    
    # Try to extract token from different possible locations
    # For Login items: .login.password
    # For Secure Notes: .notes
    # For Custom Fields: .fields[].value
    token=$(echo "$item_json" | jq -r '
        if .login.password and .login.password != "" and .login.password != null then .login.password
        elif .notes and .notes != "" and .notes != null then .notes
        elif .fields and (.fields | length) > 0 then 
            (.fields[]? | select((.name == "token" or .name == "password" or .name == "value" or .name == "Token") and .value != "" and .value != null) | .value) // empty
        else empty
        end' 2>/dev/null || echo "")
    
    # Final validation
    if [ -z "$token" ] || [ "$token" = "null" ] || [ ${#token} -le 5 ]; then
        log_debug "Token extraction failed for item: $item_name"
        if [ "$DEBUG" = "1" ]; then
            log_debug "Item JSON keys: $(echo "$item_json" | jq 'keys' 2>/dev/null || echo 'invalid json')"
            log_debug "Item type: $(echo "$item_json" | jq -r '.type // "unknown"' 2>/dev/null)"
        fi
        echo ""
        return 1
    fi
    
    log_debug "Successfully retrieved token for item: $item_name (length: ${#token})"
    echo "$token"
}

# Lock Bitwarden vault securely
lock_vault() {
    log_debug "Locking Bitwarden vault"
    # Lock vault and suppress output
    bw lock &>/dev/null || true
    # Clear any session-related environment variables
    unset BW_SESSION 2>/dev/null || true
}

# Cleanup function - ensures vault is locked even on error
cleanup() {
    lock_vault
    # Clear sensitive environment variables
    unset GIT_TOKEN GITHUB_TOKEN CARGO_REGISTRY_TOKEN 2>/dev/null || true
}

# Set up trap to lock vault on exit (including errors and interrupts)
trap cleanup EXIT INT TERM

# Verify release-plz is installed
check_release_plz_installed() {
    if ! command -v release-plz &> /dev/null; then
        log_error "release-plz is not installed."
        echo -e "${YELLOW}Install it with:${NC}" >&2
        echo "  cargo install --locked release-plz" >&2
        exit 1
    fi
    
    log_debug "release-plz is installed"
}

# Main function
main() {
    local command="${1:-}"
    
    if [ -z "$command" ]; then
        log_error "No command specified"
        echo -e "${YELLOW}Usage:${NC} $0 <release-pr|release>" >&2
        exit 1
    fi
    
    if [ "$command" != "release-pr" ] && [ "$command" != "release" ]; then
        log_error "Invalid command: $command"
        echo -e "${YELLOW}Valid commands:${NC} release-pr, release" >&2
        exit 1
    fi
    
    log_info "Running release-plz $command with Bitwarden tokens..."
    
    # Check prerequisites
    check_bw_installed
    check_release_plz_installed
    check_bw_logged_in
    
    # Unlock vault
    local session_key
    session_key=$(unlock_vault)
    
    if [ -z "$session_key" ] || [ ${#session_key} -le 10 ]; then
        log_error "Failed to obtain valid Bitwarden session key"
        echo -e "${YELLOW}Try unlocking manually: bw unlock${NC}" >&2
        exit 1
    fi
    
    log_debug "Session key obtained (length: ${#session_key})"
    
    # Verify session is valid by testing a simple command (skip if timeout not available)
    log_debug "Verifying session..."
    if command -v timeout &> /dev/null || command -v gtimeout &> /dev/null; then
        local timeout_cmd="timeout"
        [ -z "$(command -v timeout)" ] && timeout_cmd="gtimeout"
        local verify_output
        verify_output=$($timeout_cmd 5 bw list folders --session "$session_key" 2>&1)
        if [ $? -ne 0 ] || [[ "$verify_output" == *"error"* ]] || [[ "$verify_output" == *"Master password"* ]] || [[ "$verify_output" == *"Invalid session"* ]]; then
            log_warning "Session verification failed, but continuing anyway"
            if [ "$DEBUG" = "1" ]; then
                log_debug "Verify output: ${verify_output:0:100}"
            fi
        else
            log_debug "Session verified"
        fi
    else
        log_debug "Skipping session verification (timeout command not available)"
    fi
    
    # Retrieve tokens
    log_info "Retrieving tokens from Bitwarden..."
    log_debug "Looking for GitHub token item: $GITHUB_TOKEN_ITEM"
    
    # First, list available items for debugging (only in debug mode to avoid hanging)
    if [ "$DEBUG" = "1" ]; then
        log_debug "Available items in vault:"
        local items_list
        items_list=$(bw_cmd 5 bw list items --session "$session_key")
        if [ $? -eq 0 ] && [ -n "$items_list" ]; then
            echo "$items_list" | jq -r '.[].name' 2>/dev/null | head -10 || log_debug "Could not parse items list"
        else
            log_debug "Could not list items"
        fi
    fi
    
    local github_token
    log_info "Retrieving GITHUB_TOKEN..."
    github_token=$(get_token "$GITHUB_TOKEN_ITEM" "$session_key")
    
    if [ -z "$github_token" ] || [ ${#github_token} -le 5 ]; then
        log_error "Failed to retrieve GITHUB_TOKEN from Bitwarden item: $GITHUB_TOKEN_ITEM"
        echo "" >&2
        echo -e "${YELLOW}Troubleshooting steps:${NC}" >&2
        echo "  1. List your items: bw list items --session <session>" >&2
        echo "  2. Verify item name matches exactly (case-sensitive): '$GITHUB_TOKEN_ITEM'" >&2
        echo "  3. Check item type - should be Login (password field) or Secure Note (notes field)" >&2
        echo "  4. Test manually: bw get item '$GITHUB_TOKEN_ITEM' --session <session>" >&2
        echo "  5. Customize item name: export BW_GITHUB_TOKEN_ITEM=your-item-name" >&2
        echo "" >&2
        echo -e "${BLUE}To enable debug mode:${NC} BW_DEBUG=1 make release-pr-bw" >&2
        exit 1
    fi
    
    # Export tokens (release-plz expects GIT_TOKEN, not GITHUB_TOKEN)
    export GIT_TOKEN="$github_token"
    # Also export GITHUB_TOKEN for compatibility
    export GITHUB_TOKEN="$github_token"
    log_success "Retrieved GITHUB_TOKEN"
    
    # Optionally retrieve CARGO_REGISTRY_TOKEN
    local cargo_token
    cargo_token=$(get_token "$CARGO_TOKEN_ITEM" "$session_key")
    
    if [ -n "$cargo_token" ] && [ ${#cargo_token} -gt 5 ]; then
        export CARGO_REGISTRY_TOKEN="$cargo_token"
        log_success "Retrieved CARGO_REGISTRY_TOKEN"
    else
        log_warning "CARGO_REGISTRY_TOKEN not found (optional, only needed if not using trusted publishing)"
    fi
    
    # Change to project root
    cd "$PROJECT_ROOT"
    log_debug "Changed to project root: $PROJECT_ROOT"
    
    # Run release-plz command
    # Avoid passing secrets on the command line; release-plz will read GIT_TOKEN from env.
    log_info "Running: release-plz $command"
    
    if release-plz "$command"; then
        log_success "Release-plz completed successfully"
    else
        local exit_code=$?
        log_error "Release-plz failed with exit code: $exit_code"
        exit $exit_code
    fi
}

# Run main function with all arguments
main "$@"
