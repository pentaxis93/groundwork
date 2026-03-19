---
name: begin
description: >-
  Session work initiation: select issue(s), prepare workspace, declare
  direction. Opening bookend of the session lifecycle — `land` is the closing
  bookend. Trigger on: 'begin', 'begin work', 'start session', 'start issue'.
requires: ["issue"]
accepts: []
produces: ["claim"]
may_produce: []
trigger:
  on_artifact: "issue"
---

# Begin — Work Selection & Initiation

## Overview

Use this skill to start a work session: choose what to work on, prepare the
workspace, and declare the session's direction.

`begin` is the opening bookend of the session lifecycle:
`begin` (select + prepare) → implement → `submit` (package for review) →
review → `land` (merge and close).

Plan from the issue graph, not from memory. Agent sessions end and context
windows close, but the issue graph persists — it is the only working memory
that survives across sessions.

For issue decomposition and boundary contracts, use `decompose`.
For first-principles design decisions, use `reckon`.

## Procedures

### session-open

#### Phase 0: Opening

The opening ceremony equips the session with everything subsequent phases need:
the methodology context that connects skills into a coherent system, awareness
of the current workspace state, a directional frame that guides selection, and
a clean starting surface. Each step builds on the previous — orient loads the
methodology, observe reads the workspace, frame sets direction, banish clears
the path.

Follows the LBRP sequence: orient → observe → frame → banish.

##### 0a. Orient

The agent receives its operating methodology — the connected system that makes
later skills work together rather than in isolation. `begin` opens individual
work sessions; `orient` establishes the methodology those sessions operate
within. If `orient` has not been loaded this session, load it now before
proceeding.

```
◈ ORIENT
Methodology: loaded
```

Implementation: invoke the `orient` skill via the Skill tool.

##### 0b. Observe

Compact workspace snapshot. This is the only status check — do not repeat these
commands in later phases.

```
◈ STATUS
Git: [clean/dirty] | Branch: [name] | Last: [commit-oneline]
```

Implementation: `git status --short`, `git log --oneline -1`,
`git branch --show-current`. Report in one block.

##### 0c. Frame

Establish the session's purpose using the Four Touches:

| Touch | Question |
|-------|----------|
| **Purpose** | Why does this session exist? |
| **Success** | What does a completed increment look like? |
| **In scope** | What boundaries contain this work? |
| **Out of scope** | What nearby work is explicitly excluded? |

Frame depth varies by what is known at invocation. When issue number(s) are
provided, derive all four touches from the issue body — purpose from the
summary, success from acceptance criteria, scope from the issue boundary. This
yields a full frame. When invoked with a topic or no arguments, frame
directionally — purpose is "advance the project," success is "one session-sized
increment," scope sharpens after selection. A partial frame is expected; do not
block here.

```
◈ FRAME
Purpose: [one sentence]
Success: [verifiable outcome]
Scope: [in] / [out]
```

##### 0d. Banish

Evaluate workspace state (observed in 0b) against the frame (established in
0c). Clear debris so selection starts clean.

- **Clean workspace** → proceed.
- **Changes relevant to frame** → keep and note.
- **Stale or unrelated changes** → stash or discard (confirm with user before
  destructive action).

```
◈ BANISH
[CLEAN | kept: reason | stashed: reason]
```

#### Phase 1: Selection

Determine what to work on. Use observations from Phase 0b — do not re-run
status checks.

**If issue number(s) provided:** selection is already resolved. Fetch issue
thread(s) via `gh issue view`, confirm they are open and unblocked, then
proceed to Phase 2. When multiple issue numbers are given, they may be batched:
2-3 cohesive issues can be addressed in a single session and packaged as one PR
when they share a concern boundary.

**If topic string provided:**

1. List open issues: `gh issue list --state open`.
2. Identify open issues related to the topic (title, labels, body content).
3. Shortlist 3-5 matches, rank by relevance and impact.
4. Select one issue (or a cohesive batch of 2-3).
5. Proceed to Phase 2.

**If no arguments:**

1. List open issues: `gh issue list --state open`.
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

Declare the session's direction. The frame from Phase 0c feeds directly into
this — refine it with what you learned during selection and preparation.

- **Starting direction**: what you intend to accomplish (a direction, not a
  rigid prediction — this will sharpen as you work).
- **Scope gate**: specific nearby work intentionally excluded this session.
- **Issue(s) in scope**: which issue(s) this session addresses.

### session-close

1. Reach a stable checkpoint (done increment or explicit WIP note).
2. Update issue state and leave a concise progress comment.
3. Record decisions, blockers, and the exact next step.
4. Ensure any follow-up work is represented as issue(s).
5. Sync workspace and close.

## Key Terms

Brief definitions for self-contained use. See
[`issue-model.md`](../../docs/architecture/issue-model.md) for the full treatment.

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

## Corruption Modes

- `recency-drift`: picking last-touched work instead of highest leverage.
- `scope-creep`: crossing concern boundaries mid-session.
- `blocker-bypass`: beginning blocked work anyway.
- `state-lag`: issue tracker not reflecting real implementation state.
- `open-loop-close`: ending session without a concrete next step.
- `skip-preparation`: jumping from selection to implementation without setting
  up a feature branch — loses workspace isolation and makes `submit` harder.

## Cross-References

- `submit`: the next lifecycle phase — commit, push, and PR creation after
  implementation.
- `land`: the closing bookend — closing ceremony and mechanical merge after
  review. `begin`'s opening ceremony (orient, observe, frame, banish) prepares
  the agent for work; `land`'s closing ceremony (gather, verify, review, seal)
  prepares the work for delivery. Parallel structure, inverse direction.
- `decompose`: decomposition, issue boundaries, acceptance criteria contracts.
- `reckon`: validate assumptions before committing to an approach.
- `specify`: behavior-first contract definition for implementation increments.
- `orient`: the methodology map — activates the connected skill
  system that `begin` operates within. Loaded during orient (Phase 0a).
- Opening ceremony pattern adapted from LBRP (`aiandi-dev-environment`) —
  internalized, no runtime dependency.
