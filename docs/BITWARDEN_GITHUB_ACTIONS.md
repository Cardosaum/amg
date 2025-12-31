# Bitwarden CLI Integration with GitHub Actions

This guide explains how to optionally integrate Bitwarden CLI into GitHub Actions workflows for centralized secret management.

## Overview

While GitHub Actions has built-in secrets management, you can optionally use Bitwarden CLI for:
- Centralized secret management across multiple CI/CD platforms
- Single source of truth for secrets
- Easier secret rotation
- Team-wide secret sharing via Bitwarden organizations

## When to Use

**Use GitHub Secrets if:**
- You only use GitHub Actions
- You want simplicity and native integration
- You don't need centralized secret management

**Use Bitwarden CLI if:**
- You use multiple CI/CD platforms
- You want centralized secret management
- You need team-wide secret sharing
- You prefer Bitwarden's secret management features

## Setup

### 1. Create Bitwarden API Key

1. Log in to Bitwarden web vault
2. Go to Settings → Security → API Keys
3. Create a new API key:
   - **Name**: "GitHub Actions - amg"
   - **Master Password**: Enter your master password
   - Copy the `Client ID` and `Client Secret`

### 2. Store API Key in GitHub Secrets

1. Go to your repository → Settings → Secrets and variables → Actions
2. Add the following secrets:
   - `BW_CLIENTID`: Your Bitwarden Client ID
   - `BW_CLIENTSECRET`: Your Bitwarden Client Secret
   - Optionally: `BW_ORGANIZATION_ID`: If using organization vault

### 3. Update Workflow

Add Bitwarden CLI steps before release-plz steps:

```yaml
- name: Install Bitwarden CLI
  run: |
    if [ "$RUNNER_OS" == "Linux" ]; then
      wget -qO- https://vault.bitwarden.com/download/?app=cli&platform=linux | tar -xzf - -C /usr/local/bin
    elif [ "$RUNNER_OS" == "macOS" ]; then
      brew install bitwarden-cli
    elif [ "$RUNNER_OS" == "Windows" ]; then
      # Download Windows binary
      Invoke-WebRequest -Uri "https://github.com/bitwarden/cli/releases/latest/download/bw-windows.zip" -OutFile "bw.zip"
      Expand-Archive -Path "bw.zip" -DestinationPath "$env:PATH"
    fi
    bw --version

- name: Authenticate with Bitwarden
  run: |
    bw config server https://vault.bitwarden.com
    export BW_CLIENTID="${{ secrets.BW_CLIENTID }}"
    export BW_CLIENTSECRET="${{ secrets.BW_CLIENTSECRET }}"
    bw login --apikey
    export BW_SESSION=$(bw unlock --passwordenv BW_CLIENTSECRET --raw)

- name: Retrieve tokens from Bitwarden
  id: bitwarden-tokens
  run: |
    GITHUB_TOKEN=$(bw get item amg-release-github-token --session $BW_SESSION | jq -r '.login.password // .notes')
    echo "::add-mask::$GITHUB_TOKEN"
    echo "GITHUB_TOKEN=$GITHUB_TOKEN" >> $GITHUB_ENV
    
    # Optional: Retrieve Cargo token
    CARGO_TOKEN=$(bw get item amg-release-cargo-token --session $BW_SESSION 2>/dev/null | jq -r '.login.password // .notes // empty')
    if [ -n "$CARGO_TOKEN" ] && [ "$CARGO_TOKEN" != "null" ]; then
      echo "::add-mask::$CARGO_TOKEN"
      echo "CARGO_REGISTRY_TOKEN=$CARGO_TOKEN" >> $GITHUB_ENV
    fi
    
    bw lock

- name: Run release-plz
  uses: release-plz/action@v0.5
  with:
    command: release-pr
  env:
    GITHUB_TOKEN: ${{ env.GITHUB_TOKEN }}
    CARGO_REGISTRY_TOKEN: ${{ env.CARGO_REGISTRY_TOKEN }}
```

## Complete Example

