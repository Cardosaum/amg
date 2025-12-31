.PHONY: help build build-release test test-nextest lint lint-fix fmt fmt-check check ci clean install-deps install install-custom

# Default target
.DEFAULT_GOAL := help

# Colors for output
BLUE := \033[0;34m
GREEN := \033[0;32m
YELLOW := \033[0;33m
RED := \033[0;31m
NC := \033[0m # No Color

# Binary name
BINARY := amg

help: ## Show this help message
	@echo "$(BLUE)Available targets:$(NC)"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "  $(GREEN)%-20s$(NC) %s\n", $$1, $$2}'

install-deps: ## Install required development dependencies
	@echo "$(BLUE)Installing development dependencies...$(NC)"
	@cargo install cargo-edit cargo-nextest --quiet 2>/dev/null || true
	@echo "$(GREEN)✓ Dependencies installed$(NC)"

build: ## Build the project in debug mode
	@echo "$(BLUE)Building project...$(NC)"
	@cargo build
	@echo "$(GREEN)✓ Build complete$(NC)"

build-release: ## Build the project in release mode
	@echo "$(BLUE)Building project (release)...$(NC)"
	@cargo build --release
	@echo "$(GREEN)✓ Release build complete$(NC)"

test: ## Run tests with cargo test
	@echo "$(BLUE)Running tests...$(NC)"
	@cargo test --all-targets
	@echo "$(GREEN)✓ Tests passed$(NC)"

test-nextest: ## Run tests with nextest (faster, parallel)
	@echo "$(BLUE)Running tests with nextest...$(NC)"
	@cargo nextest run --all-targets
	@echo "$(GREEN)✓ Tests passed$(NC)"

lint: ## Run clippy linter
	@echo "$(BLUE)Running clippy...$(NC)"
	@cargo clippy --all-targets --all-features -- -D warnings
	@echo "$(GREEN)✓ Linting passed$(NC)"

lint-fix: ## Run clippy and automatically fix issues
	@echo "$(BLUE)Running clippy --fix...$(NC)"
	@cargo clippy --fix --all-targets --all-features --allow-dirty --allow-staged
	@echo "$(GREEN)✓ Linting fixes applied$(NC)"

fmt: ## Format code with rustfmt
	@echo "$(BLUE)Formatting code...$(NC)"
	@cargo fmt
	@echo "$(GREEN)✓ Code formatted$(NC)"

fmt-check: ## Check code formatting without modifying files
	@echo "$(BLUE)Checking code formatting...$(NC)"
	@cargo fmt --all -- --check || (echo "$(RED)Formatting check failed! Run 'make fmt' to fix.$(NC)" && exit 1)
	@echo "$(GREEN)✓ Formatting check passed$(NC)"

check: fmt-check ## Run cargo check (compile without building, includes format check)
	@echo "$(BLUE)Running cargo check...$(NC)"
	@cargo check --all-targets
	@echo "$(GREEN)✓ Check passed$(NC)"

ci: ## Run all CI checks (fmt-check, lint, check, test-nextest)
	@echo "$(BLUE)Running CI pipeline...$(NC)"
	@$(MAKE) fmt-check
	@$(MAKE) lint
	@$(MAKE) check
	@$(MAKE) test-nextest
	@echo "$(GREEN)✓ All CI checks passed$(NC)"

all: fmt-check lint check test-nextest build-release ## Run all checks and build release (ensures code quality)
	@echo "$(GREEN)✓ All checks passed and release build complete$(NC)"

clean: ## Clean build artifacts
	@echo "$(BLUE)Cleaning build artifacts...$(NC)"
	@cargo clean
	@echo "$(GREEN)✓ Clean complete$(NC)"

update: ## Update all dependencies to latest versions
	@echo "$(BLUE)Updating dependencies...$(NC)"
	@cargo upgrade --incompatible
	@echo "$(GREEN)✓ Dependencies updated$(NC)"

install: ## Install the binary to $HOME/.cargo/bin (default Cargo installation directory)
	@echo "$(BLUE)Installing $(BINARY) to $$HOME/.cargo/bin...$(NC)"
	@cargo install --path .
	@echo "$(GREEN)✓ $(BINARY) installed successfully$(NC)"
	@echo "$(YELLOW)Note: Ensure $$HOME/.cargo/bin is in your PATH$(NC)"

install-custom: ## Install the binary to a custom directory (usage: make install-custom ROOT=/path/to/root)
	@if [ -z "$(ROOT)" ]; then \
		echo "$(RED)Error: ROOT variable must be set. Example: make install-custom ROOT=$$HOME/.local$(NC)"; \
		exit 1; \
	fi
	@echo "$(BLUE)Installing $(BINARY) to $(ROOT)/bin...$(NC)"
	@cargo install --path . --root $(ROOT)
	@echo "$(GREEN)✓ $(BINARY) installed successfully to $(ROOT)/bin$(NC)"
	@echo "$(YELLOW)Note: Ensure $(ROOT)/bin is in your PATH$(NC)"

