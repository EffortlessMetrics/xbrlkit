#!/bin/bash
# Update queue state after PR merge
# Usage: ./scripts/update-queue.sh <issue_number>

ISSUE=$1
if [ -z "$ISSUE" ]; then
    echo "Usage: $0 <issue_number>"
    exit 1
fi

QUEUE_FILE=".mend/pr-queue.md"

# Move issue from "Current Queue" to "Completed" section
# Update stage to ✅ Complete
# Update timestamp

sed -i "s/| # | $ISSUE |.*$/| $ISSUE | ✅ Complete | $(date +%H:%M) |/" "$QUEUE_FILE"

echo "Queue updated: Issue #$ISSUE marked complete"
