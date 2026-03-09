---
name: systematic-debugging
description: >-
  Root-cause investigation discipline. Use when a test fails, behavior is
  unexpected, or any failure occurs — before proposing fixes. Enforces
  structured investigation before fix attempts. Fires at any pipeline stage
  when failures appear. If you are about to fix something without understanding
  why it broke, this skill applies.
metadata:
  version: "1.0.0"
  updated: "2026-03-09"
  origin: >-
    Adapted from Jesse Vincent's systematic-debugging skill in
    obra/superpowers (https://github.com/obra/superpowers, MIT license,
    pinned at e4a2375). The original is a standalone debugging discipline
    with a four-phase investigation model, 3-fix architectural escalation
    rule, and evidence-gathering-before-fixing discipline. This skill
    preserves substantial portions of the original — the Iron Law, the
    investigation phases, the escalation rule, the anti-rationalization
    patterns — and restructures them as a cross-cutting discipline that
    composes bidirectionally with groundwork's test-first, verification,
    ground, and third-force skills. See LICENSE-UPSTREAM for the full
    copyright notice and license terms.
  replaces: "systematic-debugging (obra/superpowers)"
---

# Systematic Debugging

*Find root cause before fixing. Symptom fixes are failure.*

## The Iron Law

```
NO FIXES WITHOUT ROOT CAUSE INVESTIGATION FIRST
```

Proposed a fix before understanding the cause? Retract it. Start over with
investigation.

- Do not keep the fix as a "working theory"
- Do not test it "just to see"
- Do not propose it alongside investigation
- Retract means retract

Investigate from evidence. No exceptions.

## Lifecycle Role

This skill is a cross-cutting discipline in groundwork's pipeline, alongside
`ground` (first-principles on creation) and `third-force` (structural
resolution on friction). It fires at any stage when failures appear — not
only during execution.

All three share the same cognitive shape:

| Discipline | Default Impulse | Interrupt |
|---|---|---|
| `ground` | Start from what exists | Stop. What is actually needed? |
| `third-force` | Route around friction | Stop. What is the structural cause? |
| `systematic-debugging` | Guess and fix | Stop. What is the root cause? |

Debugging is not a phase. Failures surface during grounding (constraint
violation in existing system), planning (bug in code you are designing
around), execution (test failure, integration failure), and landing
(regression discovered during merge). The trigger is the failure, not the
pipeline position.

**Handoff with test-first:** This skill owns investigation methodology.
`test-first`'s fix-bug procedure owns the execution cycle — write failing
test, implement fix, verify green. The boundary: once root cause is
established, hand off to `test-first` fix-bug. This skill does not write
tests or implement fixes.

**Handoff with verification-before-completion:** This skill does not verify
fixes. Once a fix is implemented through `test-first`, `verification-before-
completion` gates the completion claim.

## The Investigation Move

Five steps. Always the same.

0. **Stop.** You have hit a failure. Do not guess. Do not fix. Do not
   propose a solution. The instinct to "just try something" is the failure
   mode this skill exists to catch. Every guess that happens to work teaches
   you nothing and leaves the actual cause in place.

   Fix momentum carries agents past failures before investigation even
   registers as a distinct activity. Stop exists to interrupt this momentum.
   Without it, agents will analyze the failure while simultaneously drafting
   a fix, and the fix — dressed as pragmatism — will win because it is
   downstream of task momentum.

1. **Read.** Read the actual error output. All of it.

   - Full stack trace, every line — not just the top
   - Error messages, exit codes, log output
   - Warnings that preceded the failure
   - The test name and assertion that failed

   Most root causes are stated plainly in output that was skimmed or skipped.
   Do not interpret before reading. Read before interpreting.

2. **Reproduce.** Confirm you can trigger the failure reliably.

   - What exact steps trigger it?
   - Is it consistent or intermittent?
   - What changed recently? (`git diff`, recent commits, new dependencies,
     config changes, environment differences)

   If you cannot reproduce reliably, gather more data — add logging, narrow
   the conditions, isolate the component. Do not proceed to tracing with an
   unreproducible failure. You cannot verify a fix for a failure you cannot
   trigger.

