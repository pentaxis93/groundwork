# Groundwork Workflow (v0.1)

Groundwork is a full-path methodology from first problem framing to shipped change.

This workflow intentionally combines:
- **Original Groundwork skills** for cognitive discipline, research discipline, behavioral contracts, decomposition, and closure.
- **Upstream execution skills** for high-discipline implementation and verification.

## Philosophy

Start from what the work must enable, not from inherited implementation patterns.

Groundwork exists to prevent common AI coding failure modes:
- accepting the prompt frame uncritically
- coding before behavior is defined
- vague or non-executable task decomposition
- ad-hoc execution without discipline
- completion claims without fresh evidence
- shipping without closure hygiene

## Cross-Cutting Thread: Behavioral Contract (`bdd`)

`bdd` is not a one-time pre-implementation step.

`bdd` runs through the full pipeline:
- **Specification**: define behavior in Given/When/Then terms.
- **Execution**: `test-driven-development` implements those behaviors via RED-GREEN-REFACTOR.
- **Verification**: completion evidence is checked against those behaviors, not just command success.

Handoff rules:
1. `bdd -> writing-plans`: every plan item maps to one or more behavior statements.
2. `bdd -> test-driven-development`: each RED test corresponds to a named behavior scenario.
3. `bdd -> verification-before-completion`: completion claims require behavior-level evidence.
4. `bdd -> land`: closure records behavior coverage status and remaining gaps, if any.

Canonical contract: [docs/architecture/pipeline-contract.md](docs/architecture/pipeline-contract.md)

## Methodology Orientation

### `using-groundwork` (original)
Trigger: session start, task initiation, or any moment requiring methodology orientation.

Purpose:
- activate the full skill system as one connected methodology
- show how skills relate and hand off to each other
- provide routing guidance for which skill fits which situation

Prevents:
- using skills by keyword match without methodology context
- treating groundwork as isolated tools rather than a connected pipeline

## Phases

## 1. Foundation

### `ground` (original)
Trigger: before creating designs/specs/architectures/processes.

Purpose:
- establish normative constraints (what must be true)
- separate requirements from inherited conventions
- reconstruct solutions from verified constraints

Prevents:
- anchoring on existing implementation
- category inheritance and pattern copy-paste

### `research` (original)
Trigger: when reliable external evidence is needed for decisions.

Purpose:
- run systematic multi-source research with citation discipline
- synthesize findings into actionable constraints for design and execution

Prevents:
- single-source conclusions and stale assumptions
- unverified claims driving architecture or implementation choices

## 2. Specification

### `bdd` (original, cross-cutting)
Trigger: when defining or refining behavior expectations.

Purpose:
- define behavior as executable expectations (Given/When/Then style)
- maintain a stable behavior contract from design through verification

Prevents:
- implementation-first drift
- contract drift between spec, implementation, and verification

## 3. Decomposition

### `planning` (original)
Trigger: selecting session-sized work from issue graph.

Purpose:
- choose the next meaningful unit of progress
- avoid over-scoping a single session

Prevents:
- context overrun from oversized tasks
- random task switching

### `issue-craft` (original)
Trigger: creating/refining task/epic/bug/spike issues.

Purpose:
- create agent-executable issues with binary acceptance criteria
- preserve dependency clarity

Prevents:
- ambiguous issue descriptions requiring clarifications mid-execution
- non-verifiable "done" statements

## 4. Execution + Verification (upstream)

Groundwork v0.1 includes these upstream Superpowers skills:
- `brainstorming`
- `writing-plans`
- `subagent-driven-development`
- `test-driven-development`
- `systematic-debugging`
- `verification-before-completion`
- `requesting-code-review`
- `receiving-code-review`

Contract model:
- Groundwork skills define constraints and behavior contract.
- Execution skills implement and verify that contract.

Prevents:
- coding without design pressure-testing
- task execution drift
- skipping TDD loops against the behavior contract
- non-root-cause debugging
- completion claims without command evidence and behavior mapping
- shallow/unstructured code review handling

## 5. Completion

### `land` (original)
Trigger: merge-and-close completion events (`land`, `merge and close`, `ship it`).

Purpose:
- close the branch and issue lifecycle cleanly
- ensure shipped changes are discoverable and traceable
- preserve what behavior coverage shipped and what remains

Prevents:
- half-finished closure (merged but issue still stale, branches left behind)

## Original vs Upstream Transparency

Groundwork is explicit about ownership:
- Original skills are maintained in this repository.
- Upstream skills are fetched from their repositories at pinned revisions.

See:
- `CURATED.md` for inclusion rationale
- `ATTRIBUTION.md` for source/license/pinning details
- `manifests/curation.v1.toml` for machine-readable curation

## Install and Run

```bash
groundwork init
```

Then run your task through one coherent pipeline:
1. Ground with `ground`
2. Define/refine behavior contract with `bdd`
3. Decompose with `planning` + `issue-craft`
4. Execute and verify the behavior contract with execution skills
5. Complete with `land`
