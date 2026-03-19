#!/bin/bash
# Workflow health check for xbrlkit
# Reports PRs stuck in states, missing labels, etc.

set -e

REPO="EffortlessMetrics/xbrlkit"
TOKEN="${GITHUB_TOKEN:-${GITHUB_PAT}}"

if [ -z "$TOKEN" ]; then
    echo "Error: GITHUB_TOKEN or GITHUB_PAT required"
    exit 1
fi

echo "=== Workflow Health Check ==="
echo "Repo: $REPO"
echo "Time: $(date -Iseconds)"
echo ""

# Fetch open PRs with labels
echo "Fetching open PRs..."
PRS=$(curl -s -H "Authorization: Bearer $TOKEN" \
    -H "Accept: application/vnd.github.v3+json" \
    "https://api.github.com/repos/$REPO/pulls?state=open&per_page=100")

# Check for PRs without status labels
echo ""
echo "=== PRs Without Status Labels ==="
echo "$PRS" | jq -r '.[] | select(.labels | map(.name) | map(select(startswith("status/"))) | length == 0) | "PR #\(.number): \(.title)"'

# Check for stale PRs (>7 days without update)
echo ""
echo "=== Stale PRs (>7 days since update) ==="
SEVEN_DAYS_AGO=$(date -d '7 days ago' -Iseconds)
echo "$PRS" | jq -r --arg date "$SEVEN_DAYS_AGO" \
    '.[] | select(.updated_at < $date) | "PR #\(.number): \(.title) (updated: \(.updated_at))"'

# Check for stuck PRs by state
echo ""
echo "=== PRs by Status Label ==="
for status in "implementing" "reviewed-needs-work" "needs-review" "ready-for-review" "ready-to-merge" "blocked-on-X"; do
    COUNT=$(echo "$PRS" | jq -r "[.[] | select(.labels | map(.name) | contains([\"status/$status\"]))] | length")
    if [ "$COUNT" -gt 0 ]; then
        echo ""
        echo "status/$status ($COUNT):"
        echo "$PRS" | jq -r ".[] | select(.labels | map(.name) | contains([\"status/$status\"])) | \"  #\(.number): \(.title)\""
    fi
done

# Suggest actions
echo ""
echo "=== Suggested Actions ==="

STALE_COUNT=$(echo "$PRS" | jq -r --arg date "$SEVEN_DAYS_AGO" '[.[] | select(.updated_at < $date)] | length')
if [ "$STALE_COUNT" -gt 0 ]; then
    echo "- Review $STALE_COUNT stale PR(s) — ping authors or close"
fi

NO_LABEL_COUNT=$(echo "$PRS" | jq -r '[.[] | select(.labels | map(.name) | map(select(startswith("status/"))) | length == 0)] | length')
if [ "$NO_LABEL_COUNT" -gt 0 ]; then
    echo "- Label $NO_LABEL_COUNT PR(s) with status/* labels"
fi

BLOCKED_COUNT=$(echo "$PRS" | jq -r '[.[] | select(.labels | map(.name) | contains(["status/blocked-on-X"]))] | length')
if [ "$BLOCKED_COUNT" -gt 0 ]; then
    echo "- Check $BLOCKED_COUNT blocked PR(s) — dependencies resolved?"
fi

READY_COUNT=$(echo "$PRS" | jq -r '[.[] | select(.labels | map(.name) | contains(["status/ready-to-merge"]))] | length')
if [ "$READY_COUNT" -gt 0 ]; then
    echo "- Merge $READY_COUNT ready PR(s)"
fi

echo ""
echo "=== End Workflow Health Check ==="
