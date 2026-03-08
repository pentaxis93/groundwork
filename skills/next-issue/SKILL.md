---
name: next-issue
description: Session work-selection discipline for issue-tracker-first execution. Use for selecting next work, declaring a concrete session goal, and closing with explicit state updates.
---

# Next Issue

Plan from the issue graph, not from memory. Agent sessions end and context
windows close, but the issue graph persists — it is the only working memory
that survives across sessions. The goal: choose the highest-leverage unblocked
issue, execute one session-sized increment, and leave the next session a
truthful handoff.

For issue decomposition and boundary contracts, use `issue-craft`.
For first-principles design decisions, use `ground`.

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

## Operating Principles

- **Issue tracker is the source of truth.** Planning state lives in forge
  issues, not local task trackers or agent memory. Sessions end; the graph
  doesn't. Issue status and comments reflect actual implementation state —
  inaccurate state is planning debt.
- **Declare the goal before coding.** Write one concrete session goal with a
  binary done condition and an explicit scope gate (what nearby work is
  intentionally excluded). Without an explicit goal, scope creep is invisible
  until the session is over.
- **One session, one increment.** Commit to one independently verifiable
  increment. This keeps work finishable and reviewable.
- **Dependencies are hard blockers.** Do not start work whose dependencies are
  still open — blocked work produces partial results that complicate the graph.
- **Every session closes with a handoff.** End with an honest state update and a
  concrete, actionable next step. The next session (same or different agent)
  should be able to pick up without guessing.
- **Next actions are executable.** Each next action names an artifact, a command,
  and a done condition — not a vague intention.
- **Prioritize by impact.** Rank by value delivered, time criticality, and
  unblock leverage (how much downstream work this frees), weighed against
  expected effort.

## Procedures

### session-open

1. Sync local issues: `gh-issue-sync pull`.
2. Read operator request and relevant issue thread(s).
3. Identify all ready (unblocked) candidate issues — an issue is ready when its
   body is agent-executable and every hard dependency is closed.
4. Apply force filters first: a direct operator request or hard deadline wins
   immediately.
5. Shortlist 3-5 candidates from the lowest available execution layer. Rank by
   value, time criticality, and unblock leverage relative to effort. Be
   decisive — selection should not consume significant session time.
6. If candidates tie: choose the one that unblocks the most downstream work.
7. Select one session-sized increment.
8. Declare:
   - **Session goal**: one observable outcome (artifact or behavior).
   - **Done condition**: binary pass/fail check.
   - **Scope gate**: specific nearby work intentionally excluded this session.

### session-close

1. Reach a stable checkpoint (done increment or explicit WIP note).
2. Update issue state and leave a concise progress comment.
3. Record decisions, blockers, and the exact next step.
4. Ensure any follow-up work is represented as issue(s).
5. Sync all changes to remote: `gh-issue-sync push`.
6. Sync workspace and close.

## Corruption Modes

- `recency-drift`: picking last-touched work instead of highest leverage.
- `implicit-goal`: starting implementation without explicit session goal.
- `scope-creep`: crossing concern boundaries mid-session.
- `blocker-bypass`: beginning blocked work anyway.
- `state-lag`: issue tracker not reflecting real implementation state.
- `open-loop-close`: ending session without a concrete next step.
- `undefined-state`: using terms like "unblocked" or "session-sized" without
  operational definitions — see Key Terms above.

## Principles

- `clarity-over-volume`: fewer, sharper goals beat broad, vague activity.
- `truthful-state`: inaccurate issue state is planning debt.
- `finish-or-frame`: either finish the increment or clearly frame unfinished state.

## Cross-References

- `issue-craft`: decomposition, issue boundaries, acceptance criteria contracts.
- `ground`: validate assumptions before committing to an approach.
- `bdd`: behavior-first test strategy for implementation increments.