3. **Trace.** Trace backward from symptom to source.

   The error manifests at one point in the code. That is rarely where the
   bug lives. Trace the data flow backward through the call chain:

   - Where does the bad value appear? What function produced it?
   - What called that function? What did it pass?
   - Keep tracing up until you find where valid data becomes invalid.
   - That transition point is the root cause. Fix there, not at the symptom.

   **Tracing technique:**
   1. Start at the error — note the bad value or state.
   2. Find the immediate producer of that value.
   3. Ask: "What called this with bad input?" Trace one level up.
   4. Repeat until you find the origin — where valid-to-invalid happens.
   5. Verify: does fixing at this origin resolve the symptom?

   When manual tracing is blocked, add instrumentation: log the value and
   call stack at each level. Run once to gather evidence, then analyze the
   evidence — do not interleave gathering and fixing.

   After fixing at source, consider adding validation at each layer the data
   passes through — see
   [`references/defense-in-depth.md`](references/defense-in-depth.md) for
   the pattern.

4. **Hypothesize and test.** Form a single, specific hypothesis.

   - State it clearly: "The root cause is X because evidence Y shows Z."
   - Make the smallest possible change to test the hypothesis.
   - One variable at a time. Do not fix multiple things at once.
   - If the hypothesis is confirmed: hand off to `test-first` fix-bug.
   - If the hypothesis is wrong: form a new hypothesis from the evidence.
     Do not stack fixes on top of a failed hypothesis.

---

## The 3-Fix Escalation Rule

Count your fix attempts. If you have tried 3 fixes and the failure persists:

**Stop fixing. Question the architecture.**

Three failed fixes is a signal — not of bad debugging, but of a structural
problem. The pattern:

- Each fix reveals a new issue in a different place
- Fixes require "massive refactoring" to implement
- Each fix creates new symptoms elsewhere
- The root cause keeps shifting

When this fires:

1. **Stop.** Do not attempt fix #4.
2. **Invoke `ground`.** Re-examine the architectural assumptions. Is this
   pattern fundamentally sound? Is the design carrying inherited complexity?
   Should the architecture change rather than another patch?
3. **If the architecture is sound**, the investigation was incomplete — return
   to Step 0 with the new evidence from your three attempts.
4. **If the architecture is unsound**, the fix is architectural. File an issue
   via `issue-craft` — this exceeds debugging scope.

This rule is the debugging equivalent of `ground`'s "infinite decomposition"
corruption mode — it catches the failure of continuing to apply the skill's
own method past the point where it is productive.

---

## Pattern Analysis

When the root cause is not obvious from tracing, compare against known-good:

1. **Find working examples.** Locate similar working code in the same
   codebase. If implementing a known pattern, find a reference implementation.
2. **Read completely.** Do not skim the reference. Read every line. Partial
   understanding of a pattern guarantees bugs.
3. **Compare systematically.** List every difference between working and
   broken, however small. Do not assume "that cannot matter."
4. **Check assumptions.** What dependencies, config, or environment does the
   working version rely on? What does the broken version assume?

---

## Recognition Patterns

These are the signals that you are about to violate the Iron Law. When you
notice any of these, stop. Return to Step 0.

### 1. Premature Fix

You are proposing a solution before completing investigation.

***Recognition:*** You have not traced the data flow, but you have a fix in
mind. The fix "seems obvious." You are about to say "I think the issue is X,
let me fix it."
***Corrective:*** "Have I traced to root cause, or am I guessing from
symptoms?" Return to Step 3.

### 2. Fix Stacking

You are adding fixes on top of previous failed fixes.

***Recognition:*** Your previous fix did not work, and you are about to make
another change without removing the first fix or re-analyzing. The codebase
now has two changes, neither of which you understand fully.
***Corrective:*** Revert the failed fix. Return to Step 0 with new evidence.

### 3. Symptom Treatment

You are fixing where the error appears rather than where it originates.

