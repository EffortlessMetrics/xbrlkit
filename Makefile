# xbrlkit Maintainer Wrappers
# Issue #4: Quick gate and full gate shortcuts

.PHONY: help quick full

# Default target shows help
help:
	@echo "xbrlkit Maintainer Commands:"
	@echo "  make quick    Run quality gates (fmt, clippy, test) - fast feedback"
	@echo "  make full     Run full alpha gate validation"
	@echo ""
	@echo "For detailed commands, see CONTRIBUTING.md"

# Quick gate: format, clippy, test (fast feedback)
quick:
	@echo "=== Running quick quality gates ==="
	cargo fmt --check
	cargo clippy --workspace -- -D warnings
	cargo test --workspace
	@echo "=== Quick gate passed ==="

# Full gate: alpha-check (complete validation)
full:
	@echo "=== Running full alpha gate ==="
	cargo xtask alpha-check
	@echo "=== Full gate passed ==="
