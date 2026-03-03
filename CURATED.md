# Curated Skills (v0.1)

Groundwork v0.1 curates one middle layer: **Superpowers**.

Kata Orchestrator was evaluated as a potential replacement middle layer.
Recommendation for v0.1 is to use Superpowers in the middle because:
- clearer immediate compatibility path across Claude Code, Codex, and OpenCode
- strong execution guardrails (TDD, debugging discipline, verification discipline)
- clean composition with Groundwork skills without requiring Kata's `.planning` operating model

Pipeline invariant:
- Groundwork does not expose parallel BDD-vs-TDD pathways.
- `bdd` defines the behavior contract.
- Curated execution/verification skills implement and validate that same contract.

Canonical contract: [docs/architecture/pipeline-contract.md](docs/architecture/pipeline-contract.md)

## Curated Source Manifest

Machine source of truth:
- [`manifests/curation.v1.toml`](manifests/curation.v1.toml)

Pinned source in v0.1:
- `obra/superpowers` @ `e4a2375cb705ca5800f0833528ce36a3faf9017a`

## Included Skills and Failure Modes

## `brainstorming`
Failure mode prevented:
- building before design is clarified and pressure-tested

## `writing-plans`
Failure mode prevented:
- vague implementation plans that are not directly executable
- plan steps that are not traceable to declared behaviors

## `subagent-driven-development`
Failure mode prevented:
- context drift and quality decay across long execution loops

## `test-driven-development`
Failure mode prevented:
- production code written before a failing test exists
- implementation detached from the declared behavior contract

## `systematic-debugging`
Failure mode prevented:
- random symptom-fixing without root-cause analysis

## `verification-before-completion`
Failure mode prevented:
- claiming success without fresh verification command evidence
- claiming success without explicit behavior-contract evidence

## `requesting-code-review`
Failure mode prevented:
- unstructured or skipped review before completion

## `receiving-code-review`
Failure mode prevented:
- performative agreement without concrete correction loops

## Explicit Exclusions in v0.1

## Superpowers `finishing-a-development-branch`
Reason:
- overlaps with Groundwork's own `land` completion discipline.

## Kata as primary middle layer
Reason:
- strong integrated workflow, but deferred to future version pending tighter cross-agent packaging parity in Groundwork's required install surface.
