---
name: next-issue
description: >-
  Work initiation discipline: select session-sized work from the issue graph,
  set up the workspace, and leave the next session a truthful handoff.
  Opening bookend to `land`. Use for starting a work session, whether you
  already know the issue or need to choose one.
---

# Next Issue

The opening bookend of the work pipeline. `next-issue` initiates work;
`land` closes it. Between them: plan, implement, verify.

Key terms — *issue graph*, *unblocked*, *execution layer*, *session-sized* —
are defined in WORKFLOW.md § Issue-Based Development.

For issue decomposition and boundary contracts, use `issue-craft`.
For first-principles design decisions, use `ground`.

## Goal

Select session-sized work, prepare the workspace, and start with a clear
direction. End with an honest handoff to the next session.

## Invocation Modes

The skill accepts three input forms. Selection is skipped when unnecessary.

### No arguments

Full selection from the issue graph. This is the default when the agent
does not know what to work on.

→ Proceeds through: selection → preparation → session open.

### Issue number(s)

One or more issue numbers provided by the operator or already known.
Multiple issues are batched into a single PR when they form a cohesive
change.

→ Skips selection. Proceeds through: preparation → session open.

### Topic string

A phrase describing the area of work (e.g., "documentation thread",
"CLI error handling"). Narrows the candidate set, then proceeds through
selection from the narrowed list.

→ Proceeds through: narrowed selection → preparation → session open.

## Constraints

- `issue-tracker-source-of-truth`: planning state lives in forge issues, not local task trackers.
- `one-session-increment`: commit to one independently verifiable increment.
- `dependencies-are-hard-blockers`: do not start blocked work.
- `session-close-mandatory`: every session ends with explicit state update.
- `workspace-before-code`: branch and draft PR exist before implementation begins.

## Requirements

- `next-action-is-executable`: next action names artifact, command, and done condition.
- `priority-from-impact`: prioritize value, time criticality, and unblock leverage.
- `scope-gate-explicit`: record what is intentionally out of scope for this session.
- `state-is-honest`: issue status/comments reflect actual implementation state.
- `handoff-is-actionable`: end with concrete next step for the next session.

## Procedures

### select

Applies when no issue number is provided. Skip entirely when issue(s) are
already known.

0. Sync local issues: `gh-issue-sync pull`.
1. Read operator request and relevant issue thread(s).
2. If a topic string was provided, filter candidates to that area.
3. Identify all ready (unblocked) candidate issues — an issue is ready when
   its body is agent-executable and every hard dependency is closed.
4. Apply force filters first: direct operator request or hard deadline.
5. Rank top candidates by value, time criticality, unblock leverage,
   and expected effort.
6. Select one issue or a cohesive batch (2-3 related issues that belong
   in a single PR).

Decision stack (≤3 minutes):
1. Force filter: operator request or deadline cliff wins immediately.
2. Shortlist 3-5 unblocked issues from the lowest available execution layer.
3. Score (WSJF-lite): `(Value + TimeCriticality + UnblockLeverage) / Effort`.
4. Prefer the highest score that can be completed this session.
5. If tie: choose the option that unblocks the most downstream work.

### prepare

Set up the workspace for the selected work. This phase runs for all
invocation modes.

0. Sync local issues if not already synced: `gh-issue-sync pull`.
1. Create a feature branch from `main`:
   - Single issue: `issue-<number>/<slug>` (e.g., `issue-27/next-issue-initiation`)
   - Multi-issue batch: `issues-<numbers>/<slug>` (e.g., `issues-5-8/attribution-cleanup`)
2. Push the branch and open a draft PR referencing the issue(s):
   - Title: concise description of the work
   - Body: `Resolves #N` (or `Resolves #N, resolves #M` for batches)
   - Draft status: the PR is not ready for review until verification passes
3. Load issue context into the session: read issue body, comments, and
   any linked issues or dependencies.

### session-open

Note the starting direction. This is a compass heading, not a contract —
real development is iterative and the goal sharpens as you learn.

Write:
- `Starting direction`: what you intend to accomplish (1-2 sentences).
- `Scope gate`: specific nearby work intentionally excluded this session.

Do not require a binary done condition upfront. The closing handoff is
where honest state matters most.

### session-close

1. Reach stable checkpoint (done increment or explicit WIP note).
2. Push commits to the feature branch.
3. Update issue state and leave a concise progress comment.
4. Record decisions, blockers, and the exact next step.
5. Ensure any follow-up work is represented as issue(s).
6. Sync all changes to remote: `gh-issue-sync push`.

The closing handoff is the load-bearing artifact:
- Where you actually ended up (not what you predicted).
- What remains and what's next.
- Whether the draft PR is ready for review or still WIP.

## Multi-Issue Batching

Batching 2-3 cohesive issues into a single PR is a legitimate pattern when:
- The issues share a single deliverable (e.g., related cleanup tasks).
- Implementing them separately would create throwaway intermediate states.
- They are all session-sized *in aggregate*.

The draft PR references all issues. At session close, `land` closes all
referenced issues. Note: `land` currently handles single-issue closing;
for multi-issue batches, close additional issues manually until `land`
gains multi-issue support.

Do not batch unrelated work — that is scope creep, not batching.

## Corruption Modes

- `recency-drift`: picking last-touched work instead of highest leverage.
- `scope-creep`: crossing concern boundaries mid-session.
- `blocker-bypass`: beginning blocked work anyway.
- `state-lag`: issue tracker not reflecting real implementation state.
- `open-loop-close`: ending session without a concrete next step.
- `undefined-state`: using terms like "unblocked" or "session-sized" without
  operational definitions — see WORKFLOW.md § Issue-Based Development.
- `skipped-preparation`: starting implementation without a branch and draft PR.
- `directionless-start`: beginning implementation without noting a starting
  direction or scope gate — the session-open phase was skipped entirely.
- `premature-precision`: treating the starting direction as a rigid contract
  instead of a compass heading that sharpens with learning.
- `forced-selection`: running full selection when the operator already
  specified the issue — wasting time re-deriving a known answer.
- `unbatched-coupling`: working issues separately that should be a single
  cohesive PR, creating throwaway intermediate states.

## Principles

- `clarity-over-volume`: fewer, sharper goals beat broad, vague activity.
- `truthful-state`: inaccurate issue state is planning debt.
- `finish-or-frame`: either finish the increment or clearly frame unfinished state.
- `closing-over-opening`: the closing handoff matters more than the opening direction.
  Honest state at session end is more valuable than precise prediction at session start.

## Cross-References

- `land`: the closing bookend — `next-issue` initiates work, `land` closes it.
- `issue-craft`: decomposition, issue boundaries, acceptance criteria contracts.
- `ground`: validate assumptions before committing to an approach.
- `bdd`: behavior-first test strategy for implementation increments.
