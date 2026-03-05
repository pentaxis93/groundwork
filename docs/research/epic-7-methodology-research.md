# Epic 7: Decomposition Methodology Research

**Date:** 2026-03-05
**Status:** Complete
**Requested by:** Governance Committee
**Conducted by:** Research agent (Claude Code)

## Purpose

Groundwork's decomposition stage has three skills with overlapping scope and conflicting sovereignty models. This research determines the right skill topology by examining established methodologies, and determines whether "verified user benefit" can be formalized as a pipeline artifact.

The report is structured as four deliverables:
- **D1:** Decomposition Methodology Landscape Map
- **D2:** Verified User Benefit Practices Inventory
- **D3:** Decomposition Stage Topology Recommendation
- **D4:** Full BDD-to-Land Skill Inventory

---

## Background: The Problem

Groundwork's pipeline flows through five stages: frame constraints, define behavior, decompose, execute and verify, land. The decomposition stage currently contains three skills that overlap:

**`plan`** (core) converges from exploration to a decision-complete implementation design. Every approach, interface, data flow, and edge case is resolved. The implementer makes zero design decisions but retains full implementation autonomy. This is sovereignty-respecting: it produces a design contract.

**`writing-plans`** (curated, from obra/superpowers) translates a design into bite-sized implementation steps with exact file paths, complete code blocks, exact commands, and expected output at 2-5 minute granularity. The executing agent transcribes predetermined steps. This is sovereignty-violating: it collapses the boundary between decomposition and execution.

**`issue-craft`** (core) produces agent-executable issues with binary acceptance criteria, dependency graphs, and session-sized scope. Issues define what must be achieved, not how. This is sovereignty-respecting: it produces acceptance contracts.

The skills have confusing naming (you cannot "plan" and then "write a plan"), different granularity (design vs. step-by-step script vs. session-sized work unit), and incompatible sovereignty models. The pipeline needs a clear topology where each skill has a distinct domain and interfaces via boundary contract.

Additionally, the pipeline ends at "code merged and issue closed." It does not verify that shipped behavior delivers user benefit.

---

## D1: Decomposition Methodology Landscape Map

### 1.1 FDD (Feature-Driven Development)