Here's a complete example for the `release-plz-pr` job:

```yaml
release-plz-pr:
  name: Release-plz PR
  runs-on: ubuntu-latest
  if: github.repository_owner == 'Cardosaum'
  permissions:
    contents: write
    pull-requests: write
  concurrency:
    group: release-plz-${{ github.ref }}
    cancel-in-progress: false
  steps:
    - name: Checkout repository
      uses: actions/checkout@v6
      with:
        fetch-depth: 0
        persist-credentials: false

    - name: Install Rust toolchain
      uses: dtolnay/rust-toolchain@stable

    - name: Install Bitwarden CLI
      run: |
        wget -qO- https://vault.bitwarden.com/download/?app=cli&platform=linux | tar -xzf - -C /usr/local/bin
        bw --version

    - name: Install jq
      run: sudo apt-get update && sudo apt-get install -y jq

    - name: Authenticate with Bitwarden
      env:
        BW_CLIENTID: ${{ secrets.BW_CLIENTID }}
        BW_CLIENTSECRET: ${{ secrets.BW_CLIENTSECRET }}
      run: |
        bw config server https://vault.bitwarden.com
        bw login --apikey
        export BW_SESSION=$(bw unlock --passwordenv BW_CLIENTSECRET --raw)
        echo "BW_SESSION=$BW_SESSION" >> $GITHUB_ENV

    - name: Retrieve tokens from Bitwarden
      run: |
        GITHUB_TOKEN=$(bw get item amg-release-github-token --session $BW_SESSION | jq -r '.login.password // .notes')
        echo "::add-mask::$GITHUB_TOKEN"
        echo "GITHUB_TOKEN=$GITHUB_TOKEN" >> $GITHUB_ENV
        
        CARGO_TOKEN=$(bw get item amg-release-cargo-token --session $BW_SESSION 2>/dev/null | jq -r '.login.password // .notes // empty')
        if [ -n "$CARGO_TOKEN" ] && [ "$CARGO_TOKEN" != "null" ]; then
          echo "::add-mask::$CARGO_TOKEN"
          echo "CARGO_REGISTRY_TOKEN=$CARGO_TOKEN" >> $GITHUB_ENV
        fi
        
        bw lock

    - name: Run release-plz
      uses: release-plz/action@v0.5
      with:
        command: release-pr
      env:
        GITHUB_TOKEN: ${{ env.GITHUB_TOKEN }}
        CARGO_REGISTRY_TOKEN: ${{ env.CARGO_REGISTRY_TOKEN }}
```

## Security Considerations

1. **API Key Security**: Store Bitwarden API keys as GitHub Secrets, never in code
2. **Token Masking**: Use `::add-mask::` to prevent tokens from appearing in logs
3. **Vault Locking**: Always lock the vault after retrieving tokens
4. **Session Management**: Use environment variables for session keys, clean up after use
5. **Organization Vaults**: Use Bitwarden organization vaults for team projects
6. **Access Control**: Limit API key access to only necessary vault items

## Troubleshooting

### Authentication Fails

- Verify API key is correct
- Check if API key has necessary permissions
- Ensure Bitwarden server URL is correct

### Token Retrieval Fails

- Verify vault item names match
- Check if item exists and is accessible
- Ensure session is valid (unlock vault)

### jq Not Found

Install jq in the workflow:
```yaml
- name: Install jq
  run: sudo apt-get update && sudo apt-get install -y jq
```

## Alternative: GitHub Actions Secrets

For most use cases, GitHub Actions built-in secrets are simpler:

```yaml
env:
  GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
  CARGO_REGISTRY_TOKEN: ${{ secrets.CARGO_REGISTRY_TOKEN }}
```

This is the recommended approach unless you specifically need centralized secret management.

## References

- [Bitwarden CLI Documentation](https://bitwarden.com/help/cli/)
- [GitHub Actions Secrets](https://docs.github.com/en/actions/security-guides/encrypted-secrets)
- [Bitwarden API Keys](https://bitwarden.com/help/article/personal-api-key/)

