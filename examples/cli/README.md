# Sentinel Arc CLI — Usage Examples

This directory contains practical examples demonstrating Sentinel Arc CLI workflows.

## Example 1: First-time Setup

```bash
# Navigate to your project
cd my-rust-project

# Initialize Sentinel Arc workspace
sentinel-cli init

# Verify setup
sentinel-cli doctor

# Scan the project
sentinel-cli scan .

# View workspace stats
sentinel-cli stats
```

## Example 2: Exploring Code Architecture

```bash
# Search for a function
sentinel-cli search "parse_config"

# Visualize its dependency tree
sentinel-cli graph "parse_config"

# Run architectural validation
sentinel-cli validate
```

## Example 3: LLM Context Generation

```bash
# Generate a context package for a refactoring task
sentinel-cli context "refactor the authentication module"

# JSON output for piping to another tool
sentinel-cli context "add error handling" --json | jq '.nodes | length'
```

## Example 4: CI/CD Integration

```bash
# In your CI pipeline
sentinel-cli init
sentinel-cli scan .

# Fail the build if validation finds issues (exit code 1)
sentinel-cli validate

# Search in JSON for automated processing
sentinel-cli search "TODO" --json --limit 100
```

## Example 5: Search Index Recovery

```bash
# If the search index is corrupted or out of sync
sentinel-cli rebuild-index

# Verify it works
sentinel-cli search "main"
```

## Example 6: Shell Completions

```bash
# Bash
sentinel-cli completion bash > ~/.local/share/bash-completion/completions/sentinel-cli

# Zsh (add ~/.zfunc to fpath in .zshrc)
sentinel-cli completion zsh > ~/.zfunc/_sentinel-cli

# Fish
sentinel-cli completion fish > ~/.config/fish/completions/sentinel-cli.fish

# PowerShell
sentinel-cli completion powershell >> $PROFILE
```
