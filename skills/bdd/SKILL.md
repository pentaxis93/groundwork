---
name: bdd
description: Behaviour-driven development for writing and maintaining executable behavior specifications across specification, implementation, and verification. Use when deciding what to test next, structuring tests with Given/When/Then, mapping behavior to implementation work, or diagnosing failing behavior contracts.
metadata:
  version: "2.0.0"
  updated: "2026-03-07"
---

# Behaviour-Driven Development

BDD reframes testing as behavior specification: what the system should do,
under what context, with observable outcomes.

References:
- [Dan North: Introducing BDD](https://dannorth.net/introducing-bdd/)
- For concrete language-specific patterns, see [references/language-patterns.md](references/language-patterns.md)

## Goal

Drive development through sentence-named behaviors and Given/When/Then
structured scenarios, and keep that behavior contract intact through execution
and verification.

## Lifecycle Role

BDD is a cross-cutting contract, not a planning-only step. Dropping it at any
phase breaks traceability.

- **Specification:** name the behaviors the system needs. These become the
  executable spec — test names are the table of contents.
- **Implementation:** behavior gaps drive execution order. Each implementation
  increment maps to one or more behavior statements. Work that can't be traced
  to a behavior is unanchored.
- **Verification:** completion means passing evidence for each claimed behavior,
  not just "tests pass." Report behavior coverage, not command output.

## Discipline

These shape how behaviors are written and maintained. Each exists because the
default — what you'd do without it — produces tests that don't serve as
specification.

**Behavior before mechanics.** Ask "what should this do?" before "how to test
it?" The behavior question surfaces the right test; the mechanics question
optimizes the wrong one.

**Sentence-named tests.** Test names read as behavior statements — they form
a readable specification of the module. When test names are labels
(`testAuth`, `handleError`), the spec disappears and only the code remains.

> *Bad:* `test_parse_config`, `testUserAuth`, `it('handles errors')`
> *Good:* `rejects_expired_tokens_with_401`, `falls_back_to_default_when_config_missing`, `it('preserves line numbers in error messages')`

**Given/When/Then.** Every test has three phases: setup, action, assertion.
Given establishes context (only setup, no behavior). When performs one action
(the behavior trigger). Then verifies observable outcomes (not internals).
Mixing phases makes tests brittle and hard to read.

**One behavior per test.** Tests containing multiple behaviors mask which
behavior broke. Split them. Three small tests that each prove one thing are
better than one test that proves three.

**Behavior drives priority.** The next test is chosen by behavior gap
importance, not by what's convenient to test. This keeps the spec growing
toward complete coverage of what matters.

**Implementation maps to behavior.** Every implementation increment connects
to one or more behavior statements. If you can't name which behavior a code
change advances, the change is unanchored. This traceability runs the full
chain — from specification through implementation to verification evidence.

### Example: Complete Scenario

```python
def test_rejects_expired_tokens_with_401():
    """Expired authentication tokens return 401, not a silent fallback."""
    # Given
    token = create_token(expires_at=one_hour_ago)
    client = create_test_client()

    # When
    response = client.get("/api/protected", headers=auth_header(token))

    # Then
    assert response.status_code == 401
    assert "expired" in response.json()["error"].lower()
```

The test name is the specification line. The Given/When/Then structure makes
the behavior contract visible. The assertion checks observable outcome
(status code, error message), not internal state.

### Example: Bad to Good

```python
# Before — testing implementation
def test_token_validation():
    validator = TokenValidator()
    validator._cache = {}
    result = validator._parse_jwt(expired_token)
    assert result.internal_state == "expired"

# After — testing behavior
def test_rejects_expired_tokens_with_401():
    token = create_token(expires_at=one_hour_ago)
    response = client.get("/api/protected", headers=auth_header(token))
    assert response.status_code == 401
```

The before test breaks if internals change. The after test survives any
refactor that preserves the behavior.

## Procedures

### identify-next-behaviour

1. List existing behaviors from test names — these are the current spec.
2. Extract needed behaviors from acceptance criteria, issue description, or
   module purpose.
3. Compute missing behaviors (needed minus existing).
4. Rank by importance:
   - **Happy path first** — establishes the primary contract; error paths
     refine it.
   - **Core before edge** — the common case matters more than the rare case.
   - **Foundation before dependents** — behaviors that others build on come
     first.
   - **User-visible impact first** — behaviors users encounter directly before
     internal mechanics.
5. Pick the highest-priority missing behavior that can be expressed as one
   sentence.

### write-behaviour

1. Name the behavior as a sentence-style test name.
2. Write Given/When/Then structure.
3. **Red:** run and confirm failure. A test that passes immediately proves
   nothing — it means either the behavior already exists or the test doesn't
   check what it claims.
4. **Green:** implement minimal code to pass. Resist the urge to generalize.
5. **Refactor** while keeping the full suite green.

### map-behaviour-to-implementation

1. For each planned implementation step, name the behavior statements it
   advances.
2. New RED tests during implementation must correspond to named behavior
   scenarios.
3. Reject implementation steps that can't be traced to behavior requirements —
   they indicate scope creep or missing specifications.

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
matters, fix the implementation. If it doesn't, delete the test.

## Triggers

- Writing tests
- Starting a new module
- Implementing issue acceptance criteria
- Diagnosing test failures
- Deciding what to implement next
- Mapping plan tasks to behavior contract
- Validating completion claims against behavior

## Corruption Modes

**Testing implementation.** Asserting internal state, private methods, or
implementation details instead of observable behavior.
*Recognition:* Your test would break if you refactored the internals without
changing behavior. Your assertions reference private fields, internal data
structures, or intermediate state that users never see.

**Vague names.** Test names that don't describe the behavior.
*Recognition:* You cannot reconstruct what the module does by reading test
names alone. Names are labels (`test_1`, `test_auth`) rather than behavior
statements.

**Missing Given.** Tests with unclear setup context — you can't tell what
conditions the behavior requires.
*Recognition:* Reading the test, you cannot answer "under what circumstances
does this behavior apply?" Setup is implicit or buried in distant fixtures.

**Multi-behavior tests.** Tests that verify multiple behaviors in one case.
*Recognition:* A test failure could mean any of several behaviors broke. The
test has multiple When/Then sequences or assertions testing different things.

**Test hoarding.** Retaining tests for behaviors the system no longer needs.
*Recognition:* Tests pass but describe behavior nobody requires. You're afraid
to delete them because "they might catch something."

**Framework overthinking.** Choosing or debating test frameworks instead of
writing behaviors.
*Recognition:* You've spent time evaluating frameworks, configuring runners, or
comparing assertion libraries — but no behavior has been specified yet.

**Contract dropoff.** Treating BDD as a specification phase that ends when
coding begins.
*Recognition:* You wrote behavior specs, then implemented without checking work
against them. At completion, you report "tests pass" without citing which
behaviors have evidence.

## Principles

**Words shape thinking.** Behavior vocabulary — "should," "when," "given" —
improves test design by forcing you to think about what the system does rather
than how it's built. The language is the method.

**Specification, not verification.** Tests are living executable
specifications. Their primary value is documenting what the system does, not
just catching regressions. A test suite you can read as a spec is worth more
than one you can only run.

**Delete freely.** Obsolete behavior tests should be removed. Tests that
describe behaviors nobody needs are noise, not safety.

**Single pipeline.** BDD and TDD are complementary stages in one flow, not
alternatives. BDD decides what to test; TDD executes the red-green-refactor
cycle for each behavior.

## Cross-References

- `begin`: session-level prioritization and execution sequencing.
- `issue-craft`: acceptance-criteria rigor, issue decomposition, and behavior traceability.
- `test-first`: executes RED-GREEN-REFACTOR for BDD-defined behaviors.
- `verification-before-completion`: requires fresh evidence for behavior-level completion claims.
