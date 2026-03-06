---
name: land
description: "One-word closeout workflow: merge active branch to main, sync local main, delete feature branch (remote+local), post issue completion comment, and close the issue. Trigger on: 'land', 'merge and close', 'ship it'."
---

# Land — Merge, Sync, Cleanup, Close

**Version 1.0**

## Overview

Use this skill when the user wants full delivery closure in one command.

`land` means:
1. Merge the active feature branch into `main`
2. Push `main`
3. Remove the feature branch (remote + local)
4. Post a completion comment on the issue
5. Close the issue
6. Verify final state

Do not stop after merge.

---

## Preconditions

- Issue number must be known:
  - Prefer explicit user-provided issue number.
  - Else infer from branch name pattern `issue-<number>`.
- Use WeForge API token from `pass`:
  - Default: `pass show weforge/fulcrum-token`

---

## Procedure

### 1. Resolve context

```bash
FEATURE_BRANCH="$(git branch --show-current)"

ORIGIN_URL="$(git remote get-url origin)"
REPO_PATH="$(echo "$ORIGIN_URL" | sed -E 's#(https?://[^/]+/|git@[^:]+:)##; s#\\.git$##')"
OWNER="${REPO_PATH%%/*}"
REPO="${REPO_PATH##*/}"

ISSUE_NUMBER="${ISSUE_NUMBER:-}"
if [ -z "$ISSUE_NUMBER" ]; then
  ISSUE_NUMBER="$(echo "$FEATURE_BRANCH" | sed -nE 's#.*issue-([0-9]+).*#\\1#p')"
fi
if [ -z "$ISSUE_NUMBER" ]; then
  echo "ERROR: cannot infer issue number from branch name; require explicit issue"; exit 1
fi

AUTO_BRANCH_CREATED=0
if [ "$FEATURE_BRANCH" = "main" ]; then
  FEATURE_BRANCH="issue-${ISSUE_NUMBER}-land-$(date +%Y%m%d-%H%M%S)"
  git checkout -b "$FEATURE_BRANCH"
  AUTO_BRANCH_CREATED=1
fi

if ! git diff --quiet || ! git diff --cached --quiet || [ -n "$(git ls-files --others --exclude-standard)" ]; then
  git add -A
  git commit -m "chore: finalize issue #${ISSUE_NUMBER} before land"
fi
```

### 2. Merge and push

```bash
git fetch origin --prune
git checkout main
git pull --ff-only origin main
git merge --no-ff "$FEATURE_BRANCH"
git push origin main
MERGE_SHA="$(git rev-parse --short HEAD)"
```

### 3. Delete feature branch

```bash
git push origin --delete "$FEATURE_BRANCH" || true
git branch -d "$FEATURE_BRANCH"
git fetch origin --prune
```

### 4. Discover PR number (best effort)

```bash
PR_NUMBER="$(curl -sf \
  -H "Authorization: token $(pass show weforge/fulcrum-token)" \
  "https://weforge.build/api/v1/repos/${OWNER}/${REPO}/pulls?state=closed&limit=100" \
  | jq -r --arg b "$FEATURE_BRANCH" '.[] | select(.head.ref == $b) | .number' \
  | head -n1)"
```

If PR is not found, continue with issue close using merge commit only.

### 5. Comment and close issue

```bash
if [ -n "$PR_NUMBER" ]; then
  BODY="Implemented and merged in PR #${PR_NUMBER} (commit ${MERGE_SHA}). Closing as complete."
else
  BODY="Implemented and merged in commit ${MERGE_SHA}. Closing as complete."
fi

curl -sf -X POST \
  -H "Authorization: token $(pass show weforge/fulcrum-token)" \
  -H "Content-Type: application/json" \
  -d "$(jq -n --arg body "$BODY" '{body: $body}')" \
  "https://weforge.build/api/v1/repos/${OWNER}/${REPO}/issues/${ISSUE_NUMBER}/comments" >/dev/null

curl -sf -X PATCH \
  -H "Authorization: token $(pass show weforge/fulcrum-token)" \
  -H "Content-Type: application/json" \
  -d '{"state":"closed"}' \
  "https://weforge.build/api/v1/repos/${OWNER}/${REPO}/issues/${ISSUE_NUMBER}" >/dev/null
```

### 5a. Sync issue state to local mirror

```bash
gh-issue-sync pull
```

### 6. Verify and report

```bash
git status --short
git branch --show-current
git rev-parse --short HEAD

curl -sf \
  -H "Authorization: token $(pass show weforge/fulcrum-token)" \
  "https://weforge.build/api/v1/repos/${OWNER}/${REPO}/issues/${ISSUE_NUMBER}" \
  | jq -r '.state'
```

Success conditions:
- current branch is `main`
- working tree is clean
- feature branch absent on origin
- issue state is `closed`

---

## Failure Policy

- If merge/push fails: stop immediately, do not close issue.
- If branch deletion fails after successful merge: report partial completion; issue may still be closed.
- If issue comment/close API fails: report partial completion and include exact failing step.

---

## Related Skills

- `forgejo-api` for API endpoint details
- `credentials` for token-safe command patterns