**Source:** Jeff De Luca (1997), codified in Palmer & Felsing, *A Practical Guide to Feature-Driven Development* (2002). Process model: [featuredrivendevelopment.com](http://www.featuredrivendevelopment.com/files/FDD%20Process%20Model%20Diagram.pdf). De Luca 2007 interview: [it-agile.de](https://www.it-agile.de/fileadmin/docs/FDD-Interview_en_final.pdf).

**Stages between design and coding:**

FDD defines five processes. The relevant ones are Process 4 ("Design by Feature") and Process 5 ("Build by Feature"), which form the most formally documented design-to-work-unit handoff in the methodologies examined.

- **Design by Feature:** A Chief Programmer selects a feature set, identifies participating classes and class owners, leads a design walkthrough, and produces a **design package**.
- **Build by Feature:** Class Owners implement their classes according to the design package, unit test, and submit for integration.

**The design package** (handoff artifact) contains:
- A covering memo integrating the package so it stands alone
- Referenced requirements documents
- Design alternatives (when applicable)
- Sequence diagrams showing class collaboration
- Updated object model with new/updated classes, methods, and attributes
- Class and method prologues (pseudo-code for signatures and behavior)
- A to-do task list per team member's affected classes

**Sovereignty analysis:** FDD has a nuanced sovereignty model. The design package *partially* prescribes: sequence diagrams specify which classes collaborate and in what order, and method prologues define signatures. But **Class Owners have implementation sovereignty within their class boundary.** De Luca stated explicitly: "One of the fundamental principles of OO is encapsulation; how a class does what it does is private and internal to that class... humans naturally encapsulate."

The sovereignty boundary is the **class interface**, not the feature boundary. The design specifies collaborations and contracts; the class owner decides internal implementation.

**Relevance to AI agents:** Strong. The design package is highly structured and machine-readable. Class ownership maps naturally to agent task boundaries. The Chief Programmer role maps to an orchestrating agent or the `plan` skill itself.

### 1.2 BDD / ATDD / Specification by Example

**Sources:** Dan North, "Introducing BDD" (2006): [dannorth.net](https://dannorth.net/blog/introducing-bdd/). Cucumber BDD docs: [cucumber.io/docs/bdd](https://cucumber.io/docs/bdd/). Gaspar Nagy & Seb Rose, *The BDD Books* and *Effective BDD* (Manning): [bddbooks.com](https://bddbooks.com/). Gojko Adzic, *Specification by Example* (2011, Jolt Award): [gojko.net](https://gojko.net/books/specification-by-example/). Matt Wynne, Example Mapping: [cucumber.io/blog/bdd/example-mapping-introduction](https://cucumber.io/blog/bdd/example-mapping-introduction/).

**Stages between design and coding:**

BDD defines three practices (codified by Nagy & Rose):

1. **Discovery** -- Structured conversations to build shared understanding. The primary technique is **Example Mapping** (Matt Wynne): colored cards for stories (yellow), rules (blue), examples (green), questions (red). Output: shared understanding, business rules, concrete examples, and unresolved questions. Time-boxed to ~25 minutes.

2. **Formulation** -- Express examples as Given/When/Then scenarios in Gherkin. Output: executable specifications readable by humans and machines. The formulation step is where natural-language examples become structured, unambiguous behavioral contracts.

3. **Automation** -- Connect scenarios to the system under test. Output: living documentation that verifies behavior continuously.

**ATDD** (Acceptance Test-Driven Development) is the predecessor pattern. Its "Three Amigos" collaboration model (business representative, developer, tester) ensures three perspectives shape the specification. This maps to groundwork's human/governance/agent triad.

**Specification by Example** (Adzic, 2011) synthesized practices from 50+ teams into a seven-step flow: deriving scope from goals, specifying collaboratively, illustrating with examples, refining the specification, automating without changing the specification, validating frequently, and evolving a documentation system.

**Sovereignty analysis:** BDD has the **cleanest behavioral contract** of any methodology examined. Given/When/Then scenarios deliberately avoid implementation details. They specify WHAT the system should do from the user's perspective. The implementer has full sovereignty over HOW. This is not accidental -- North's original design intention was to shift vocabulary from "test" to "behavior" precisely to prevent implementation thinking from contaminating the specification.

**Gap:** None of BDD, ATDD, or SBE explicitly address the design-to-work-unit decomposition. They go from behavioral specification to implementation without a formalized decomposition stage. Discovery produces understanding and examples; Formulation produces scenarios; Automation produces tests. The question "how do you break a large feature into session-sized work units?" is not in BDD's scope.

**Relevance to AI agents:** Excellent. Gherkin scenarios are structured, unambiguous, machine-parseable, and directly verifiable. Example Mapping output (rules + examples) is equally machine-friendly. These are ideal handoff artifacts between a design agent and an implementation agent.

### 1.3 DDD (Domain-Driven Design)

**Sources:** Eric Evans, *Domain-Driven Design* (2004). DDD Reference: [domainlanguage.com](https://www.domainlanguage.com/wp-content/uploads/2016/05/DDD_Reference_2015-03.pdf). Martin Fowler on Bounded Context: [martinfowler.com/bliki/BoundedContext.html](https://martinfowler.com/bliki/BoundedContext.html). Alberto Brandolini, Event Storming: [eventstorming.com](https://www.eventstorming.com/). ddd-crew Context Mapping: [github.com/ddd-crew/context-mapping](https://github.com/ddd-crew/context-mapping).

**Stages between design and coding:**

DDD separates **Strategic Design** (bounded contexts, context maps) from **Tactical Design** (aggregates, entities, value objects, domain services). Strategic design determines what systems exist and how they relate. Tactical design determines internal structure within a bounded context.

**Context Mapping patterns** define the sovereignty relationship between contexts:

| Pattern | Sovereignty Model |
|---------|-------------------|
| **Anti-Corruption Layer (ACL)** | Full sovereignty on both sides; translation at boundary |
| **Open Host Service + Published Language** | Upstream publishes stable protocol; downstream adapts freely |
| **Shared Kernel** | Partial sovereignty; changes require agreement |
| **Conformist** | Downstream surrenders sovereignty to upstream model |
| **Customer-Supplier** | Downstream has influence but upstream sets priorities |

**Ubiquitous Language** serves as the interface contract within a bounded context. At boundaries, **Published Language** serves as the cross-context contract.

**Event Storming** (Brandolini) provides the workshop technique for discovering bounded contexts. Three levels progressively narrow scope: Big Picture (domain exploration) -> Process Modeling (detailed flow) -> Software Design (aggregates, commands, events).

**Sovereignty analysis:** DDD provides the most sophisticated sovereignty model. The ACL pattern explicitly defends context sovereignty: Evans wrote that "a large interface with an upstream system can eventually overwhelm the intent of the downstream model altogether." The OHS+PL pattern is the cleanest contract-based interface: the upstream publishes a stable protocol, and the downstream adapts freely.

Evans himself reflected in 2009 that building blocks (entities, value objects) were over-emphasized by practitioners. Bounded contexts and anti-corruption layers are the strategic patterns that matter most -- precisely because they are sovereignty mechanisms.

**Relevance to AI agents:** Excellent at the bounded context level. Each context can be an independent agent domain with a Published Language interface. The ACL pattern maps to agent-to-agent translation layers. DDD is primarily about architectural decomposition, not work-unit decomposition, but its sovereignty model (each bounded context owns its domain) is directly applicable to skill boundary design.

### 1.4 SAFe BDD Extension

**Sources:** SAFe BDD: [scaledagileframework.com/behavior-driven-development](https://scaledagileframework.com/behavior-driven-development/). Ken Pugh, SAFe BDD workshops: [kenpugh.com](https://kenpugh.com/).

SAFe applies BDD across a four-tier hierarchy (Epic -> Capability -> Feature -> Story) with the **Triad** collaboration model (Product Owner + Developer + Tester, mapped from BDD's Three Amigos).

**Two-dimensional story detailing:**
- **Horizontal:** More acceptance criteria per story (broadening scope)
- **Vertical:** More tests per criterion (deepening verification)

The decomposition from feature to story involves splitting feature-level acceptance criteria into story-sized vertical slices, each with its own BDD scenarios. This is the closest any scaled framework comes to formalizing the design-to-work-unit decomposition.

**Sovereignty analysis:** Story-level BDD preserves sovereignty -- each story specifies behavior, not implementation. Feature-to-story decomposition requires architectural judgment about where to draw slice boundaries. The Triad ensures no single perspective dominates the decomposition.

### 1.5 Supplementary Methodologies

**Example Mapping** (Matt Wynne): The most structured discovery technique. Yellow/blue/green/red cards (Story/Rules/Examples/Questions) produce highly structured, machine-parseable output in a time-boxed session (~25 minutes). Feeds directly into BDD Formulation. Source: [cucumber.io/blog/bdd/example-mapping-introduction](https://cucumber.io/blog/bdd/example-mapping-introduction/).

**User Story Mapping** (Jeff Patton, 2005): Backbone (user journey) + vertical decomposition into stories + horizontal release slicing. A what-to-build tool, not a how-to-build specification. Patton explicitly says stories are "invitations to conversation," not contracts. Source: *User Story Mapping* (O'Reilly, 2014).

**Vertical Slicing** (Jimmy Bogard et al.): Self-contained end-to-end feature slices. Bogard: "Each of our vertical slices can decide for itself how to best fulfill the request." Maximum implementation sovereignty -- the slice boundary defines the work unit, and everything inside the boundary is the implementer's domain.

**Job Stories** (Alan Klement): `When [situation], I want [motivation], so I can [outcome]`. Maximum sovereignty -- describes only the situation and desired outcome, prescribes nothing about how to get there. Source: [jtbd.info](https://jtbd.info/replacing-the-user-story-with-the-job-story-af7cdee10c27).

### 1.6 Cross-Methodology Synthesis

**The sovereignty spectrum:**

```
MOST PRESCRIPTIVE                                         MOST SOVEREIGN
writing-plans --- FDD --- SAFe BDD --- BDD/SBE --- DDD --- Vertical Slicing --- Job Stories
```

`writing-plans` sits beyond even FDD in prescriptiveness. FDD's design package specifies class collaborations and method signatures but leaves internal implementation to Class Owners. `writing-plans` prescribes the exact code to write in exact files -- it has no sovereignty boundary at all.

**Three distinct concerns at the design-to-code boundary:**

All methodologies reveal three concerns:

1. **Problem Understanding (WHY)** -- Impact Mapping, Job Stories, BDD Discovery
2. **Behavioral Specification (WHAT)** -- BDD Formulation, DDD Published Language, FDD design package interfaces
3. **Implementation (HOW)** -- FDD Build by Feature, BDD Automation, Vertical Slicing internals

**The cleanest sovereignty partition exists between concerns 2 and 3** -- between behavioral specification and implementation. This is where BDD, FDD, and DDD all draw the line, though each does so differently:
- BDD: Given/When/Then scenarios (behavior) | implementation code
- FDD: Design package interfaces (contracts) | class-internal implementation
- DDD: Published Language (boundary protocol) | bounded context internals

This three-concern model directly maps to groundwork's decomposition skills:
- `bdd` handles concern 1-2 (discovery + specification)
- `plan` handles the design dimension of concern 2 (resolving how concerns 1-2 map to an implementation approach)
- `issue-craft` handles the decomposition of concern 2 into work units
- `writing-plans` inappropriately invades concern 3

---

## D2: Verified User Benefit Practices Inventory

### 2.1 The Layered Model

Research reveals that "verified user benefit" is not a single concern but a **five-layer stack** with decreasing formalizability:

| Layer | Question | Formalizability | Key Practice |
|-------|----------|----------------|--------------|
| 1 | Did we deploy successfully? | Fully automatable | DORA metrics, deployment records |
| 2 | Did deployment cause harm? | Mostly automatable | Feature flags, RUM, auto-rollback |
| 3 | Did the feature produce expected metric movement? | **Formalizable with lightweight artifacts** | Test Cards, HDD, A/B testing |
| 4 | Did users actually benefit? | Requires human judgment | Surveys, interviews, NPS |
| 5 | Should we have built this at all? | Fundamentally human | Impact Mapping, Customer Development |

Layers 1-2 are already well-served by existing DevOps practices. Layer 3 is the opportunity zone for pipeline integration. Layers 4-5 require human sensemaking that cannot be reduced to structured artifacts a pipeline engine tracks.

### 2.2 Practice-by-Practice Analysis

#### Hypothesis-Driven Development (HDD)

**Source:** Jeff Gothelf & Josh Seiden, *Lean UX* (O'Reilly, 2013, 3rd ed. 2021). [jeffgothelf.com](https://jeffgothelf.com/).

**Artifact:** Hypothesis statement in canonical form:
```
We believe that [capability]
will result in [measurable outcome]
We will know we are right when [metric] [threshold]
```

**Pipeline trackability:** HIGH. The hypothesis statement is structured, has named fields, and produces a binary verification result (metric met or not). A pipeline engine could track hypothesis → deployment → measurement → result.

**Automation vs. human judgment:** Hypothesis formation is human. Measurement can be automated. Interpretation of whether the metric movement represents genuine benefit is human.

#### Strategyzer Test Card / Learning Card

**Source:** Alexander Osterwalder & David Bland, *Testing Business Ideas* (Wiley, 2019). [strategyzer.com/library/test-card](https://www.strategyzer.com/library/test-card).

**Artifact:** Test Card (pre-deployment):

| Field | Content |
|-------|---------|
| Hypothesis | We believe that... |
| Test | To verify, we will... |
| Metric | We will measure... |
| Criteria | We are right if... |

Learning Card (post-deployment):

| Field | Content |
|-------|---------|
| Hypothesis | (from Test Card) |
| Observation | What actually happened |
| Learnings | What we learned |
| Decisions | What to do next |

**Pipeline trackability:** VERY HIGH. Four fields that directly map to a pipeline workflow. The most compact, most structured hypothesis-to-learning artifact found in this research. Test Card is the pre-deployment declaration; Learning Card is the post-deployment record. Together they form a complete declare-measure-learn loop.

**Automation vs. human judgment:** Test Card creation requires human judgment (what to test, what metric, what threshold). Metric measurement can be automated. Learning Card's "observation" can be auto-populated from metrics. "Learnings" and "decisions" require human judgment.

#### Feature Flags with Measurement

**Source:** LaunchDarkly Guarded Releases: [launchdarkly.com](https://launchdarkly.com/product/feature-flags/). Pete Hodgson, "Feature Toggles": [martinfowler.com/articles/feature-toggles.html](https://martinfowler.com/articles/feature-toggles.html).

**Artifact:** Flag configuration with metric bindings:
- Flag definition (name, variants, targeting rules)
- Metric binding (which metrics to monitor per flag)
- Rollout plan (percentage ramp, canary criteria)
- Measurement results (per-variant metric comparison)

**Pipeline trackability:** VERY HIGH. LaunchDarkly's Guarded Releases already implement the full "declare expected metric -> measure actual -> compare -> auto-act" loop in production. This is the most mature, production-proven practice for automated post-deployment verification.

**Automation vs. human judgment:** Almost fully automatable for layers 1-3. Flag configuration, metric monitoring, statistical comparison, and auto-rollback on regression can all be automated. Deciding what metrics matter and what thresholds to set is human.

#### A/B Testing / Experimentation Platforms

**Source:** Ron Kohavi, Diane Tang, Ya Xu, *Trustworthy Online Controlled Experiments* (Cambridge, 2020). Optimizely: [optimizely.com](https://www.optimizely.com/). Google experimentation: Kohavi et al., "Online Controlled Experiments at Large Scale" (KDD 2013).

**Artifact:** Experiment definition + results:
- Hypothesis, variants, assignment method
- Primary and guardrail metrics
- Sample size calculation
- Results with statistical confidence bounds (p-values, confidence intervals)

**Pipeline trackability:** VERY HIGH. Produces machine-readable experiment results with statistical confidence. The gold standard for causal verification. However, requires sufficient traffic volume -- not applicable to all features or all projects.

**Automation vs. human judgment:** Experiment setup requires human decisions (what to test, metrics, significance threshold). Execution and statistical analysis are fully automated. Interpreting results and deciding next actions is human.

#### Impact Mapping

**Source:** Gojko Adzic, *Impact Mapping* (2012). [impactmapping.org](https://www.impactmapping.org/).

**Structure:** Goal -> Actor -> Impact -> Deliverable. Connects features to measurable business outcomes through a chain of reasoning.

**Pipeline trackability:** LOW for the mapping itself (it's a thinking tool), MODERATE for the goal metrics. Impact maps do not produce structured verification artifacts -- they produce strategic alignment. The goals at the top of the map could be tracked as metrics, but the map itself is a planning artifact, not a verification artifact.

#### Continuous Discovery (Teresa Torres)

**Source:** Teresa Torres, *Continuous Discovery Habits* (2021). [producttalk.org](https://www.producttalk.org/).

**Artifact:** Opportunity Solution Tree (OST) -- desired outcome at root, opportunities as branches, solutions as leaves, with assumption tests for validation.

**Pipeline trackability:** LOW. The OST is a sensemaking tool for product teams. It produces knowledge and alignment, not structured verification artifacts. Assumption tests can be tracked, but they are pre-build validation, not post-deployment verification.

#### DORA Metrics / Value Stream Mapping

**Source:** Nicole Forsgren, Jez Humble, Gene Kim, *Accelerate* (2018). DORA: [dora.dev](https://dora.dev/).

**Metrics:** Deployment frequency, lead time for changes, change failure rate, time to restore service.

**Pipeline trackability:** VERY HIGH for operational health. These are deployment-level metrics (layers 1-2), not feature-level benefit metrics (layer 3). They answer "is our delivery pipeline healthy?" not "did this feature help users?"

#### Lean Startup Build-Measure-Learn

**Source:** Eric Ries, *The Lean Startup* (2011).

**Pipeline trackability:** LOW as a formal artifact producer. Build-Measure-Learn is a strategic loop, not a structured artifact. It influenced HDD and the Test Card format but does not itself define trackable artifacts beyond "pivot or persevere" decisions.

### 2.3 Synthesis: What a Pipeline Can Track

Three practices stand out for immediate pipeline integration:

1. **Strategyzer Test Card / Learning Card** -- Most structured, most compact. Four fields map directly to a pipeline workflow. Recommended as the primary artifact format.

2. **Feature Flags with Measurement** -- Most production-proven. Full automated loop already exists in commercial products. Applies when the feature is flag-gated.

3. **A/B Testing** -- Most statistically rigorous. Applies when traffic volume supports controlled experiments.

**What AI agents can and cannot do in this territory:**

| Can | Cannot |
|-----|--------|
| Configure feature flags | Form genuinely novel hypotheses |
| Set up experiments | Conduct customer interviews |
| Monitor metrics | Interpret whether "passing" metric = real benefit |
| Calculate DORA scores | Make pivot-or-persevere decisions |
| Detect metric anomalies | Detect metric gaming (Goodhart's Law) |
| Auto-populate measurement results | Determine if the right thing was measured |
| Generate result summaries | Assign meaning to unexpected results |

**Decision Gate G3 resolution:** The territory is **not** entirely organizational. Layer 3 (expected metric movement) is formalizable with lightweight artifacts -- specifically the Test Card / Learning Card format. Layers 1-2 are already automatable. Layers 4-5 require human judgment and should not be forced into structured artifacts.

**Recommendation:** Formalize layer 3 with a lightweight, optional skill. Do not attempt to formalize layers 4-5. The pipeline should track "we declared an expected outcome, we measured, here is what happened" without claiming to track "users actually benefited" -- that interpretation remains human.

---

## D3: Decomposition Stage Topology Recommendation

### 3.1 The Core Finding

Every methodology examined draws the sovereignty boundary between **behavioral specification** (WHAT the system must do) and **implementation** (HOW it does it). No methodology prescribes exact code for the implementer to type. Even FDD, the most prescriptive, stops at class interfaces and method signatures -- internal implementation is the Class Owner's domain.

`writing-plans` crosses this line. It prescribes exact file paths, complete code blocks, exact commands, and expected output. Its own framing says to "assume the engineer has zero context for our codebase and questionable taste." This is a deliberate sovereignty transfer -- the operator choosing to pre-script rather than delegate. It has a valid use case (when an operator explicitly wants a step-by-step playbook), but it is not compatible with a pipeline that requires sovereignty at every boundary.

### 3.2 Recommended Topology

```
ground (verified constraints)
  |
  v
bdd (behavior contract: Given/When/Then scenarios)
  |
  v
plan (decision-complete design: approach, interfaces, edge cases, test strategy)
  |
  v
issue-craft / decompose-design (agent-executable issues with behavior traceability)
  |
  v
next-issue (selected session-sized issue with declared goal)
  |
  v
[executing agent] (owns HOW -- uses TDD, debugging, review skills autonomously)
  |
  v
verification-before-completion (behavior-level evidence)
  |
  v
land (merged code, closed issue, behavior coverage record)
```

Optional cross-cutting thread: `hypothesis` (Test Card at specification, Learning Card at completion).

### 3.3 Skill-by-Skill Resolution

#### `plan` -- Keep as-is

`plan` is well-designed. It has a clear domain (design convergence), a clean sovereignty boundary (decision-complete design that resolves all design choices but does not prescribe implementation steps), and sound procedures (explore-before-assuming, explicit assumptions, corruption modes).

It maps precisely to FDD's "Design by Feature" phase: the Chief Programmer produces a design package, and Class Owners implement. In groundwork's terms: `plan` produces a design, and the executing agent implements.

**One change needed:** Update cross-references. The skill currently says "For step-by-step execution breakdown after design convergence, use `writing-plans`." This should change to reference `issue-craft` for decomposition into executable work units.

**File:** `skills/decomposition/plan/SKILL.md` -- update cross-references section.

#### `writing-plans` -- Drop from default pipeline path

**The sovereignty analysis is decisive.** `writing-plans` collapses the boundary between decomposition and execution. Its output artifact ("write this exact code in this exact file") leaves the executing agent with no meaningful domain. Every methodology examined -- FDD, BDD, DDD, vertical slicing -- draws the line before this point.

`writing-plans` is not worthless. It has a valid use case: when an operator explicitly wants a prescriptive step-by-step playbook, or when the implementing agent is extremely constrained (zero codebase familiarity, narrow context window). In those cases, it is a **deliberate sovereignty transfer** -- the operator choosing to pre-script rather than delegate.

**Why not wrap or fix `writing-plans`?** Because the skill's fundamental exigence is wrong for the pipeline. Its purpose is to produce scripts an agent transcribes. Making it sovereignty-compliant would mean rewriting it to produce behavioral contracts instead of code scripts -- at which point it becomes `issue-craft`. There is no distinct sovereignty-respecting domain between "design" (`plan`) and "executable work unit" (`issue-craft`) that `writing-plans` could occupy.

**Concrete changes:**
1. Remove `writing-plans` from the default pipeline flow in WORKFLOW.md and pipeline-contract.md
2. Remove the `bdd -> writing-plans` handoff contract from pipeline-contract.md
3. Keep `writing-plans` as a curated skill available for explicit invocation
4. Update its routing table entry: "when the operator explicitly requests a prescriptive step-by-step playbook"

**Impact on `brainstorming`:** The curated `brainstorming` skill hard-routes its terminal state to `writing-plans` ("The ONLY skill you invoke after brainstorming is writing-plans"). Since `brainstorming` is curated (obra/superpowers, pinned commit), groundwork cannot modify it directly. The pipeline documentation must override this routing: after brainstorming produces a design document, the pipeline routes to `plan` (for design convergence) or `issue-craft` (for issue decomposition). This is the established pattern per ADR-0001: documentation overlays handle pipeline integration for curated skills. If this override proves insufficient in practice, `brainstorming` becomes a candidate for a wrapper skill (Epic 3).

**Decision Gate G4 resolution:** Option (c) -- let `issue-craft` absorb the territory. Creating a wrapper that constrains `writing-plans` to contract-level granularity (option a) would produce `issue-craft` by another name. Replacing it with a different skill (option b) is unnecessary since `issue-craft` already occupies the sovereign decomposition territory.

#### `issue-craft` -- Keep, refocus, extend

`issue-craft` is the right skill for the decomposition territory. Its sovereignty model (behavioral outcomes, not implementation prescription) aligns with every methodology examined. Its known gap -- optimizing for artifact shape rather than problem transfer -- is a fixable exigence issue, not a structural flaw.

**Changes needed:**

1. **Add `decompose-design` procedure.** This is the missing link between `plan` output and executable work units. The procedure takes a decision-complete design (from `plan`) and a behavior contract (from `bdd`) as inputs, and produces a set of issues with:
   - Explicit behavior traceability (which BDD scenarios each issue advances)
   - Dependency graph (execution layers)
   - Session-sized scope
   - Binary acceptance criteria derived from behavior statements

   This maps to FDD's implicit "plan work" step between Design by Feature and Build by Feature, and to SAFe's feature-to-story decomposition with BDD scenarios. The difference from the existing `decompose-epic` procedure: `decompose-epic` starts from deliverables and produces vertical slices; `decompose-design` starts from a resolved design and maps behavioral requirements to work units.

2. **Refocus on problem transfer as core exigence.** The current skill optimizes for artifact shape (well-formed issues with the right fields). The regeneration (Issue #9) should reorient around: "Could a fresh agent, reading only this issue, understand WHY this change is needed -- not just WHAT to produce?" This is the "reader test" applied to issues.

3. **Strengthen anti-prescription guardrails.** Elevate "no implementation prescription in acceptance criteria" from a pre-save check to a constraint (same level as `agent-executable` and `independently-verifiable`). Add a corruption mode for the Issue #5 failure pattern: a prescribed solution that was wrong, followed blindly by an implementing agent.

4. **Add "problem context" to the task template.** Distinct from "scope" and "acceptance criteria," problem context explains WHY the acceptance criteria matter, giving the implementing agent enough understanding to exercise judgment when the landscape differs from expectations.

**File:** `skills/decomposition/issue-craft/SKILL.md` -- regeneration via Issue #9 using `writing-skills` TDD process.

#### No new decomposition skills needed

The gap between `plan` and execution is fully covered by extending `issue-craft` with the `decompose-design` procedure. Creating a separate `design-to-issues` skill would introduce skill proliferation at a project scale where it is not justified. If `issue-craft`'s two decomposition modes (epic-to-tasks and design-to-issues) diverge significantly in practice, separation can happen later.

#### `hypothesis` -- New optional skill for verified user benefit

A new skill in the specification stage, operating as an optional cross-cutting thread (like BDD and documentation):

- **Trigger:** When a feature has measurable user-facing impact (not all changes do).
- **Produces:** Test Card (Strategyzer format) before deployment; Learning Card after deployment.
- **Consumes:** Problem statement and behavior contract.
- **Handoffs:**
  - `hypothesis -> issue-craft`: if a hypothesis exists, the issue links to it
  - `hypothesis -> land`: closure records hypothesis status (untested / confirmed / refuted / inconclusive)
  - `land -> hypothesis`: when measurement data is available, produce a Learning Card

This is lightweight and optional. Most single-developer changes do not need hypothesis testing. The trigger should be explicit: "this change has a measurable user-facing impact that I want to verify." When it does not trigger, the pipeline operates unchanged.

This is **not** a new pipeline stage. It threads through specification, decomposition, and completion like BDD does. Adding a sixth stage would break the pipeline's symmetry and create a disconnected appendage.

### 3.4 Sovereignty Verification at Every Boundary

| From | To | Boundary Artifact | Sovereignty Test |
|------|-----|-------------------|-----------------|
| `ground` | `bdd` | Verified constraints | `bdd` decides which behaviors to specify from the constraints |
| `bdd` | `plan` | Behavior contract (G/W/T) | `plan` decides the design approach to realize the behaviors |
| `plan` | `issue-craft` | Decision-complete design | `issue-craft` decides how to decompose the design into work units |
| `issue-craft` | `next-issue` | Issue set + dependency graph | `next-issue` decides execution order and session selection |
| `next-issue` | executing agent | Selected issue + session goal | Agent decides all implementation details |
| executing agent | `verification` | Implementation + evidence | `verification` decides if evidence is sufficient |
| `verification` | `land` | Verified implementation | `land` executes deterministic closure mechanics |

Every interface passes a **WHAT** artifact. No interface passes a **HOW** prescription. The executing agent receives an issue (what to achieve) and a behavior contract (what behaviors to verify), not a script (what code to type). Sovereignty holds at every boundary.

### 3.5 Decision Gate Resolutions

**Gate G1:** No single established methodology cleanly resolves the full decomposition sovereignty question with artifact-producing stages that map directly to groundwork's typed pipeline. BDD resolves the specification-to-test bridge but not design-to-work-unit decomposition. FDD resolves design-to-build with a formal handoff but has no behavioral specification layer. DDD resolves sovereignty at architectural boundaries but not at work-unit boundaries.

**Gate G2 applies:** Multiple methodologies each resolve a portion. The recommended composite:
- **BDD** informs the specification stage (behavior contracts as the integration thread)
- **FDD** informs the design-to-work-unit handoff (design package -> class ownership, adapted as `plan` -> `issue-craft`)
- **DDD** informs the sovereignty model at every boundary (Published Language as boundary contract, ACL as sovereignty defense)
- **Vertical Slicing** informs the work-unit granularity (independently shippable slices)

The composite does not introduce sovereignty violations at the seams because each methodology's contribution is at a different level of abstraction: BDD at specification, FDD at decomposition mechanics, DDD at sovereignty enforcement, vertical slicing at granularity.

**Gate G3:** The verified-user-benefit territory is **not** entirely organizational. Layer 3 (expected metric movement) is formalizable with the Test Card / Learning Card format. Recommend formalizing it as the optional `hypothesis` skill. Layers 4-5 (genuine benefit interpretation, strategic validity) require human judgment and should not be forced into pipeline artifacts.

**Gate G4:** `writing-plans` cannot be sovereignty-compliant at any pipeline boundary in its current form. Its fundamental exigence (produce scripts an agent transcribes) is incompatible with sovereignty. Recommend option (c): drop from the default pipeline and let `issue-craft` absorb the decomposition territory. Keep available for explicit operator invocation outside the pipeline.

---

## D4: Full BDD-to-Land Skill Inventory

### 4.1 Boundary Skills (produce/consume named artifacts)

| Skill | Stage | Core/Curated | Produces | Consumes | Changes Needed |
|-------|-------|-------------|----------|----------|----------------|
| `ground` | Foundation | Core | Verified constraints | Problem statement | None |
| `research` | Foundation | Core | Substantiated evidence | Research questions | None |
| `bdd` | Specification | Core | Behavior contract (G/W/T) | Grounded constraints | None |
| `hypothesis` | Specification | Core (new) | Test Card / Learning Card | Problem + behavior contract | Create new skill |
| `issue-craft` | Decomposition | Core | Agent-executable issues | Design + behavior contract | Add `decompose-design` procedure; refocus on problem transfer |
| `next-issue` | Decomposition | Core | Session goal | Issue graph | None |
| `plan` | Decomposition | Core | Decision-complete design | Issue + behavior contract | Update cross-refs from `writing-plans` to `issue-craft` |
| `documentation` | Cross-cutting | Core | Doc coverage evidence | Code changes | None |
| `land` | Completion | Core | Merged code, closed issue, coverage record | Verified implementation | Add hypothesis status to closure artifacts |

### 4.2 Operational Skills (execution-phase, context-independent)

| Skill | Stage | Core/Curated | Wrapper Needed? |
|-------|-------|-------------|-----------------|
| `brainstorming` | Decomposition | Curated | **Yes** -- override terminal routing from `writing-plans` to `plan`/`issue-craft` |
| `writing-plans` | (Removed from default pipeline) | Curated | No -- demoted to optional explicit invocation |
| `test-driven-development` | Execution | Curated | No |
| `subagent-driven-development` | Execution | Curated | No |
| `systematic-debugging` | Execution | Curated | No |
| `requesting-code-review` | Verification | Curated | No |
| `receiving-code-review` | Verification | Curated | No |
| `verification-before-completion` | Verification | Curated | No |
| `writing-skills` | Meta | Curated | No |

### 4.3 Meta Skills

| Skill | Purpose | Changes Needed |
|-------|---------|----------------|
| `using-groundwork` | Methodology orientation and routing | Update routing table and flow description to reflect new topology |

### 4.4 Wrapper Skill Requirements (Epic 3)

Only one curated skill requires a wrapper:

**`brainstorming`** -- Its hard terminal routing ("The ONLY skill you invoke after brainstorming is writing-plans") conflicts with the pipeline's sovereignty model. The wrapper must:
1. Preserve brainstorming's core value (exploring 2-3 design approaches with trade-offs)
2. Override the terminal state to route to `plan` (for design convergence) or `issue-craft` (for issue decomposition)
3. Not modify the curated skill directly (per ADR-0001)

The first attempt should be a documentation overlay in WORKFLOW.md and `using-groundwork`. If the override proves insufficient in practice (agents follow brainstorming's internal routing despite pipeline documentation), escalate to a wrapper skill that wraps the curated skill with corrected routing.

All other curated skills (TDD, debugging, subagent orchestration, code review, verification) are context-independent and integrate cleanly via pipeline documentation without wrappers.

---

## Implementation Sequence

Ordered by dependency and leverage:

1. **Regenerate `issue-craft`** (Issue #9) -- Add `decompose-design` procedure, refocus on problem transfer, strengthen anti-prescription guardrails. Use `writing-skills` TDD process.

2. **Update pipeline documentation** -- Remove `writing-plans` from default flow in WORKFLOW.md, pipeline-contract.md, and `using-groundwork`. Update handoff contracts. Document `plan -> issue-craft` as the canonical decomposition boundary. Override `brainstorming` terminal routing.

3. **Update `plan` cross-references** -- Change references from `writing-plans` to `issue-craft`.

4. **Create `hypothesis` skill** -- Specification-stage skill for measurable user benefit (Test Card / Learning Card format). Optional trigger.

5. **Update `land` for hypothesis closure** -- Add hypothesis status to closure artifacts.

6. **Evaluate `brainstorming` wrapper need** -- After documentation overlay is in place, test whether agents follow the overridden routing. Escalate to wrapper skill if not.

---

## Sources

### Primary Methodology Sources

- Dan North, "Introducing BDD" (2006): https://dannorth.net/blog/introducing-bdd/
- Eric Evans, *Domain-Driven Design* (2004). DDD Reference: https://www.domainlanguage.com/wp-content/uploads/2016/05/DDD_Reference_2015-03.pdf
- Palmer & Felsing, *A Practical Guide to Feature-Driven Development* (2002). Process model: http://www.featuredrivendevelopment.com/files/FDD%20Process%20Model%20Diagram.pdf
- Jeff De Luca, 2007 Interview: https://www.it-agile.de/fileadmin/docs/FDD-Interview_en_final.pdf
- Gojko Adzic, *Specification by Example* (2011): https://gojko.net/books/specification-by-example/
- Gaspar Nagy & Seb Rose, *The BDD Books*: https://bddbooks.com/
- Matt Wynne, "Example Mapping": https://cucumber.io/blog/bdd/example-mapping-introduction/
- Cucumber BDD documentation: https://cucumber.io/docs/bdd/
- Alberto Brandolini, Event Storming: https://www.eventstorming.com/
- Martin Fowler, "Bounded Context": https://martinfowler.com/bliki/BoundedContext.html
- ddd-crew Context Mapping: https://github.com/ddd-crew/context-mapping
- SAFe BDD: https://scaledagileframework.com/behavior-driven-development/
- Jeff Patton, *User Story Mapping* (O'Reilly, 2014)
- Alan Klement, Job Stories: https://jtbd.info/replacing-the-user-story-with-the-job-story-af7cdee10c27

### Verified User Benefit Sources

- Jeff Gothelf & Josh Seiden, *Lean UX* (O'Reilly, 3rd ed. 2021): https://jeffgothelf.com/
- Alexander Osterwalder & David Bland, *Testing Business Ideas* (Wiley, 2019). Test Card: https://www.strategyzer.com/library/test-card
- LaunchDarkly Guarded Releases: https://launchdarkly.com/product/feature-flags/
- Pete Hodgson, "Feature Toggles": https://martinfowler.com/articles/feature-toggles.html
- Ron Kohavi, Diane Tang, Ya Xu, *Trustworthy Online Controlled Experiments* (Cambridge, 2020)
- Gojko Adzic, *Impact Mapping* (2012): https://www.impactmapping.org/
- Teresa Torres, *Continuous Discovery Habits* (2021): https://www.producttalk.org/
- Nicole Forsgren, Jez Humble, Gene Kim, *Accelerate* (2018). DORA: https://dora.dev/
- Eric Ries, *The Lean Startup* (2011)

### Groundwork Internal Sources

- `skills/decomposition/plan/SKILL.md` -- design convergence skill
- `skills/decomposition/issue-craft/SKILL.md` -- issue lifecycle skill
- `skills/specification/bdd/SKILL.md` -- behavior-driven development skill
- `skills/completion/land/SKILL.md` -- closeout workflow
- `.claude/skills/writing-plans/SKILL.md` -- curated implementation planning skill
- `docs/architecture/pipeline-contract.md` -- formal handoff contracts
- `docs/architecture/decisions/0001-pipeline-integration-strategy.md` -- curate-vs-own ADR
- `WORKFLOW.md` -- integration manual
- `ARCHITECTURE.md` -- system architecture
