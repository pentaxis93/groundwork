---
name: bdd
description: Behaviour-driven development for writing and maintaining executable behavior specifications across specification, implementation, and verification. Use when deciding what to test next, structuring tests with Given/When/Then, mapping behavior to implementation work, or diagnosing failing behavior contracts.
---

# Behaviour-Driven Development

BDD reframes testing as behavior specification: what the system should do,
under what context, with observable outcomes.

References:
- [Dan North: Introducing BDD](https://dannorth.net/introducing-bdd/)
- [language patterns](references/language-patterns.md)

## Goal
Drive development through sentence-named behaviors and Given/When/Then
structured scenarios, and keep that behavior contract intact through execution
and verification.

## Lifecycle Role
BDD is a cross-cutting contract, not a planning-only step.

- **Specification:** define required behaviors in executable language.
- **Implementation:** drive execution order from behavior gaps; each implemented test maps to a behavior scenario.
- **Verification:** confirm shipped outcomes against behavior statements, not only tool output.

## Constraints
- `behaviour-not-test`: ask "what should this do?" before "how to test?"
- `sentences-not-labels`: test names read as behavior statements.
- `given-when-then`: every case contains setup, action, assertion phases.
- `native-tooling`: use language-native test framework.
- `one-behaviour-per-test`: split cases containing multiple behaviors.
- `contract-continuity`: preserve behavior traceability from spec through completion.

## Requirements
- `name-reads-as-spec`: test names form readable module specification.
- `given-establishes-context`: setup only.
- `when-performs-one-action`: single behavior trigger.
- `then-verifies-outcome`: assert observable outcomes, not internals.
- `behaviour-drives-priority`: next test chosen by behavior gap importance.
- `behaviour-maps-to-work`: every implementation increment maps to one or more behavior statements.

## Procedures

### identify-next-behaviour
1. List existing behaviors from test names.
2. Extract needed behaviors from AC/issue/module purpose.
3. Compute missing behaviors.
4. Rank by importance:
- happy path before error path
- core before edge
- dependency foundation before dependents
- user-visible impact first
5. Pick highest-priority behavior that can be expressed as one sentence.

### write-behaviour
1. Name behavior as sentence-style test.
2. Write Given/When/Then structure.
3. Red: run and confirm failure.
4. Green: implement minimal code to pass.
5. Refactor while keeping full suite green.

### map-behaviour-to-implementation
1. For each planned implementation step, link the behavior statements it advances.
2. Ensure RED tests introduced during implementation correspond to named behavior scenarios.
3. Reject implementation steps that cannot be traced to behavior requirements.

### verify-behaviour-contract
1. For each claimed completed behavior, cite passing evidence.
2. For failing/deferred behaviors, mark explicit gaps.
3. Report completion status as behavior coverage, not only command success.

### evaluate-existing-tests
When a test fails after change, classify failure as one of:
- `bug_introduced`: fix implementation.
- `behaviour_moved`: move/redirect test.
- `behaviour_obsolete`: delete outdated test.

## Triggers
- writing tests
- starting new module
- implementing issue acceptance criteria
- diagnosing test failures
- deciding what to implement next
- mapping plan tasks to behavior contract
- validating completion claims against behavior

## Corruption Modes
- `testing-implementation`: asserting internals instead of behavior.
- `vague-names`: non-descriptive test names.
- `missing-given`: unclear setup context.
- `multi-behaviour-tests`: multiple behaviors in one case.
- `test-hoarding`: retaining obsolete tests.
- `framework-over-thinking`: choosing framework over method.
- `contract-dropoff`: treating BDD as complete before execution starts.

## Principles
- `words-shape-thinking`: behavior vocabulary improves test design.
- `specification-not-verification`: tests are living executable spec.
- `delete-freely`: obsolete behavior tests should be removed.
- `single-pipeline`: BDD and TDD are complementary stages in one flow, not alternatives.

## Cross-References
- `planning`: session-level prioritization and execution sequencing.
- `issue-craft`: acceptance-criteria rigor and issue decomposition.
- `superpowers/writing-plans`: decomposes implementation while preserving behavior traceability.
- `superpowers/test-driven-development`: executes RED-GREEN-REFACTOR for BDD-defined behaviors.
- `superpowers/verification-before-completion`: requires fresh evidence for behavior-level completion claims.