***Recognition:*** Your fix adds a null check, a try/catch, a default value,
or a conditional at the error site. The bad data still flows through the
system — you are just catching it later.
***Corrective:*** "Am I fixing the source or the symptom?" Return to Step 3.

### 4. Evidence-Free Hypothesis

You are guessing without evidence.

***Recognition:*** Your hypothesis does not reference specific evidence from
Steps 1-3. You are reasoning from how the code "should" work rather than
from what it actually does. You are about to say "it is probably X."
***Corrective:*** "What specific evidence supports this hypothesis?" If none,
return to Step 1.

### 5. Investigation Fatigue

You are tempted to skip investigation because prior attempts failed.

***Recognition:*** You have been investigating for a while without progress.
The instinct is to "just try something and see." This is the most dangerous
moment — fatigue makes guess-and-check feel like progress.
***Corrective:*** Take a different angle. Add more instrumentation. Narrow
the reproduction. Do not abandon methodology when it feels slow — abandoning
it is slower.

---

## Anti-Rationalization

Every excuse in this table has been offered sincerely. Every one leads to
thrashing.

| Excuse | Reality |
|--------|---------|
| "The issue is simple, I do not need the process" | Simple issues have root causes too. Investigation is fast for simple bugs. |
| "Emergency, no time for process" | Systematic investigation is faster than guess-and-check thrashing. |
| "Just try this first, then investigate" | The first guess sets the pattern. Do it right from the start. |
| "I see the problem, let me fix it" | Seeing the symptom is not understanding the root cause. |
| "Multiple fixes at once saves time" | Cannot isolate what worked. Creates new bugs. |
| "The reference is too long, I will adapt the pattern" | Partial understanding guarantees bugs. Read it completely. |
| "One more fix attempt" (after 2+ failures) | 3+ failures = architectural problem. Question the pattern. |
| "I do not fully understand, but this might work" | "Might work" is guessing. Investigate until you understand. |

---

## Corruption Modes

**Guess-and-check.** Proposing fixes without investigation. Recognition: you
have a fix before you have a root cause. You are testing hypotheses by
changing code rather than by reading evidence. Your "debugging" is a series
of edits followed by test runs.

**Symptom-fixing.** Fixing where the error appears rather than where it
originates. Recognition: your fix is a guard, a check, a catch, or a default
at the error site. The invalid data still enters the system — you are just
absorbing it downstream.

**Fix-stacking.** Adding changes without removing failed attempts.
Recognition: you have multiple uncommitted changes, some of which are from
prior failed hypotheses. You are no longer sure which changes are diagnostic
and which are fixes. The codebase is in an unknown state.

**Investigation-without-action.** Investigating indefinitely without
converging on a hypothesis. Recognition: you have extensive notes on what the
code does but no specific hypothesis about what is wrong. Investigation
serves hypothesis formation — if investigation is not converging, change
angle or add instrumentation. This is the debugging equivalent of `ground`'s
"infinite decomposition."

**Process as theater.** Going through investigation motions without
questioning. Recognition: your investigation confirms your initial intuition
at every step. You are collecting evidence that supports the fix you already
have in mind. If investigation always confirms your first guess, you are
rationalizing, not investigating.

---

## Cross-References

- `test-first`: owns the execution cycle for bug fixes. This skill
  establishes root cause; `test-first` fix-bug writes the failing test and
  implements the fix. The handoff: root cause established, hand off to
  `test-first`.
- `verification-before-completion`: owns fix verification. This skill does
  not verify — it investigates.
- `ground`: the 3-fix escalation rule invokes `ground` to re-examine
  architectural assumptions. The investigation move shares `ground`'s
  discipline of establishing truth before acting.
- `third-force`: when investigation reveals the failure is caused by
  operational friction (missing tool, broken config, stale convention), hand
  off to `third-force` — the root cause is environmental, not logical.
- `bdd`: behavior contracts define what "unexpected" means. When behavior is
  unexpected, check it against the behavior contract first — "unexpected" is
  only meaningful relative to a defined expectation.
