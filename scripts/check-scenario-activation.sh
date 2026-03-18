#!/bin/bash
# Pre-push checklist for scenario activation PRs
# Usage: ./scripts/check-scenario-activation.sh SCN-XK-EXPORT-001

set -e

SCENARIO_ID="${1:-}"
AC_ID="${2:-}"

if [ -z "$SCENARIO_ID" ] || [ -z "$AC_ID" ]; then
    echo "Usage: $0 <scenario-id> <ac-id>"
    echo "Example: $0 SCN-XK-EXPORT-001 AC-XK-EXPORT-001"
    exit 1
fi

echo "🔍 Checking scenario activation for $SCENARIO_ID..."

# Check 1: Feature file has @alpha-active
echo -n "□ @alpha-active tag in feature file... "
if grep -r "@alpha-active" specs/features --include="*.feature" | grep -q "$SCENARIO_ID"; then
    echo "✅"
else
    echo "❌ MISSING"
    exit 1
fi

# Check 2: AC in ACTIVE_ALPHA_ACS
echo -n "□ AC in ACTIVE_ALPHA_ACS... "
if grep -q "$AC_ID" xtask/src/alpha_check.rs; then
    echo "✅"
else
    echo "❌ MISSING"
    exit 1
fi

# Check 3: Assertion in assert_scenario_outcome
echo -n "□ Assertion in assert_scenario_outcome... "
if grep -q "$AC_ID" crates/scenario-runner/src/lib.rs; then
    echo "✅"
else
    echo "❌ MISSING"
    exit 1
fi

# Check 4: Tests pass
echo -n "□ cargo test passes... "
if cargo test --workspace --quiet 2>/dev/null; then
    echo "✅"
else
    echo "❌ FAILED"
    exit 1
fi

echo ""
echo "✅ All checks passed! Ready to push."
