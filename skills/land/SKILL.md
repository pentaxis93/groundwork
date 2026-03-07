---
name: land
description: "One-word closeout workflow: merge active branch to main, sync local main, delete feature branch (remote+local), post issue completion comment, and close issue(s). Trigger on: 'land', 'merge and close', 'ship it'."
---

# Land — Merge, Sync, Cleanup, Close

**Version 1.0**

## Overview

Use this skill when the user wants full delivery closure in one command.

`land` means:
1. Merge the active feature branch into `main`
2. Push `main`
3. Remove the feature branch (remote + local)
4. Post a completion comment on the issue(s)
5. Close the issue(s)
6. Verify final state

Do not stop after merge.
Do not ask for an additional confirmation before landing; invoking `land` is the user's approval to execute this workflow.

---

## Preconditions

- Working tree must be clean before starting.
- Current branch must not be `main`.
- Commands in this skill are Bash-specific and must be run under `bash`.
- CHANGELOG must include entries for user-visible changes. Version bumps must state the rationale for the increment level.
- Issue number(s) must be known:
  - Prefer explicit user-provided issue number(s).
  - Else infer from branch name using one of:
    - `issue-<number>/<slug>` (single issue)
    - `issues-<number>-<number>-.../<slug>` (multi-issue, unbounded)

---

## Procedure

### 1. Resolve context

```bash
FEATURE_BRANCH="$(git branch --show-current)"
if [ "$FEATURE_BRANCH" = "main" ]; then
  echo "ERROR: land must run from a feature branch"; exit 1
fi

if [ -z "${BASH_VERSION:-}" ]; then
  echo "ERROR: land requires bash (BASH_VERSION not set)"; exit 1
fi

git diff --quiet && git diff --cached --quiet || {
  echo "ERROR: working tree not clean"; exit 1;
}

ISSUE_NUMBERS=()

if [[ "$FEATURE_BRANCH" =~ ^issue-([0-9]+)/ ]]; then
  ISSUE_NUMBERS=("${BASH_REMATCH[1]}")
elif [[ "$FEATURE_BRANCH" =~ ^issues-([0-9]+(-[0-9]+)*)/ ]]; then
  IFS='-' read -r -a ISSUE_NUMBERS <<< "${BASH_REMATCH[1]}"
else
  echo "ERROR: cannot infer issue number(s) from branch name; require explicit issue number(s)"; exit 1
fi

if [ "${#ISSUE_NUMBERS[@]}" -eq 0 ]; then
  echo "ERROR: parsed zero issue numbers"; exit 1
fi
```

### 2. Verify CHANGELOG

```bash
if git diff origin/main...HEAD --name-only | grep -q CHANGELOG.md; then
  echo "CHANGELOG entry present"
else
  echo "WARNING: no CHANGELOG entry — verify this branch has no user-visible changes"
fi
```

If the warning fires, confirm with the user before proceeding.

### 3. Merge and push

```bash
git fetch origin --prune
git checkout main
git pull --ff-only origin main
git merge --no-ff "$FEATURE_BRANCH"
git push origin main
MERGE_SHA="$(git rev-parse --short HEAD)"
```

### 4. Delete feature branch

```bash
git push origin --delete "$FEATURE_BRANCH" || true
git branch -d "$FEATURE_BRANCH"
git fetch origin --prune
```

### 5. Discover PR number (best effort)

```bash
PR_NUMBER="$(gh pr list --head "$FEATURE_BRANCH" --state merged --json number --jq '.[0].number')"
```

If PR is not found, continue with issue close using merge commit only.

### 6. Comment and close issue(s)

```bash
if [ -n "$PR_NUMBER" ]; then
  BODY="Implemented and merged in PR #${PR_NUMBER} (commit ${MERGE_SHA}). Closing as complete."
else
  BODY="Implemented and merged in commit ${MERGE_SHA}. Closing as complete."
fi

FAILED_ISSUES=()
for ISSUE_NUMBER in "${ISSUE_NUMBERS[@]}"; do
  gh issue comment "$ISSUE_NUMBER" --body "$BODY" || FAILED_ISSUES+=("$ISSUE_NUMBER")
  gh issue close "$ISSUE_NUMBER" --reason completed || FAILED_ISSUES+=("$ISSUE_NUMBER")
done

# Optional: de-duplicate failures before reporting.
if [ "${#FAILED_ISSUES[@]}" -gt 0 ]; then
  echo "WARNING: failed issue operations for: ${FAILED_ISSUES[*]}"
fi
```

### 6a. Sync issue state to local mirror

```bash
gh-issue-sync pull
```

### 7. Verify and report

```bash
git status --short
git branch --show-current
git rev-parse --short HEAD

for ISSUE_NUMBER in "${ISSUE_NUMBERS[@]}"; do
  gh issue view "$ISSUE_NUMBER" --json state --jq '.state'
done
```

Success conditions:
- current branch is `main`
- working tree is clean
- feature branch absent on origin
- every target issue state is `CLOSED`

---

## Failure Policy

- If merge/push fails: stop immediately, do not close issue.
- If branch deletion fails after successful merge: report partial completion and keep issue(s) open.
- If issue comment/close API fails for one issue: continue processing remaining issues, then report failed issue number(s) explicitly.

---

## Related Skills

- `issue-craft` for issue lifecycle patterns
- `next-issue` for issue selection and session workflow
