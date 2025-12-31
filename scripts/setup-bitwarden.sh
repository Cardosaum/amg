#!/usr/bin/env bash
# setup-bitwarden.sh
# Helper script to set up Bitwarden CLI for use with release-plz
#
# Usage:
#   ./scripts/setup-bitwarden.sh
#
# This script:
# 1. Checks if Bitwarden CLI is installed
# 2. Guides user through login process
# 3. Tests vault access
# 4. Provides instructions for creating vault items

set -euo pipefail

# Colors for output
BLUE='\033[0;34m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Bitwarden vault item names
GITHUB_TOKEN_ITEM="${BW_GITHUB_TOKEN_ITEM:-amg-release-github-token}"
CARGO_TOKEN_ITEM="${BW_CARGO_TOKEN_ITEM:-amg-release-cargo-token}"

# Check if Bitwarden CLI is installed
check_bw_installed() {
    if ! command -v bw &> /dev/null; then
        echo -e "${RED}Error: Bitwarden CLI (bw) is not installed.${NC}" >&2
        echo "" >&2
        echo -e "${BLUE}Installation options:${NC}" >&2
        echo "" >&2
        echo -e "${YELLOW}macOS (Homebrew):${NC}" >&2
        echo "  brew install bitwarden-cli" >&2
        echo "" >&2
        echo -e "${YELLOW}Linux:${NC}" >&2
        echo "  See: https://bitwarden.com/help/cli/#download-and-install" >&2
        echo "" >&2
        echo -e "${YELLOW}Windows:${NC}" >&2
        echo "  See: https://bitwarden.com/help/cli/#download-and-install" >&2
        echo "" >&2
        exit 1
    fi

    if ! command -v jq &> /dev/null; then
        echo -e "${RED}Error: jq is not installed (required for JSON parsing).${NC}" >&2
        echo "" >&2
        echo -e "${BLUE}Installation options:${NC}" >&2
        echo "" >&2
        echo -e "${YELLOW}macOS (Homebrew):${NC}" >&2
        echo "  brew install jq" >&2
        echo "" >&2
        echo -e "${YELLOW}Debian/Ubuntu:${NC}" >&2
        echo "  sudo apt-get install -y jq" >&2
        echo "" >&2
        exit 1
    fi

    echo -e "${GREEN}✓ Bitwarden CLI and jq are installed${NC}"
}

# Check if user is logged in
check_login() {
    local status
    status=$(bw status | jq -r '.status' 2>/dev/null || echo "not_logged_in")
    
    if [ "$status" = "authenticated" ] || [ "$status" = "unlocked" ]; then
        echo -e "${GREEN}✓ Already logged in to Bitwarden${NC}"
        return 0
    fi
    
    return 1
}

# Guide user through login
login_guide() {
    echo -e "${BLUE}Setting up Bitwarden CLI login...${NC}"
    echo ""
    echo -e "${YELLOW}You will need:${NC}"
    echo "  - Your Bitwarden email"
    echo "  - Your Bitwarden master password"
    echo ""
    echo -e "${BLUE}Running: bw login${NC}"
    echo ""
    
    if bw login; then
        echo ""
        echo -e "${GREEN}✓ Successfully logged in${NC}"
    else
        echo ""
        echo -e "${RED}✗ Login failed${NC}"
        exit 1
    fi
}

# Test vault access
test_vault_access() {
    echo ""
    echo -e "${BLUE}Testing vault access...${NC}"
    
    local session_key
    session_key=$(bw unlock --raw 2>/dev/null || echo "")
    
    if [ -z "$session_key" ]; then
        echo -e "${YELLOW}Vault is locked. Attempting to unlock...${NC}"
        session_key=$(bw unlock --raw)
    fi
    
    if [ -n "$session_key" ]; then
        echo -e "${GREEN}✓ Vault unlocked successfully${NC}"
        
        # Test listing items
        if bw list items --session "$session_key" &>/dev/null; then
            echo -e "${GREEN}✓ Can access vault items${NC}"
        else
            echo -e "${YELLOW}⚠ Warning: Could not list vault items${NC}"
        fi
        
        # Lock vault after test
        bw lock &>/dev/null || true
    else
        echo -e "${RED}✗ Failed to unlock vault${NC}"
        exit 1
    fi
}

# Print instructions for creating vault items
print_vault_item_instructions() {
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}Next Steps: Create Vault Items${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo -e "${YELLOW}You need to create the following items in your Bitwarden vault:${NC}"
    echo ""
    echo -e "${GREEN}1. GitHub Token${NC}"
    echo "   Item Name: ${GITHUB_TOKEN_ITEM}"
    echo "   Type: Secure Note or Login"
    echo "   Field: Store your GitHub Personal Access Token"
    echo "   Scopes needed: repo, workflow"
    echo ""
    echo "   Create token at: https://github.com/settings/tokens"
    echo ""
    echo -e "${GREEN}2. Cargo Registry Token (Optional)${NC}"
    echo "   Item Name: ${CARGO_TOKEN_ITEM}"
    echo "   Type: Secure Note or Login"
    echo "   Field: Store your crates.io token"
    echo ""
    echo "   Only needed if not using trusted publishing"
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo -e "${YELLOW}To customize item names, set environment variables:${NC}"
    echo "  export BW_GITHUB_TOKEN_ITEM=your-github-token-item"
    echo "  export BW_CARGO_TOKEN_ITEM=your-cargo-token-item"
    echo ""
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    echo -e "${GREEN}After creating the vault items, you can use:${NC}"
    echo "  make release-pr-bw    # Run release-pr with Bitwarden tokens"
    echo "  make release-bw       # Run release with Bitwarden tokens"
    echo ""
    echo "Or use the script directly:"
    echo "  ./scripts/release-with-bitwarden.sh release-pr"
    echo "  ./scripts/release-with-bitwarden.sh release"
    echo ""
}

# Main function
main() {
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo -e "${BLUE}Bitwarden CLI Setup for Release-plz${NC}"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
    echo ""
    
    # Check installation
    check_bw_installed
    
    # Check login status
    if ! check_login; then
        login_guide
    fi
    
    # Test vault access
    test_vault_access
    
    # Print instructions
    print_vault_item_instructions
    
    echo -e "${GREEN}✓ Setup complete!${NC}"
}

# Run main function
main "$@"

