---
name: next-issue
description: >-
  Session work initiation: select issue(s), prepare workspace, declare
  direction. Opening bookend of the session lifecycle — `land` is the closing
  bookend. Trigger on: 'next-issue', 'next issue', 'start issue', 'begin work'.
---

# Next Issue — Work Selection & Initiation

**Version 2.0**

## Overview

Use this skill to start a work session: choose what to work on, prepare the
workspace, and declare the session's direction.

`next-issue` is the opening bookend of the session lifecycle:
`next-issue` (select + prepare) → implement → `propose` (package for review) →
review → `land` (merge and close).

Plan from the issue graph, not from memory. Agent sessions end and context
windows close, but the issue graph persists — it is the only working memory
that survives across sessions.

For issue decomposition and boundary contracts, use `issue-craft`.
For first-principles design decisions, use `ground`.

## Invocation Modes

The skill accepts three invocation patterns:

- **No arguments** → full selection from the issue graph.
- **Issue number(s)** (e.g., `#27` or `#5 #8`) → skip selection, proceed
  directly to preparation.
- **Topic string** (e.g., `"skill testing"`) → narrow candidates to
  topic-related issues, then select.

When issue numbers are provided, they may be batched: 2-3 cohesive issues can
be addressed in a single session and packaged as one PR. This is a legitimate
pattern when the issues share a concern boundary.

## Key Terms

Brief definitions for self-contained use. See WORKFLOW.md § Issue-Based
Development for the full treatment.

- **Issue graph**: the set of open issues and their dependency edges — the live
  map of what remains and what blocks what.
- **Unblocked**: an issue whose hard dependencies are all closed.
- **Execution layer**: a set of issues that share no mutual dependencies and can
  be worked in parallel once their shared ancestors are closed. Layer 0 has no
  dependencies; layer 1 depends only on layer 0; and so on.
- **Session-sized**: an issue that one agent can complete — from reading context
  through passing verification — in a single focused session.
- **Issue batch**: 2-3 cohesive issues addressed together when they share a
  concern boundary and their combined scope is still session-sized.

## Operating Principles

- **Issue tracker is the source of truth.** Planning state lives in forge
  issues, not local task trackers or agent memory. Sessions end; the graph
  doesn't. Issue status and comments reflect actual implementation state —
  inaccurate state is planning debt.
- **Direction over prediction.** Capture starting direction at session open.
  Goals sharpen through implementation — rigid upfront done conditions are
  premature precision. The closing handoff matters more than the opening
  prediction.
- **One session, one increment.** Commit to one independently verifiable
  increment. Fewer, sharper goals beat broad, vague activity — this keeps
  work finishable and reviewable.
- **Dependencies are hard blockers.** Do not start work whose dependencies are
  still open — blocked work produces partial results that complicate the graph.
- **Every session closes with a handoff.** Either finish the increment or
  clearly frame unfinished state. End with an honest state update and a
  concrete, actionable next step. The next session (same or different agent)
  should be able to pick up without guessing.
- **Next actions are executable.** Each next action names an artifact, a command,
  and a done condition — not a vague intention.
- **Prioritize by impact.** Rank by value delivered, time criticality, and
  unblock leverage (how much downstream work this frees), weighed against
  expected effort.

## Procedures

### session-open

#### Phase 1: Selection

Determine what to work on. The path depends on invocation mode.

**If issue number(s) provided:** fetch issue thread(s) via `gh issue view`,
confirm they are open and unblocked, and skip to Phase 2.

**If topic string provided:**

1. Sync local issues: `gh-issue-sync pull`.
2. Identify open issues related to the topic (title, labels, body content).
3. Shortlist 3-5 matches, rank by relevance and impact.
4. Select one issue (or a cohesive batch of 2-3).
5. Proceed to Phase 2.

**If no arguments:**

1. Sync local issues: `gh-issue-sync pull`.
2. Identify all ready (unblocked) candidate issues — an issue is ready when its
   body is agent-executable and every hard dependency is closed.
3. Apply force filters first: a direct operator request or hard deadline wins
   immediately.
4. Shortlist 3-5 candidates from the lowest available execution layer. Rank by
   value, time criticality, and unblock leverage relative to effort. Be
   decisive — selection should not consume significant session time.
5. If candidates tie: choose the one that unblocks the most downstream work.
6. Select one issue (or a cohesive batch of 2-3).

#### Phase 2: Preparation

Set up the workspace for the selected work.

1. Ensure on `main` and up-to-date: `git checkout main && git pull --ff-only`.
2. Create a feature branch:
   - Single issue: `issue-<N>/<slug>`
   - Issue batch: `issues-<N>-<M>-.../<slug>` (unbounded)
   - Topic without issue: `feat/<slug>`, `fix/<slug>`, or `chore/<slug>`

   Where slug is the issue title (or topic string, when no issue) — lowercase,
   hyphenated, truncated to 40 chars.
3. Load issue context: read issue body, comments, and linked issues to build
   working understanding.

#### Phase 3: Declaration

Declare the session's direction:

- **Starting direction**: what you intend to accomplish (a direction, not a
  rigid prediction — this will sharpen as you work).
- **Scope gate**: specific nearby work intentionally excluded this session.
- **Issue(s) in scope**: which issue(s) this session addresses.

### session-close

1. Reach a stable checkpoint (done increment or explicit WIP note).
2. Update issue state and leave a concise progress comment.
3. Record decisions, blockers, and the exact next step.
4. Ensure any follow-up work is represented as issue(s).
5. Sync all changes to remote: `gh-issue-sync push`.
6. Sync workspace and close.

## Corruption Modes

- `recency-drift`: picking last-touched work instead of highest leverage.
- `scope-creep`: crossing concern boundaries mid-session.
- `blocker-bypass`: beginning blocked work anyway.
- `state-lag`: issue tracker not reflecting real implementation state.
- `open-loop-close`: ending session without a concrete next step.
- `undefined-state`: using terms like "unblocked" or "session-sized" without
  operational definitions — see Key Terms above.
- `skip-preparation`: jumping from selection to implementation without setting
  up a feature branch — loses workspace isolation and makes `propose` harder.

## Cross-References

- `propose`: the next lifecycle phase — commit, push, and PR creation after
  implementation.
- `land`: the closing bookend — merge, cleanup, and issue closure after review.
  `next-issue` opens the session lifecycle; `land` closes it.
- `issue-craft`: decomposition, issue boundaries, acceptance criteria contracts.
- `ground`: validate assumptions before committing to an approach.
- `bdd`: behavior-first test strategy for implementation increments.
