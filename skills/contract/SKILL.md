---
name: contract
description: Use when carrying a behavior contract through implementation and verification. This skill keeps code changes, tests, and completion claims traceable to the specified behaviors instead of drifting toward implementation convenience.
metadata:
  version: "1.0.0"
  updated: "2026-03-17"
---

# Contract

The behavior contract remains the source of truth after specification is
written. This skill is the discipline of carrying that contract through
implementation, verification, and closure without losing traceability.

## Lifecycle Role

This skill is cross-cutting, not planning-only. Dropping the behavior contract
at any phase breaks traceability.

- **Implementation:** behavior gaps drive execution order. Each implementation
  increment maps to one or more behavior statements. Work that cannot be
  traced to a behavior is unanchored.
- **Verification:** completion means passing evidence for each claimed
  behavior, not just "tests pass." Report behavior coverage, not command
  output.
- **Closure:** shipped work should still name what behaviors were covered and
  what gaps remain.

## Discipline

**Implementation maps to behavior.** Every implementation increment connects to
one or more behavior statements. If you cannot name which behavior a code
change advances, the change is unanchored. This traceability runs the full
chain: from specification through implementation to verification evidence.

**Behavior is the unit of progress.** Execution order follows behavior gaps,
not code adjacency or convenience. Work advances by proving another behavior,
not by accumulating unanchored edits.

**Completion is behavior coverage.** "All tests pass" is incomplete if you
cannot name which claimed behaviors those tests verify.

## Procedures

### map-behaviour-to-implementation

1. For each planned implementation step, name the behavior statements it
   advances.
2. New RED tests during implementation must correspond to named behavior
   scenarios.
3. Reject implementation steps that cannot be traced to behavior requirements;
   they indicate scope creep or missing specification.

### verify-behaviour-contract

1. For each claimed completed behavior, cite passing evidence (test name +
   pass).
2. For failing or deferred behaviors, mark explicit gaps.
3. Report completion status as behavior coverage, not only command success.
   "All tests pass" is not "all behaviors verified."

### evaluate-existing-tests

When a test fails after a change, classify the failure:

- **Bug introduced:** the behavior is still required and the implementation
  broke it. Fix the implementation.
- **Behavior moved:** the behavior still exists but lives somewhere else.
  Redirect the test.
- **Behavior obsolete:** the system no longer needs this behavior. Delete the
  test.

Resist the urge to fix failing tests by weakening assertions. If the behavior
matters, fix the implementation. If it does not, delete the test.

## Triggers

- Implementing issue acceptance criteria
- Deciding what to implement next from an existing behavior contract
- Mapping plan tasks to behavior statements
- Validating completion claims against behavior coverage
- Evaluating whether existing tests still represent required behavior

## Corruption Modes

**Testing implementation.** Asserting internal state, private methods, or
implementation details instead of observable behavior.
*Recognition:* Your test would break if you refactored the internals without
changing behavior. Your assertions reference private fields, internal data
structures, or intermediate state that users never see.

**Contract dropoff.** Treating behavior specification as a phase that ends when
coding begins.
*Recognition:* You wrote behavior specs, then implemented without checking work
against them. At completion, you report "tests pass" without citing which
behaviors have evidence.

**Test hoarding.** Retaining tests for behaviors the system no longer needs.
*Recognition:* Tests pass but describe behavior nobody requires. You are afraid
to delete them because "they might catch something."

## Principles

**Single pipeline.** Specification and `test` execution are complementary
stages in one flow, not alternatives. Specification decides what to prove.
Execution proves it. Verification reports the resulting behavior coverage.

## Cross-References

- `specify`: produces the behavior contract this skill carries forward.
- `test`: executes RED-GREEN-REFACTOR for the named behaviors.
- `verify`: requires behavior-level evidence before completion claims.
- `land`: records shipped behavior coverage and explicit gaps.
