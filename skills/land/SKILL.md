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
Do not ask for an additional confirmation before landing; invoking `land` is the user's approval to execute this workflow.

---

## Preconditions

- Working tree must be clean before starting.
- Current branch must not be `main`.
- Issue number must be known:
  - Prefer explicit user-provided issue number.
  - Else infer from branch name pattern `issue-<number>`.

---

## Procedure

### 1. Resolve context

```bash
FEATURE_BRANCH="$(git branch --show-current)"
if [ "$FEATURE_BRANCH" = "main" ]; then
  echo "ERROR: land must run from a feature branch"; exit 1
fi

git diff --quiet && git diff --cached --quiet || {
  echo "ERROR: working tree not clean"; exit 1;
}

ISSUE_NUMBER="$(echo "$FEATURE_BRANCH" | sed -nE 's#.*issue-([0-9]+).*#\\1#p')"
if [ -z "$ISSUE_NUMBER" ]; then
  echo "ERROR: cannot infer issue number from branch name; require explicit issue"; exit 1
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
PR_NUMBER="$(gh pr list --head "$FEATURE_BRANCH" --state merged --json number --jq '.[0].number')"
```

If PR is not found, continue with issue close using merge commit only.

### 5. Comment and close issue

```bash
if [ -n "$PR_NUMBER" ]; then
  BODY="Implemented and merged in PR #${PR_NUMBER} (commit ${MERGE_SHA}). Closing as complete."
else
  BODY="Implemented and merged in commit ${MERGE_SHA}. Closing as complete."
fi

gh issue comment "$ISSUE_NUMBER" --body "$BODY"
gh issue close "$ISSUE_NUMBER" --reason completed
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

gh issue view "$ISSUE_NUMBER" --json state --jq '.state'
```

Success conditions:
- current branch is `main`
- working tree is clean
- feature branch absent on origin
- issue state is `closed`

---

## Failure Policy

- If merge/push fails: stop immediately, do not close issue.
- If branch deletion fails after successful merge: report partial completion and keep issue open.
- If issue comment/close API fails: report partial completion and include exact failing step.

---

## Related Skills

- `issue-craft` for issue lifecycle patterns
- `next-issue` for issue selection and session workflow
