# Bitwarden CLI Setup for Release-plz

This guide explains how to set up Bitwarden CLI to securely manage tokens required by release-plz for local development.

> Note: This repository intentionally does **not** integrate Bitwarden into GitHub Actions. CI releases use GitHub-native auth (the workflow `GITHUB_TOKEN` plus crates.io trusted publishing via OIDC).

## Overview

Bitwarden CLI integration allows you to:
- Securely store GitHub and Cargo registry tokens in your Bitwarden vault
- Automatically retrieve tokens when running release-plz commands
- Avoid storing tokens in plain text files or environment variables
- Maintain security while enabling automation

## Prerequisites

- Bitwarden account (free or paid)
- Bitwarden CLI (`bw`) installed
- `jq` installed (for JSON parsing, usually pre-installed on macOS/Linux)

## Installation

### macOS (Homebrew)

```bash
brew install bitwarden-cli
```

### Linux

Download from [Bitwarden CLI releases](https://github.com/bitwarden/cli/releases) or use your distribution's package manager.

### Windows

Download the Windows executable from [Bitwarden CLI releases](https://github.com/bitwarden/cli/releases).

### Verify Installation

```bash
bw --version
```

## Quick Setup

Run the setup script to guide you through the process:

```bash
make setup-bitwarden
# or
./scripts/setup-bitwarden.sh
```

## Manual Setup

### 1. Install Bitwarden CLI

See installation instructions above.

### 2. Log in to Bitwarden

```bash
bw login
```

Enter your Bitwarden email and master password when prompted.

### 3. Create Vault Items

You need to create items in your Bitwarden vault to store tokens.

#### GitHub Token Item

1. Open Bitwarden (web app, desktop app, or CLI)
2. Create a new item:
   - **Name**: `amg-release-github-token` (or customize with `BW_GITHUB_TOKEN_ITEM`)
   - **Type**: Secure Note or Login
   - **Field**: Store your GitHub Personal Access Token

**Creating a GitHub Personal Access Token:**

1. Go to https://github.com/settings/tokens
2. Click "Generate new token" → "Generate new token (classic)"
3. Set expiration and scopes:
   - **Scopes needed**: `repo`, `workflow`
   - **Note**: "amg release-plz"
4. Copy the token and store it in Bitwarden

#### Cargo Registry Token Item (Optional)

Only needed if you're not using trusted publishing on crates.io **or** you want to run `release-plz release` locally.

If you publish from GitHub Actions using crates.io trusted publishing (OIDC), you typically do **not** need a crates.io token in CI.

1. Create a new item:
   - **Name**: `amg-release-cargo-token` (or customize with `BW_CARGO_TOKEN_ITEM`)
   - **Type**: Secure Note or Login
   - **Field**: Store your crates.io token

**Getting a crates.io token:**

1. Go to https://crates.io/settings/tokens
2. Create a new token with `publish-new` and `publish-update` scopes
3. Copy the token and store it in Bitwarden

### 4. Test Setup

```bash
# Unlock vault (will prompt for password)
export BW_SESSION=$(bw unlock --raw)

# Test retrieving GitHub token
bw get item amg-release-github-token --session $BW_SESSION | jq -r '.login.password // .notes'

# Lock vault
bw lock
```

## Usage

### Using Makefile Targets

```bash
# Run release-pr with Bitwarden tokens
make release-pr-bw

# Run release with Bitwarden tokens
make release-bw
```

### Using Scripts Directly

```bash
# Run release-pr
./scripts/release-with-bitwarden.sh release-pr

# Run release
./scripts/release-with-bitwarden.sh release
```

### Customizing Vault Item Names

If your vault items have different names, set environment variables:

```bash
export BW_GITHUB_TOKEN_ITEM=your-github-token-item
export BW_CARGO_TOKEN_ITEM=your-cargo-token-item

make release-pr-bw
```

## How It Works

1. **Script checks prerequisites**: Verifies Bitwarden CLI is installed and user is logged in
2. **Unlocks vault**: Prompts for master password if vault is locked
3. **Retrieves tokens**: Fetches tokens from Bitwarden vault items
4. **Exports tokens**: Sets `GITHUB_TOKEN` and optionally `CARGO_REGISTRY_TOKEN` environment variables
5. **Runs release-plz**: Executes the release-plz command with tokens
6. **Locks vault**: Automatically locks vault after use for security

## Security Best Practices

1. **Never commit tokens**: Tokens should only exist in Bitwarden vault, never in git
2. **Use strong master password**: Protect your Bitwarden account with a strong password
3. **Enable 2FA**: Enable two-factor authentication on your Bitwarden account
4. **Lock vault after use**: The script automatically locks the vault, but you can manually lock with `bw lock`
5. **Rotate tokens regularly**: Periodically rotate your GitHub and Cargo tokens
6. **Use organization vaults**: For team projects, use Bitwarden organization vaults
7. **Avoid `BW_PASSWORD`**: Prefer `export BW_SESSION=$(bw unlock --raw)` in an interactive terminal instead of putting your Bitwarden master password in an environment variable.

## Troubleshooting

### "Bitwarden CLI (bw) is not installed"

Install Bitwarden CLI using the instructions above.

### "Not logged in to Bitwarden"

```bash
bw login
```

### "Failed to unlock Bitwarden vault"

- Make sure you're using the correct master password
- Check if your Bitwarden account is locked
- Try logging out and logging back in: `bw logout && bw login`

### "Failed to retrieve GITHUB_TOKEN from Bitwarden"

- Verify the vault item exists: `bw list items | grep amg-release-github-token`
- Check the item name matches (default: `amg-release-github-token`)
- Ensure the token is stored in the password field or notes field
- Try customizing the item name: `export BW_GITHUB_TOKEN_ITEM=your-item-name`

### "Vault is locked" errors

The script should automatically unlock the vault, but if you encounter issues:

```bash
# Manually unlock
bw unlock

# Or unlock and get session key
export BW_SESSION=$(bw unlock --raw)
```

### Session expired

Bitwarden sessions expire after a period of inactivity. The script will prompt you to unlock again.

### jq not found

Install `jq`:
- macOS: `brew install jq`
- Linux: Use your package manager (`apt install jq`, `yum install jq`, etc.)

## Advanced Usage

### Using Bitwarden API Keys (for automation)

For automated scripts, you can use Bitwarden API keys:

```bash
# Login with API key
bw login --apikey

# Set environment variables
export BW_CLIENTID=your-client-id
export BW_CLIENTSECRET=your-client-secret
```

**Note**: Store API keys securely (never in the repository).
This repository does not use Bitwarden in CI; prefer GitHub-native secrets/OIDC for GitHub Actions.

### Integration with Shell Config

Add to your `~/.zshrc` or `~/.bashrc`:

```bash
# Quick unlock function
bw-unlock() {
    export BW_SESSION=$(bw unlock --raw)
}

# Quick lock function
bw-lock() {
    bw lock
    unset BW_SESSION
}
```

Then use:
```bash
bw-unlock
make release-pr-bw
bw-lock
```

## References

- [Bitwarden CLI Documentation](https://bitwarden.com/help/cli/)
- [GitHub Personal Access Tokens](https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens)
- [Crates.io Token Management](https://doc.rust-lang.org/cargo/reference/publishing.html)
- [Release-plz Documentation](https://release-plz.dev/docs)

