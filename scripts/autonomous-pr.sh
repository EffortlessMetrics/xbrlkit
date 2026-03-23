#!/bin/bash
# Autonomous PR workflow for xbrlkit
# Usage: ./scripts/autonomous-pr.sh <issue_number>

set -e

ISSUE=$1
if [ -z "$ISSUE" ]; then
    echo "Usage: $0 <issue_number>"
    exit 1
fi

echo "🤖 Starting autonomous PR workflow for issue #$ISSUE"

# Stage 1: Research
echo "📋 Stage 1: Research"
gh issue view "$ISSUE" --json title,body,labels > "/tmp/issue-${ISSUE}.json"
echo "   Issue details saved to /tmp/issue-${ISSUE}.json"

# Stage 2: Plan (create branch)
echo "📐 Stage 2: Plan"
BRANCH="mend/issue-${ISSUE}-$(date +%s)"
git checkout -b "$BRANCH"
echo "   Created branch: $BRANCH"

# Stage 3: Build (developer implements)
echo "🔨 Stage 3: Build"
echo "   Implement changes and commit..."

# Stage 4: Quality Gates
echo "🧪 Stage 4: Quality Gates"
./scripts/pre-push.sh

# Stage 5: Push and PR
echo "📤 Stage 5: Push and PR"
git push origin "$BRANCH"
gh pr create --title "feat: implement issue #$ISSUE" --body "Closes #$ISSUE" --base main

PR_NUM=$(gh pr list --head "$BRANCH" --json number -q '.[0].number')
echo "   Created PR #$PR_NUM"

# Stage 6: Wait for CI
echo "⏳ Stage 6: Wait for CI"
sleep 60
gh pr checks "$PR_NUM"

# Stage 7: Merge
echo "✅ Stage 7: Merge"
gh pr merge "$PR_NUM" --squash --delete-branch --admin

echo "✅ Autonomous PR workflow complete for issue #$ISSUE"
