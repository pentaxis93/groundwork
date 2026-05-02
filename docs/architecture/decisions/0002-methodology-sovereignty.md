# ADR-0002: Methodology Sovereignty

**Status:** Accepted \
**Date:** 2026-05-02 \
**Traces to:** `tesserine/commons` PRINCIPLES.md P1 (Sovereignty); `tesserine/commons` ADR-0001 (Sovereignty).

*Architectural reckoning produced by the substrate-aware cycle that followed the withdrawal of the prior architectural epic (`tesserine/groundwork#256`, withdrawn 2026-05-02 for frame-inheritance).*
*Anchor commit:* `e0f32ff` *in* `tesserine/groundwork`*.*
*Substrate ground at the time of the reckoning:* `tesserine/groundwork`, `tesserine/commons`*.*

---

## Preamble

This ADR documents the architectural reckoning that produced the methodology sovereignty principle and its operational consequences. The body's nine numbered sections answer the architectural questions the cycle was commissioned to address; each section shows the reckoning explicitly — the inherited answer, the diagnostic question, the comparison, the surviving answer.

The cycle's discipline was mutual. Governance probed frame-soundness; the produced work surfaced frame-inheritance by name. Where a heuristic carried forward from current substrate, the reckoning said so and showed the comparison that supports retention. Where a heuristic failed first-principles reckoning, the reckoning named the replacement.

The diagnostic question — *"what would I propose if no precedent existed?"* — fires at every layer in this ADR, not as a one-shot at any single layer. The prior cycle's failure was that no layer received the question; every layer accepted current substrate as fix-point. This cycle's posture is opposite: every layer's inherited answer is named first, the diagnostic question is applied, and the surviving answer must trace either to first-principles ground or to a comparison that vindicates the inherited answer over alternatives.

**Recursive sub-structural application.** The diagnostic question fires not only at each layer but at every sub-structural layer within any choice that survives. When a format is chosen at a layer, the format's internal structure (the sub-shape it admits — flat array, graph, tree, key-value map, etc.) is itself a choice that must receive the diagnostic question. Inheritance can ride into a layer through a top-level format choice; it can also ride in through the sub-structural shape that format implies. The discipline is "diagnostic at every layer AND every sub-layer," not "diagnostic at every layer." This sub-structural application emerged as the discipline-revision finding from this cycle's first-revision round (see Finding 9.12); it carries forward as the cycle's running articulation.

Length is the reckoning being shown. Where a comparison is doing substantive work, the comparison is shown. Governance pre-committed to review-by-reckoning-quality (not deliverable-length); that pre-commitment was the contract under which this document was written.

---

## 1. Principles articulated

### Inherited articulation

The prior cycle (issue #256) articulated **cognitive sovereignty** as a fractal application of P1/ADR-0001 at a fourth interface: the intra-unit WHAT/HOW boundary inside methodology documents.

The prior cycle's verbatim outcome statement: *"each protocol or skill encodes either WHAT (intent/outcome) or HOW (procedure/mechanics) at any single level of presentation. Workflow specifications, tool mechanics, and cognitive disciplines are distinct content shapes."*

Two structural choices in that articulation are inherited heuristics worth naming separately:

1. **Per-level shape purity.** The constraint binds per "level of presentation" inside a unit, not per unit. A protocol document with a clean WHAT section followed by a clean HOW section conforms.
2. **The fractal extension is to a "fourth interface."** The articulation positions the methodology-internal interface alongside ADR-0001's enumerated interfaces (operator↔agent, skill↔consumer, etc.) as a peer.

### Diagnostic question

What principle (or principles) does the redesign rest on if no precedent — neither the prior cycle's articulation nor the current substrate's protocol/skill split — is treated as fix-point?

### Reckoning

**Substrate observation, restated.** `protocols/submit/PROTOCOL.md` at 404 lines (verified-broken instance, anchor commit `e0f32ff`):

- Step 1 ("Resolve context") spans lines 55–101 and mixes workflow-narration ("Determine: current branch name; whether on `main`...") with GitHub-specific JSON field paths (`headRefOid`, `headRefName`, `headRepository`, `headRepositoryOwner.login`) and shared-remote-matching rules ("strip `.git`, compare the lowercase `<owner>/<repo>`") at the same bullet-level indent.
- Step 6 ("Create or identify PR"), lines 257–283, embeds literal shell heredoc snippets (`cat > /tmp/pr-body.md <<'EOF'`) inside the workflow specification.
- Step 7 ("Deliver `patch`"), lines 290–308, includes an MCP-tool invocation expressed as JavaScript-shaped syntax (`patch({ instance_id: ..., pr_reference: ..., ... })`).

By contrast, `skills/debug/SKILL.md` (307 lines) contains zero shell syntax, zero forge JSON paths, zero MCP-tool invocation syntax. It is uniformly cognitive content: "Stop. Read. Reproduce. Trace. Hypothesize." — instructions to the agent's reasoning, not to the agent's tooling. `skills/orient/SKILL.md` (171 lines) has the same character: "Read the work-unit graph first... The Flow: five stages, in dependency order..." Pure cognitive discipline. The separation between WHAT-only and WHAT-mixed-with-HOW is not theoretical; it is observable in the substrate as the difference between skills-folder and protocols-folder.

A separate witness: `tests/test_submit_protocol.py` contains 28 substring-presence assertions on submit's prose. Lines 30 through 285 are predominantly `self.assertIn("specific phrase", protocol)` — verifying that specific words exist in the document, not that the encoded behavior is correct. This is the verification surface the broken substrate forces: when a methodology unit is prose mixing WHAT and HOW, behavior-level testing is unavailable, and substring-presence is the residual.

**Candidate principle stacks tested against the observation.** The substrate observation must trace cleanly to the principle stack the redesign rests on. The candidates:

- **P1 Sovereignty alone, fractal extension.** P1 says: "the operator declares WHAT: direction, vision, intent. The agent owns HOW: execution, implementation, craft." The fractal claim says: "It applies at every interface: human-agent, agent-agent, skill-skill, and stage-stage." The methodology document is a human-authored artifact addressed to an agent; the document plays the operator role and the agent invoking it plays the agent role. Mixing WHAT and HOW in the document is the operator reaching across the sovereignty boundary into the agent's domain. *This trace is clean.* The substrate observation is exactly a sovereignty violation at the methodology-document interface.

- **P3 Grounding alone.** P3 says: "Existing code is evidence about one attempt to meet requirements; it is not the requirements themselves." Embedded HOW (specific gh JSON fields, specific git refspecs) is "yesterday's HOW" frozen into the methodology document. Once written, it persists across forges, across runtime versions, across tool generations. P3 says grounding "re-fires on every new generative act"; embedded HOW prevents this re-firing because the document specifies the HOW directly, not the WHAT-the-HOW-must-achieve. *This trace is also clean,* but it explains the persistence pathology rather than the structural defect. P3 names why mixed documents accumulate over time; it does not directly name what is wrong with them at any single moment.

- **P6 Transmission alone.** P6 says: "Work completes when the recipient can act on it, not when the maker finishes creating it. The door must fit who enters." A mixed-shape document has multiple recipients with incompatible needs: a reader who wants to understand the protocol's outcome must wade through forge-specific tool syntax; a reader who wants to execute the protocol must extract the workflow logic from the implementation specifics. *The trace is clean,* but it explains why mixed documents fail readers, not what makes the mix structurally wrong.

- **P7 Verifiable Completion alone.** P7 says: "Every unit of work has mechanically verifiable completion criteria... pass/fail, not subjective and approximate." The substring-test pattern in `tests/test_submit_protocol.py` is the symptom: when a methodology unit mixes shapes, behavior-level verification is unavailable and substring-presence is the only residual. *This trace is clean,* but it explains the verification consequence, not the structural defect that produced it.

- **Composite (P1 primary + P3, P6, P7 corollaries).** All four principles fire on the substrate observation, in different layers of the same defect:
  - P1 names the *structural* defect (sovereignty boundary crossed).
  - P3 names the *temporal* defect (yesterday's HOW persists, prevents re-grounding).
  - P6 names the *audience* defect (mixed content fails both readers).
  - P7 names the *verification* defect (mixed content admits only substring-tests).

  The composite reading is empirically right: all four principles trigger. The *structural* layer is P1; the others are downstream consequences. A clean articulation that grounds in P1 inherits the others as corollaries without asserting them as independent principles.

**Where the candidate stacks differ from each other.** The choice between P1-primary-with-corollaries and a composite presentation is a presentation question, not a substantive one. Both stacks land in the same place: the redesign rests on P1 sovereignty applied at the methodology-document interface, and the corollary consequences (re-groundability, transmission cleanliness, verifiability) follow.

**The unit-boundary question.** The prior cycle's articulation positions the constraint per "level of presentation" inside a unit. The diagnostic question: if no precedent existed, would I bind the constraint per-level or per-unit?

Per-level allows: a single document with a clean WHAT section followed by a clean HOW section. Mixed at the document level, pure at the section level.
Per-unit allows only: the document is uniformly one shape. Mixed-shape documents do not exist.

The case for per-level: less file proliferation, single-source authoring, easier cross-reference within a document.

The case for per-unit: the unit boundary becomes the natural file boundary, file naming/path encodes shape, schemas can validate by document type, tooling can transform per-document, readers can choose which file to consult based on their need.

Per-unit is stricter, and the day-one stance argues for the stricter reading: making mixing structurally impossible at the document boundary is stronger than making it visible at the section boundary. A linter that catches section-level mixing detects the violation; a file-type schema that rejects mixed-shape documents prevents the violation. P1's "clean boundaries" reads more cleanly when the boundary is a file, not a heading.

**The fractal-extension question.** The prior cycle positions the methodology-internal interface as a "fourth interface" alongside ADR-0001's enumerated set. ADR-0001 enumerates five interfaces (daemon↔runtime, runtime↔methodology, operator↔agent, skill↔consumer, producer↔validator) and claims fractality. PRINCIPLES.md P1 enumerates fractal scales (human-agent, agent-agent, skill-skill, stage-stage). Neither enumeration explicitly names the methodology-document-internal interface.

The diagnostic question: does the methodology-document interface need to be added to ADR-0001's enumeration as a peer ("fourth interface" or "sixth interface" depending on counting), or is it covered by the fractal claim itself?

The cleaner reading is the second. ADR-0001's fractal claim ("It applies at every interface, at every scale") is general; the enumerations are illustrative, not exhaustive. The methodology-document interface is one instance the enumeration didn't name explicitly. Treating it as covered by the fractal claim — rather than as a peer requiring ADR enumeration extension — is structurally simpler and avoids an ecosystem-level commission against `tesserine/commons`.

This is a substantive divergence from the prior cycle: the prior cycle's positioning ("fourth interface, peer to ADR-0001's enumeration") implied an ADR-0001 update was a prerequisite. The cleaner reading does not: P1's fractal claim already covers the methodology-document interface; the redesign articulates *which* fractal scale it operates at, not *that* a new fractal scale exists.

### Surviving articulation

**Methodology Sovereignty.** Each methodology unit specifies content of a single shape — WHAT (intent, outcome, contract, invariant) or HOW (procedure, tool-invocation, recipe) — not both. The document boundary is the unit boundary; per-document shape purity is the constraint. Units may reference each other; they may not embed content of the other shape.

**Derivation.** P1 Sovereignty's fractal claim ("It applies at every interface, at every scale" — ADR-0001 line 16; "It applies at every interface: human-agent, agent-agent, skill-skill, and stage-stage" — PRINCIPLES.md line 43). The methodology document is a human-authored artifact addressed to an agent. The document plays the operator role (declares WHAT: intent, outcome, contract); the agent invoking the document plays the agent role (owns HOW: execution, implementation, craft). Methodology Sovereignty is P1 fractal at this interface. ADR-0001's enumeration of five interfaces is illustrative, not exhaustive; the methodology-document interface is covered by the fractal claim and does not require ADR-0001 enumeration extension.

**Corollaries.** Three corollary derivations operationalize Methodology Sovereignty:

- **Re-groundability (P3 corollary).** When HOW is separated from WHAT into distinct units, HOW can be re-grounded on each fresh execution without disturbing the WHAT layer. The methodology resists tool-syntax obsolescence because tool-syntax lives in HOW-shaped units that the methodology author does not own.
- **Transmission cleanliness (P6 corollary).** A unit of single shape serves a single audience well; a mixed unit fails both audiences. Methodology Sovereignty enables transmission by giving each unit a single addressee.
- **Verifiability (P7 corollary).** WHAT-only units admit structural verification (the unit's content is parsed against its shape's grammar; mismatches reject). Mixed units admit only substring-presence verification. Methodology Sovereignty makes mechanical verification possible.

**The principle does not assert that HOW is unimportant or absent.** HOW exists; HOW is needed; HOW is sovereign-to-the-agent. The principle asserts that HOW lives in HOW-shaped units, distinct from WHAT-shaped units. The methodology contains both shapes; it does not mix them within a single unit.

### Divergence from the prior cycle's articulation, named

Two named divergences:

1. **Per-unit, not per-level.** The constraint binds the document, not the section within the document. A protocol document with a clean WHAT section followed by a clean HOW section is *not* conformant under this articulation; the WHAT section and the HOW section are different units that must live in different documents.

2. **Fractal coverage, not enumeration extension.** The methodology-document interface is covered by P1/ADR-0001's existing fractal claim, not added as a new enumerated interface. No ADR-0001 update is implied. The ecosystem-level numbering inconsistency in commons ADR-0007 (item 9 of this deliverable) is a separate finding.

### What this principle requires of subsequent layers

- **Layer B (categorization)** must enumerate the unit shapes and confirm that each shape is uniformly WHAT or uniformly HOW.
- **Layer C (representation)** must choose per-shape formats that admit shape-purity at the parser level (the strong tier of Methodology Sovereignty operates when the format itself rejects shape-violations).
- **Layer D (verification)** matches its tier to Layer C's tier per shape. Where Layer C achieves structural-impossibility, Layer D's verification is the parser; where Layer C settles for after-the-fact detection, Layer D adds a linter; where neither is possible, Layer D specifies the inspection procedure.
- **Layer E (migration and vehicle)** addresses how current substrate (where the principle is violated in protocols and confused in tests) becomes the redesigned substrate (where the principle is structurally enforced).

---

## 2. Content categorization

### Inherited categorization

The prior cycle (issue #256) decomposed methodology content into **five categories**: discipline, workflow specifications, tool mechanics, topology, principle articulation. The categorization treated current substrate's protocol/skill split + manifest as fix-point and added "mechanics" as the missing extraction.

### Diagnostic question

If no precedent existed — neither the current protocol/skill split nor the prior cycle's five-category cut — what categories would I propose for methodology content, with what purpose / sovereignty / composition rules per category?

### Reckoning

**The principle constrains the shape boundary.** Methodology Sovereignty (Layer A) says each unit specifies content of a single shape: WHAT or HOW. The top-level cut is therefore binary. The diagnostic question collapses to: within WHAT, what sub-shapes exist; within HOW, what sub-shapes exist; and what determines a sub-shape boundary.

**Sub-shape boundaries are determined by representation need, not by domain.** Two units share a sub-shape if they admit the same representation format and the same conformance verification. Two units have different sub-shapes if they need different formats or different verification approaches. This is downstream of the principle: the principle gives the WHAT/HOW cut; representation needs give the within-shape cuts.

**Substrate witnesses, classified by current shape:**

- `skills/{contract,debug,orient,reckon,research,resolve}/SKILL.md` — uniformly cognitive content, no tool syntax, no MCP-tool invocations, no shell heredocs. WHAT-pure today. Shape: cognitive prose.
- `protocols/{specify,plan,verify,take,survey,document,implement,land,decompose,submit}/PROTOCOL.md` — currently mixed. WHAT layer (workflow narrative: when fires, what consumes, what produces) and HOW layer (gh JSON paths, git refspecs, MCP-tool invocation syntax) co-exist at the same indent level. Two distinct sub-shapes confused into one document.
- `manifest.toml` — declarative typed structure. Per-protocol entries (`requires`, `accepts`, `produces`, `may_produce`, `trigger`) plus artifact type registry. WHAT-pure, graph-shape encoded in TOML.
- `schemas/{12 artifacts}.json` — JSON Schema 2020-12 documents. WHAT-pure, contract-typed.
- `tests/test_submit_protocol.py` — Python test code. Substring-presence assertions on submit's prose (28 of them). HOW (substring matching mechanic) entangled with implicit WHAT (the phrases to be present). The verification surface forced by the broken substrate.
- `docs/architecture/connecting-structure.md` (1152 lines), `docs/architecture/work-unit-model.md` (52 lines), `docs/authoring/skills.md` (138 lines) — cognitive prose addressed to contributors.
- `docs/architecture/decisions/0001-internal-development-history-policy.md` — ADR. Cognitive prose, decision-shaped.
- `README.md` — cognitive prose, addressed to first encounters.

**The substrate's current categories partially align with the principle.** Skills, manifest, and schemas are already WHAT-pure. Protocols and tests are not. The redesign's task is to make every category WHAT-pure or HOW-pure at the unit boundary.

**Candidate cuts tested.**

- **Cut A: Binary by shape (WHAT, HOW).** Two categories. Pro: minimal, principle-aligned. Con: ignores representation differences within WHAT (a cognitive discipline and a typed schema both encode WHAT, but they need very different formats).
- **Cut B: By audience (agent / runa / contributor / operator).** Four categories. Pro: clean addressee mapping. Con: cuts across shape (a workflow contract is read by both runa-as-validator and agent-as-executor; sub-shape determines format more than audience does).
- **Cut C: By lifecycle position (design / authoring / execution / post-execution).** Four categories. Pro: clean temporal mapping. Con: cuts across shape and provides no representational guidance.
- **Cut D: By composition role (foundational / interface-defining / behavior-specifying / executing / verifying).** Five categories. Pro: clean composition. Con: reproduces shape mixing within composition role (e.g., "verifying" mixes behavior-contract WHAT with test-runner HOW).
- **Cut E: Shape boundary primary, representation-need secondary.** Top cut: WHAT / HOW. Within each, sub-shapes determined by representation need. Pro: principle-aligned, format-driven within shape. Con: requires per-sub-shape format reckoning at Layer C.

**Cut E survives.** The principle requires the WHAT/HOW boundary; representational needs determine the sub-shapes. No alternative cut satisfies the principle without reproducing shape mixing.

**Sub-shapes within WHAT, by representation need:**

1. **Cognitive prose** — content addressed to agent reasoning or to contributor learning. The substrate exemplar is `skills/debug/SKILL.md`: instructions to the agent's reasoning, not its tooling. Format need: prose with structured frontmatter; structural validation of frontmatter; prose body validated negatively (absence of HOW-shaped patterns).
2. **Typed protocol contract** — per-protocol specification of interface (when fires, what consumes, what produces) and workflow (the cognitive shape of execution: which disciplines fire when, which mechanics get invoked). Format need: structured data with parser/schema validation; rejection of malformed contracts at parse time.
3. **Typed artifact contract** — per-artifact-type specification of structure (required fields, value constraints, cross-references). Format need: schema language with parser/validator (current substrate uses JSON Schema 2020-12; this is the inherited choice tested at Layer C).
4. **Architectural decision and principle prose** — content addressed to contributors making future decisions. Foundational reasoning (principles); operationalized reasoning (ADRs). Format need: prose with structured frontmatter (decision shape); validation similar to cognitive prose.

**Sub-shapes within HOW, by representation need:**

5. **Mechanics** — agent-callable recipes for tool invocation (forge operations, MCP tool calls, git refspec patterns, shell heredoc patterns). Format need: structured recipes with parameter schemas and examples, OR executable code, OR a mix. The format choice depends on whether the methodology owns the mechanics (and provides them) or the agent owns the mechanics (and the methodology only references their existence). Layer C reckons.
6. **Verification specifications** — code that verifies methodology units conform to their shape (parser-level, automatic) and that protocols produce their declared outcomes when executed (behavior-level, requires interpretation). Format need: depends on whether methodology is executable. Layer C reckons in conjunction with Layer D.

### Surviving categorization

Six categories. Top-level: **WHAT** (four sub-shapes) and **HOW** (two sub-shapes). Per-category specification:

#### Category 1: Disciplines (WHAT, cognitive prose)

- **Purpose.** Encode the cognitive content the agent applies during execution: how to think, when to halt, what to investigate, how to decide. Disciplines are the methodology's cognitive surface.
- **Sovereignty.** WHAT. The discipline declares what reasoning the agent applies; the agent owns whether and how to apply it in any given execution. The methodology-author declares the discipline; the agent internalizes it.
- **Composition rules.**
  - May reference: other disciplines (debug invokes reckon at the 3-fix escalation rule; orient names every other skill).
  - May reference: workflow contracts (by name; e.g., debug's "Handoff with `implement`").
  - May reference: mechanics (by name only; never embed mechanic content).
  - May NOT embed: workflow contract content, mechanic content, tool syntax, MCP-tool invocations, shell commands, JSON field paths.
- **Substrate witness.** `skills/{contract,debug,orient,reckon,research,resolve}/SKILL.md`. WHAT-pure today. The separation-achievable proof: 1300 lines of cognitive content with zero tool syntax. Also includes orient's "Documentation Discipline" subsection — procedural in shape ("Before writing any documentation: Name the audience... State what they already know...") but cognitive in content (instructions to thinking, not to tooling). Cognitive procedure is WHAT; tool procedure is HOW. The distinction is observable: a procedure that says "decide X" is WHAT; a procedure that says "run `gh pr list`" is HOW.
- **Audience.** Agent reasoning during execution; contributors learning the methodology; future maintainers.
- **Sub-shape unification.** Includes what the prior cycle called "discipline" and adds the cognitive-prose components of authoring guides and orientation content. README's WHAT-portions and authoring guides' WHAT-portions are cognitive prose addressed to contributors; they share the discipline category's representation needs.

#### Category 2: Workflow contracts (WHAT, typed)

- **Purpose.** Per-protocol specification of the protocol's interface (when it fires, what artifacts it consumes, what artifacts it produces) and its workflow (the cognitive shape of execution: what disciplines apply during which steps, what mechanics it references). One workflow contract unit per protocol.
- **Sovereignty.** WHAT. The workflow contract declares the protocol's outcome and the workflow's structure; the agent owns the execution (which mechanics to invoke, in what order, with what arguments). The methodology-author declares the contract; the agent owns conformance.
- **Composition rules.**
  - References: disciplines (by name — the cognitive content the agent applies during execution); mechanics (by name — the HOW-shaped recipes the agent invokes); artifact schemas (by name — what consumed/produced artifacts must satisfy).
  - May NOT embed: discipline content, mechanic content (tool syntax), schema content (full type definitions).
  - The interface declaration (`requires`, `accepts`, `produces`, `may_produce`, `trigger`) is a sub-shape of the workflow contract; whether it co-locates with the workflow narrative in a single file or distributes (current `manifest.toml` centralization vs. per-protocol distribution) is a Layer C representation question.
- **Substrate witness.** Currently mixed across two locations: `manifest.toml` carries the typed interface declaration (per-protocol entry with `requires`/`produces`/etc.); `protocols/X/PROTOCOL.md` carries the workflow narrative entangled with mechanics. Submit's 404 lines, with witness passages at lines 55–101 (step 1, mixed), 257–283 (step 6, embedded heredoc), 290–308 (step 7, embedded MCP invocation), are this category's broken instances.
- **Audience.** Runa (validates the typed interface; drives protocol firing on artifact triggers); agent (orients to the protocol's outcome and workflow); contributors (read the protocol's purpose and structure).
- **Sub-shape note.** A workflow contract has two sub-shapes: the interface-typed sub-shape (graph-compatible: requires/produces/trigger) and the workflow-narrative sub-shape (cognitive: what the protocol does in WHAT-only terms, referencing disciplines and mechanics). Layer C addresses whether these sub-shapes share a single document format or split into two co-located documents. The category itself groups them because they share a unit (the protocol) and audience (runa + agent).

#### Category 3: Mechanics (HOW, structured recipes)

- **Purpose.** Encode the tool-invocation recipes the agent uses during execution: forge operations (gh PR creation, gh issue lookup), MCP tool invocations (patch artifact delivery), git refspec patterns (PR head fetch, push to existing PR branch), shell heredoc patterns (multiline body transport). Mechanics are HOW.
- **Sovereignty.** HOW. The agent owns mechanics fully — when to invoke them, with what arguments, in what order. The methodology may curate a library of well-formed mechanics; the agent invokes them as the workflow contract specifies. P1: the agent's "execution, implementation, craft."
- **Composition rules.**
  - References: other mechanics (composition: a high-level mechanic may compose lower-level ones).
  - References: artifact schemas (by name — when the mechanic produces an artifact whose shape is schema-defined).
  - May NOT reference: workflow contracts, disciplines, topology declarations, principles. Mechanics are HOW; they must not depend on WHAT-shapes for correctness. (A mechanic for "create a GitHub PR" is the same mechanic regardless of which workflow contract invokes it; coupling mechanics to specific workflow contracts re-creates shape mixing.)
  - May NOT embed: workflow content, discipline content. Mechanics are pure recipe.
- **Substrate witness.** Currently embedded in protocol bodies. The witness passages above (submit step 1's gh JSON paths, step 6's heredoc, step 7's MCP invocation) ARE current mechanics, embedded in the wrong category. After redesign: extracted into mechanic units, referenced from workflow contracts.
- **Audience.** Agent execution.
- **Sub-shape note.** Mechanics may be expressed as: structured recipes (parameters + invocation pattern), executable code (Python, shell), or specifications consumed by code generation. Layer C reckons. The category groups them because they share sovereignty (agent-side HOW) and composition rules (no WHAT-references).
- **Forge-neutrality.** Mechanics are forge-specific by definition; the methodology may provide multiple mechanics for the same workflow contract step (e.g., a GitHub-shaped "create PR" mechanic and a Forgejo-shaped "create PR" mechanic). The workflow contract references the operation (create-pr); the mechanics provide forge-specific implementations. This is the prior cycle's "forge-neutrality as genuine sovereignty boundary" insight, retained.

#### Category 4: Artifact schemas (WHAT, typed contracts)

- **Purpose.** Per-artifact-type specification of structure: required fields, value constraints, cross-references between artifact types. Artifact schemas are the contract surface between protocols (one protocol produces, the next consumes; the schema is the boundary).
- **Sovereignty.** WHAT. The schema declares what an artifact must satisfy; the agent owns producing artifacts that satisfy. Runa enforces.
- **Composition rules.**
  - References: other artifact schemas (an artifact may carry references to other artifacts; the reference shape is encoded in the schema).
  - May NOT reference: workflow contracts (schemas are consumed by workflow contracts, not the reverse), disciplines, mechanics.
  - May NOT embed: workflow content, mechanic content.
- **Substrate witness.** `schemas/{12 artifacts}.json`. WHAT-pure today. JSON Schema 2020-12 format. The `request` schema is canonically vendored from `tesserine/commons`; the others are methodology-private. Format choice tested at Layer C.
- **Audience.** Runa (validates artifact instances at protocol boundaries); agent (produces and consumes artifacts conforming to schemas).

#### Category 5: Verification specifications (HOW, with referenced WHAT)

- **Purpose.** Encode the verification mechanisms that establish methodology conformance. Two distinct verification kinds: **conformance verification** (does a methodology unit match its shape's grammar?) and **behavior verification** (when this protocol is fired with these inputs, does the declared outcome occur?). Both are HOW; the WHAT being verified lives in other categories (workflow contracts, artifact schemas, disciplines via prose).
- **Sovereignty.** HOW. Verification is the methodology's testing surface; the methodology-author owns the verification mechanism design; CI / runa / agent execution exercise it.
- **Composition rules.**
  - References: workflow contracts (what behaviors verify), artifact schemas (what shapes verify), disciplines (what disciplines may need cognitive-conformance review where mechanical verification is unavailable).
  - May NOT embed: workflow content, schema content, discipline content. References, not embeddings.
- **Substrate witness.** `tests/test_submit_protocol.py` (28 substring-presence assertions on submit's prose) and `tests/test_request_schema_vendoring.py` (schema vendoring discipline). The substring-presence pattern is the symptom of mixed-shape units: behavior verification is unavailable when the unit isn't parser-checkable, so substring-presence is the residual. Issue #255 questions this pattern explicitly.
- **Audience.** CI; runa (during validation passes); contributors authoring verification specs.
- **Lifecycle relationship to redesign.** The verification approach depends on Layer C's format choices: structural-impossibility formats (parser-level) make conformance verification automatic; weaker formats require linter-level conformance verification. Behavior verification depends on whether methodology becomes interpretable (a runa or build-step interpreter that exercises workflow contracts and observes outcomes).

#### Category 6: Architectural decisions and foundational principles (WHAT, prose)

- **Purpose.** Architectural decisions (ADRs) operationalize foundational principles into specific architectural commitments. Foundational principles (PRINCIPLES.md, principles.md if introduced) ground the methodology's existence: the bedrock reasoning that ADRs trace to.
- **Sovereignty.** WHAT. The methodology author (after governance reckoning) declares decisions and principles; future contributors and maintainers consult them. P1 fractal at the design-time interface (governance ↔ implementation).
- **Composition rules.**
  - ADRs reference: foundational principles (each ADR's "Traces to" line names the principle the decision operationalizes); other ADRs (cross-decision references).
  - Principles do NOT reference ADRs (principles are foundational; ADRs are downstream).
  - May NOT embed: workflow content, mechanic content, schema content (full).
  - References to current substrate (file paths, line numbers, code snippets) are admissible for *grounding* a decision, not for *substituting* the decision's content.
- **Substrate witness.** `docs/architecture/decisions/0001-internal-development-history-policy.md` (only local ADR currently); `tesserine/commons/PRINCIPLES.md` and `tesserine/commons/adr/{0001-0008}.md` (ecosystem-level principles and ADRs that this methodology grounds in).
- **Audience.** Contributors making future decisions; future maintainers understanding why current decisions exist; agents orienting to the methodology's foundations.

### Composition graph (between categories)

```
Foundational principles (cat 6)
  ←─ traces from ─ Architectural decisions (cat 6)
                     ↓ ground
Disciplines (cat 1) ── reference ──→ Workflow contracts (cat 2) ── reference ──→ Mechanics (cat 3)
       │                                  │   │
       │                                  │   └── reference ──→ Artifact schemas (cat 4)
       │                                  │
       │                                  └── triggered by ──→ Artifact schemas (cat 4)
       │
       └── invoked from ─── Workflow contracts (cat 2)

Verification specifications (cat 5) ── reference (verify) ──→ Workflow contracts (cat 2),
                                                              Artifact schemas (cat 4),
                                                              Disciplines (cat 1) [conformance only]
```

The graph is a DAG. Mechanics depend on artifact schemas (output shapes) but not on workflow contracts (mechanics are reusable across contracts). Workflow contracts depend on disciplines (cognitive content), mechanics (HOW recipes), and artifact schemas (artifact shapes). Disciplines may reference workflow contracts and mechanics by name (handoffs) but cannot embed their content.

### Divergences from the prior cycle's categorization, named

1. **Artifact schemas as a distinct category.** The prior cycle did not break out artifact schemas; they were implicit under topology or workflow specs. They have a distinct representation (JSON Schema), distinct audience (runa validation), and distinct composition rules (no workflow references). Promoting them to their own category clarifies the boundary.

2. **Verification specifications as a distinct category.** The prior cycle did not break out verification. Issue #255 questions the current substring-presence pattern; the redesign needs a category to address what verification IS in the new architecture. Verification is HOW with referenced WHAT — its own category, with explicit composition rules.

3. **Topology is derived, not authored as a separate category.** The prior cycle had "topology" as a category co-equal with "workflow specifications." Per the principle's single-source-of-truth implication, the topology graph is a *view* over per-protocol typed interfaces (sub-shape of workflow contracts). Whether the typed interfaces centralize in one file (current `manifest.toml`) or distribute per-protocol is a Layer C representation question. As a *category*, topology is workflow contracts' interface-typed sub-shape, not a peer.

4. **ADRs and foundational principles as one category.** The prior cycle had only "principle articulation." ADRs are operationalized decisions with distinct composition rules (ADRs reference principles; principles do not reference ADRs). Combining into a single category preserves the distinction internally (sub-shape) while grouping by representation (prose, structured frontmatter).

5. **Per-document, not per-section, unit boundary.** Inherited from Layer A's per-unit articulation. The categorization assumes each methodology unit (file) is uniformly one shape. The prior cycle's per-level constraint allowed multi-shape documents; this categorization does not.

### What this categorization requires of subsequent layers

- **Layer C** must specify a representation per category (six categories) with rationale for the format tier. Where a category has internal sub-shapes (workflow contracts: interface-typed + workflow-narrative; mechanics: recipe vs. executable code; verification: conformance vs. behavior), Layer C addresses sub-shape format choices.
- **Layer D** must specify a verification mechanism per category, matching tier to Layer C's format choice. Where Layer C achieves structural-impossibility, Layer D's conformance verification is the parser. Where Layer C settles for after-the-fact detection, Layer D includes a linter. Where neither is available, Layer D specifies the inspection procedure.
- **Layer E** must address how current substrate's mixed-shape protocols transition to the per-category cleaned-up substrate. This is the largest migration concern; the strong-tier format choice in Layer C may make in-place conversion infeasible (existing PROTOCOL.md files don't parse as workflow contracts; they have to be authored fresh).

---

## 3. Representation per category

### Inheritances to make explicit

Six representation choices currently operate in the substrate:

1. **Disciplines:** Markdown with YAML frontmatter (skills/*/SKILL.md). Body is free prose.
2. **Workflow interfaces:** TOML with implicit JSON Schema (manifest.toml's `[[protocols]]` entries; no schema currently validates them).
3. **Workflow narratives:** Markdown prose with implicit section conventions (protocols/*/PROTOCOL.md; no conventions enforced).
4. **Mechanics:** None — embedded inside protocol narratives.
5. **Artifact schemas:** JSON Schema 2020-12 (schemas/*.json).
6. **Verification specs:** Python `unittest` with `assertIn` substring assertions on prose strings.
7. **ADRs and principles:** Markdown with conventional frontmatter (numbering, status, traces-to lines).

The dominant inheritance is **Markdown as the methodology format**. Five of seven existing representations use Markdown directly. The exceptions are TOML for the typed manifest and JSON for artifact schemas — both islands of structural validation in a sea of prose. The pattern is: structure where structure was forced (typed manifest, JSON-validated artifacts); prose everywhere else.

The strongest available format-as-discipline tier per-category is the question the prior cycle answered with "Markdown + linter for workflow specifications" (the weakest tier). Day-one stance argues for the strongest tier where the costs justify it.

### Diagnostic question (per-category, not one-shot)

Per category: if no precedent existed — neither the current substrate's format choices nor the prior cycle's Markdown+linter answer — what format would I propose, and at what tier? The question fires six times in this layer (once per category), with category-specific cost-benefit reckoning each time.

### Per-category reckoning

#### Format C-1: Disciplines (cat 1, WHAT, cognitive prose)

**Inherited.** Markdown with YAML frontmatter; no schema validation on frontmatter; no body-pattern linter.

**Mistakes possible.**
- Body embeds HOW-shaped patterns: shell command syntax, MCP-tool invocations expressed as code, JSON field paths (`headRefOid`, `headRepository.name`), git command-line forms, heredoc patterns.
- Body embeds workflow-narrative content: per-protocol step descriptions, references to specific artifact-firing logic.
- Body embeds mechanic content: forge-specific recipes, runtime-specific invocations.
- Frontmatter drifts: missing required fields, malformed values, invalid frontmatter shape.

**Format candidates.**

- **Strong tier (typed prose DSL).** A custom format with structured sections (e.g., "Iron Law," "Investigation Move," "Recognition Patterns" as typed top-level keys with constrained body content). Parser rejects malformed sections. Pros: structural-impossibility for shape-violations. Cons: authoring complexity is high; cognitive disciplines are inherently prose-shaped, and constraining sections too tightly makes natural authoring awkward. The cost is felt by every contributor authoring a discipline; the benefit is detection at parse time of an already-rare class of error (skills are observably WHAT-pure today).
- **Strong-ish tier (Markdown + frontmatter schema + body pattern linter).** Frontmatter validates against a JSON Schema (name, description, metadata.version, metadata.updated, metadata.origin where applicable). Body is free prose, but a linter detects HOW-shaped patterns: regex matches against shell command grammars, MCP-tool invocation patterns, JSON-path patterns, code-block fences containing executable syntax. The linter rejects discipline files containing such patterns.
- **Weak tier (Markdown + frontmatter convention).** No body validation. Inspection-only.

**Cost-benefit.** Strong tier is over-engineered for a category where shape violations are observably rare and cognitive content is inherently prose-shaped. Weak tier is under-engineered: it does not catch the violation pattern even as it accumulates. Strong-ish tier hits the sweet spot — frontmatter is structurally validated (frontmatter drift IS observed today: `metadata.updated` dates, version strings), and body-pattern linter catches the rare HOW-leak.

**Surviving format C-1.** **Markdown body + JSON Schema-validated YAML frontmatter + body-pattern linter.** The linter operates on a pattern catalog: shell command prefixes (`gh `, `git `, `cat > `, `curl `), MCP-tool invocation shape (`<word>\(\{[^}]*instance_id`), JSON-field-path shape (in code blocks), code-block fences containing executable syntax. Pattern catalog grows as new HOW-shapes appear; linter rejects on any catalog match.

**Named tier.** **Mixed: structural-impossibility for frontmatter; after-the-fact detection for body.** The frontmatter parser rejects malformed frontmatter at parse time (strong tier for that sub-surface). The body linter detects HOW-pattern leaks after expression (after-the-fact tier for that sub-surface). The tier mix is named explicitly, not concealed.

**Tradeoff acknowledged.** A discipline file *can* contain HOW-pattern text that the linter doesn't catch (e.g., conceptual descriptions of HOW that don't match the pattern catalog). The category accepts this gap because the alternative — typed prose DSL — costs more in authoring complexity than it saves in violation prevention, given the substrate witness that disciplines are already WHAT-pure under the looser format.

#### Format C-2: Workflow contracts (cat 2, WHAT, typed)

**Inherited.** TOML for the per-protocol typed interface (in `manifest.toml`); Markdown prose for the workflow narrative (in `protocols/*/PROTOCOL.md`); the two are unlinked at the format level (no schema connecting them).

**Mistakes possible.**
- Workflow narrative embeds HOW (the witnessed mixing in submit/PROTOCOL.md). This is the dominant defect.
- Workflow narrative embeds mechanic content (specific tool invocations rather than mechanic references).
- Workflow narrative embeds discipline content (re-explaining what disciplines say rather than referencing them).
- Drift between typed interface (manifest entry) and workflow narrative (PROTOCOL.md): they describe the same protocol but cannot be cross-validated.
- Per-step intents drift: a step's stated outcome doesn't match what the workflow actually claims to deliver.

**Format candidates.**

- **Strong tier (typed structured contract: TOML/YAML with parser + JSON Schema).** The workflow contract is a single document containing per-step intents, references (to disciplines, mechanics, artifact schemas), preconditions, declared outcomes, and failure modes. The format admits no free-prose section that could absorb HOW-shapes; everything is typed fields. Mistakes are caught at parse time. Pros: structural-impossibility for the dominant defect (HOW in workflow narrative); explicit references to disciplines and mechanics enable composition-graph validation; per-step outcome declarations enable behavior verification (Layer D). Cons: prose-narrative authoring is more constrained — contributors must structure workflow contracts as typed step lists with explicit references, not as free prose.
- **Strong-ish tier (structured Markdown with required sections + linter).** The workflow contract is a Markdown document with required top-level sections (Purpose, Trigger, Requires/Produces, Steps, Failure Modes); per-step content is bullet-listed with conventional sub-headings. A linter detects: missing required sections, HOW-pattern in body, missing step outcome declarations. Pros: lower authoring complexity. Cons: linter-level enforcement (after-the-fact detection); some violations are hard to detect via patterns; the section structure is convention, not contract.
- **Weak tier (free Markdown with section conventions + body-pattern linter).** Like disciplines but with prose narrative. Pros: minimal authoring overhead. Cons: catches the HOW-pattern leak but not the structural drift between intended workflow shape and actual document content.

**Cost-benefit.** This is the category where the strong tier pays off most. Workflow contracts are the dominant broken instances in current substrate (submit's 404 lines; the witness passages identified in implement, land, decompose). The cost of authoring complexity is local to ~10 protocols; the benefit is structural-impossibility for the most-frequent and most-damaging shape violation. Day-one stance applies forcefully here: the format that *makes the wrong thing impossible* is preferred over the format that *flags it after expression*.

**Sub-structural reckoning: flow-control encoding** *(added per first-review revision 4)*.

The format-choice layer above settled on TOML+JSON Schema. The format admits an internal structure for the workflow contract; that internal structure is the *next* layer of the diagnostic question. The v1 of this section answered the format-choice question without asking the sub-structural question, and inherited "ordered array of steps" as the workflow's internal shape. Governance flagged this: workflows are not flat ordered sequences. The diagnostic question fires recursively at the sub-structural layer.

**Diagnostic question at the sub-structural layer.** What shape encodes inter-step flow within a workflow contract, if no precedent (specifically: no inheritance of "ordered array of steps" as the structural default) existed?

**Substrate witnesses for actual workflow shapes.**

- **Submit** (verified-broken witness, 404 lines). Step 4 ("Resolve PR delivery path") is a four-way conditional branching point: open-PR-with-deliverable-work → existing-PR-update path; no-open-PR-with-upstream → new-PR path; no-open-PR-no-upstream → first-push path; ancestry-divergence → terminal abort. Multiple paths converge at step 7 (`patch` delivery). Multiple terminal abort paths exit at various failure_modes ("Ambiguous PR discovery," "No local remote matches," "PR head fetch fails," etc.).
- **Land**. Phase 0 (gather/verify/review/seal) is conditionally sequential; Phase 1 (squash/discover/merge/delete/comment/deliver/report) has internal branching (squash decision based on commit history evaluation; merge path based on PR state).
- **Implement**. RED-GREEN-REFACTOR is an explicit loop construct (per-test-case iteration); fix-bug is a conditional sub-procedure invoked when failures appear.
- **Survey**. Research loop (research-record may produce; protocol may iterate based on research findings).
- **Decompose**. Per-work-unit creation is a loop; some work-units carry forward to next phase, others escalate.

At least half the protocols genuinely require branching, convergence, loops, or terminal-state distinctions. The flat-array shape does not encode any of these as first-class structure.

**Format candidates at the sub-structural layer.**

- **Sub-structural option 1: Flat ordered array of steps (inherited).** Steps execute in array order; control flow is sequential by default. Branches, loops, convergence land outside the structural surface — typically in step-intent prose ("if X, do Y") or in failure_modes responses. Pros: simple authoring; familiar list shape. Cons: branching/convergence/loops are not first-class; flow-mixing returns at the sub-structural layer (the principle's same failure mode at a deeper recursion depth); the structural-impossibility tier is undermined because non-encoded flow shapes accumulate elsewhere.

- **Sub-structural option 2: Directed graph with conditional edges.** Nodes are workflow steps; edges encode transitions, with optional conditions; one or more terminal nodes mark workflow exit. Format: `nodes` array (step records); `edges` array (transition records with `from`, `to`, optional `condition`); `terminals` array (terminal records). Pros: branching, convergence, loops, terminal-state distinctions are all first-class structure; conditions are typed references (not free prose); the structural-impossibility tier extends to flow shape. Cons: authoring complexity higher than flat array — contributors graph-think rather than list-think; reading a workflow requires graph traversal rather than linear scan; tooling for graph rendering may be needed for human consumption.

- **Sub-structural option 3: State machine.** States = workflow positions (with optional entry/exit semantics); transitions = guarded changes-of-state; terminal states explicitly declared. Pros: industry-known pattern with established tooling (graphviz, model checkers); clean operational semantics. Cons: state-machine semantics import business-process baggage (entry/exit hooks add ceremony for our use); "states" framing is awkward for protocols that are about *executing steps* rather than *occupying states*; workflow contracts are closer to procedures-with-control-flow than to states-with-transitions, structurally.

- **Sub-structural option 4: Behavior tree.** Tree-structured composition: leaf actions composed by sequence/selector/parallel control nodes. Pros: natural composition; reactive-AI heritage; fit-for-purpose for autonomous-agent action selection. Cons: tree shape cannot represent convergence (two paths leading to same node) without duplication or external references; convergence is exactly the pattern submit's `patch` delivery exhibits; the tree shape is wrong for forge-shaped workflows.

- **Sub-structural option 5: BPMN.** Business Process Model and Notation. Industry standard for business processes with rich semantic surface (events, gateways, tasks, sub-processes). Pros: mature standard; widely-tooled; admits parser/validator tooling. Cons: heavyweight for ~10 protocols; XML/JSON serialization is verbose; the standard is over-specified for this use; brings external semantic conventions that may not align with methodology principles (e.g., BPMN's notion of "user task" vs "service task" doesn't map cleanly to methodology categories).

- **Sub-structural option 6: Petri nets.** Concurrency-aware control flow with places, transitions, and tokens. Pros: concurrent execution natural; mathematically grounded. Cons: overkill for sequential/branching workflows; tooling unfamiliar; the Petri-net abstraction is deeper than methodology workflows need (no methodology workflow currently has true concurrency; sequencing with branching is sufficient).

- **Sub-structural option 7: Linear array with conditional/merge/loop step types.** A hybrid: flat array of step records, where some records are "branch" markers (with conditions and label targets), others are "merge" markers, others are "loop" markers (with exit conditions). Pros: familiar imperative shape; can be parsed sequentially. Cons: branches encoded via labels (lookup) rather than direct references; convergence requires explicit merge markers that aren't structurally distinct from steps; the format is spaghetti-shaped for complex flows; effectively an emulation of graph encoding via flat-array shape — pays graph-encoding's authoring cost without gaining graph-encoding's structural clarity.

**Cost-benefit comparison.**

Option 1 (flat array): rejected. The substrate observation (multiple protocols with real branching/convergence/loops) is incompatible with flat encoding. Inheritance from "ordered array of steps" was the failure mode this revision exists to repair.

Options 5 and 6 (BPMN, Petri nets): rejected as overinvestment. The methodology's flow-shape requirements are simpler than these standards address; the cost of importing the standards exceeds the benefit at current scale.

Option 4 (behavior tree): rejected for the convergence problem. Submit's flow has convergence at `patch` delivery; behavior trees don't admit convergence cleanly.

Option 7 (linear-array-with-marker-step-types): rejected as a degraded form of option 2. It pays graph-encoding's authoring cost (must reason about branches, merges, loops) without gaining graph-encoding's structural clarity (the graph is encoded as labels-and-lookups rather than as nodes-and-edges).

Option 3 (state machine): possible. Operational semantics are cleaner than option 2's graph; but the "state" framing imports semantic baggage that doesn't match methodology workflows (steps are *actions to execute*, not *states to occupy*); the entry/exit hook surface is ceremony without value for our use.

Option 2 (directed graph with conditional edges): survives. Branching, convergence, loops, terminal-state distinctions are all first-class structure; conditions are typed references to source-node outcomes (not free prose); the structural-impossibility tier extends to flow shape.

**Surviving sub-structural shape: directed graph with conditional edges + explicit terminals.**

A workflow contract is a directed graph: `nodes` are workflow steps; `edges` are transitions with optional conditions; `terminals` are explicit exit points. The graph is connected from a designated start node; every terminal is reachable from start; conditions on outgoing edges from a single node are exclusive (at most one edge fires per node-exit, except for explicit fan-out cases which require explicit declaration).

The flow-mixing pathway (branching landing in step intent prose or untyped failure_modes responses) is closed: branches live in `edges`; conditions are typed references to declared outcomes from source nodes; merges are convergent edges to a shared node; loops are edges-back-to-earlier-nodes; terminals are declared explicitly.

**Surviving format C-2** *(revised per sub-structural reckoning)*. **TOML document per workflow contract**, parser-validated against a JSON Schema for the workflow-contract shape. Schema specifies:

```
contract:
  name (string, must match a manifest protocol name)
  purpose (string, single-line WHAT statement)
  session_role (enum, optional: opening / middle / closing / standalone)
  preconditions (array of structured preconditions)
  start_node (string, must match a name in `nodes`)
  nodes (array of step records, unordered; order is determined by edges)
  edges (array of transition records)
  terminals (array of terminal records)
  failure_modes (object: failure_name → response_specification)
  corruption_modes (array of corruption-mode references to disciplines)

node:
  name (string, hyphen-case, unique within contract)
  intent (string, single-line WHAT statement)
  disciplines (array of discipline names; must resolve to existing skills)
  mechanics (array of mechanic names; must resolve to existing mechanic units)
  outcomes (array of structured outcome declarations; named outcomes are
            referenced by edges' conditions)

edge:
  from (string, must match a name in `nodes` or be the start_node)
  to (string, must match a name in `nodes` or `terminals`)
  condition (optional: structured reference to a source-node outcome name,
             or the special value `unconditional` for the unique exit edge)

terminal:
  name (string, hyphen-case, unique within contract)
  outcome (string, single-line declaration of what terminal state means)
  artifact_produced (string, optional; an artifact-type name from the schema
                     registry — terminals may produce artifacts that runa
                     captures as protocol output)
```

Cross-references (discipline names, mechanic names, artifact-schema names, edge endpoints, condition references) validate at parse time against the methodology's discipline / mechanic / schema / contract-internal-name registries.

**Graph-level invariants enforced at parse time:**
- Every edge's `from` and `to` resolve to existing `nodes`, `terminals`, or `start_node`.
- The graph is connected from `start_node`: every node and every terminal is reachable.
- Conditions on outgoing edges from a single node are mutually exclusive: at most one fires per node-exit (the parser enforces this; a node with multiple unconditional outgoing edges, or with overlapping conditions, is a parse error). Exception: explicit fan-out declarations (currently not motivated by any methodology protocol; may be added later if a protocol genuinely fans out concurrently).
- Loops are admissible (an edge whose `to` precedes its `from` in topological order); they must have at least one exit edge with a termination condition (the parser detects unbounded loops and rejects them).

**Where the manifest fits.** The single canonical typed-interface declaration (`manifest.toml`) survives as the topology source. Per-protocol contract files reference their protocol by name (matching a manifest entry); they do NOT redeclare the typed interface (`requires`, `produces`, `trigger`). The manifest stays canonical at the protocol-topology layer; workflow contracts add the per-protocol flow-graph specification. Cross-validation: every manifest protocol entry must have a corresponding contract file; every contract file must match a manifest protocol entry. This is enforceable at parse time.

**Named tier.** **Structural-impossibility.** The format rejects shape-violations: free-prose sections do not exist; HOW-content has no place to land; cross-references that don't resolve are parse errors; flow shapes that the inheritance would have absorbed (branching, convergence, loops, terminals) are first-class structure rather than prose-narrative content. Per the recursive sub-structural discipline, this tier extends both to top-level shape (no HOW in WHAT-units) and to flow-shape (no flow-mixing into intent prose or failure_modes responses).

**Tradeoff acknowledged.** Authoring a workflow contract requires structured thinking: identify nodes, declare per-node intent + outcomes, draw the graph (edges with conditions), declare terminals. This is *more* work than writing prose, and *more* work than the flat-array v1 articulation. The work is the discipline. The benefit is that the methodology's flow shape is itself reckoned-on rather than implicit; reading a workflow contract surfaces the actual control flow rather than burying it in conditionals embedded in step prose. Tooling: a methodology-vendored visualizer that renders the workflow graph as a directed-graph diagram is a Phase 7 deliverable (or earlier if Phase 5 contributors find graph-traversal-without-visualization too costly during authoring).

**Behavior verification fidelity gain.** The flat-array shape admitted only sequence-path testing (one path through the array). The directed-graph shape admits property-based testing: every path through the graph; every reachable node; every terminal; every transition condition. This raises the ceiling on what behavior verification *could* test if a future methodology-execution interpreter is built (deferred per Layer C-5). The current methodology layer's verification is unchanged (conformance only); the future possibility is structurally enabled.

#### Format C-3: Mechanics (cat 3, HOW, structured recipes)

**Inherited.** No format — mechanics are embedded inside protocol bodies. The substrate has no extracted mechanics yet.

**Mistakes possible.**
- Mechanic embeds workflow content (the recipe assumes a specific protocol context rather than being a reusable tool operation).
- Mechanic embeds discipline content (the recipe explains cognitive content that should be in a discipline).
- Mechanic over-specifies HOW (constrains agent execution beyond what the operation requires; agents lose HOW-sovereignty).
- Mechanic under-specifies HOW (the recipe is so abstract that agents can't infer the actual operation).
- Mechanic forge-couples without forge-tagging (a GitHub-shaped recipe that claims to be forge-neutral).

**Format candidates.**

- **Strong tier (executable code as the mechanic).** Each mechanic is an actual function/script (Python, shell). The methodology distributes a mechanic library; agents invoke library functions. Pros: end-to-end testable; one source of truth. Cons: violates Methodology Sovereignty in a different direction — the methodology now owns *implementation*, removing the agent's HOW-sovereignty. Day-one stance: this trades one violation for another.
- **Strong tier (typed structured recipe: TOML/YAML with parser + JSON Schema).** Each mechanic is a TOML document declaring: name, purpose (one-line WHAT the mechanic achieves), parameters (with types), default invocation pattern (the canonical HOW expressed as a template), expected outcome, forge-tag where applicable. Pros: structural-impossibility for shape-violations; agent retains HOW-sovereignty (default invocation is a template, not a contract). Cons: the "default invocation" is example HOW; the format must distinguish "this is the default the agent may use" from "this is the contract the agent must satisfy."
- **Weak tier (Markdown recipe with conventional sections).** Like disciplines. Cons: re-creates the protocol-narrative pattern; HOW-content sits in free prose; no structural enforcement.

**Cost-benefit.** Strong tier with structured recipe is the right call. The methodology distributes mechanics as recipes; agents invoke them as templates with HOW-sovereignty over substitution. The structured format makes the mechanic's interface (parameters, outcome) parser-checkable while leaving the invocation template as a string field that the agent reads as advisory HOW.

**Surviving format C-3.** **TOML document per mechanic**, parser-validated against a JSON Schema for the mechanic shape. Schema specifies:

```
mechanic:
  name (string, hyphen-case, namespaced: forge.github.create-pr)
  purpose (string, single-line WHAT statement)
  forge_tag (enum, optional: github / forgejo / generic / runtime / git / shell)
  parameters (object: param_name → param_spec)
  default_invocation (string: template with parameter substitution)
  outcome (string, single-line declaration)
  examples (array of examples with parameter values and resulting invocation)
```

Cross-references: parameter shapes may reference artifact schemas; outcome may reference artifact-types.

**Forge-neutrality structurally encoded.** The `forge_tag` field is required for forge-specific mechanics. A workflow contract's `mechanics` reference resolves to a *family* of mechanics (e.g., `create-pr` resolves to whichever forge-tagged mechanic the active forge requires). Forge-substitution is at the methodology-config layer, not at the workflow-contract layer.

**Named tier.** **Structural-impossibility.** The mechanic file's shape is parser-validated; cross-references are parse-time-checked; the forge-tag is required where forge-specific content appears.

**Tradeoff acknowledged.** The `default_invocation` field is a free-string template — the parser cannot verify that the template is well-formed shell or correct gh syntax. That verification is at execution time, not at parse time. The mechanic format protects against shape-violations (workflow content in mechanics) but does not protect against syntax errors in the template itself. This is acceptable: template syntax verification is the agent's responsibility (HOW-sovereignty), not the methodology's.

#### Format C-4: Artifact schemas (cat 4, WHAT, typed contracts)

**Inherited.** JSON Schema 2020-12 (12 schemas in `schemas/`).

**Mistakes possible.**
- Schema doesn't match actual artifact instances produced by protocols.
- Schemas drift between protocols (one protocol produces a shape the next protocol's schema rejects).
- Schemas duplicate across artifact types (similar fields not factored into a base shape).
- Schema language doesn't express needed constraints (e.g., conditional required-ness, cross-field invariants).

**Format candidates.**

- **JSON Schema 2020-12 (current).** Pros: widest tooling, broadest community, mature validation. Cons: expressivity has known gaps (cross-field invariants, conditional required-ness express awkwardly).
- **CUE.** Pros: stronger type system, can express more constraints; unification semantics. Cons: tooling less ubiquitous; learning curve.
- **JSON Type Definition (JTD, RFC 8927).** Pros: simpler than JSON Schema, less ambiguity. Cons: less expressive; ecosystem smaller.
- **Protobuf.** Pros: binary efficient, strict typing, cross-language. Cons: assumes binary serialization; over-investment for JSON-shaped artifacts.

**Cost-benefit.** JSON Schema 2020-12 is fit-for-purpose at this scale. The expressivity gaps (cross-field invariants) are real but don't fire on the artifact shapes the methodology currently uses (artifacts are structurally simple: required fields, optional metadata, typed values). CUE would buy expressivity that isn't needed; the cost (community size, learning) outweighs the benefit. The diagnostic question's answer: I'd propose JSON Schema 2020-12 if no precedent existed; the inheritance survives because it's the right answer, not because it's inherited.

**Surviving format C-4.** **JSON Schema 2020-12.** Inheritance retained with first-principles ratification.

**Named tier.** **Structural-impossibility.** Validators reject malformed instances at parse/validation time.

**Tradeoff acknowledged.** Cross-field invariants require schema gymnastics (conditional `if`/`then`/`else` blocks) and may degrade readability. Where invariants exceed JSON Schema's expressivity, supplemental validators may be needed (validation code that runs after schema validation). This is a known JSON Schema limitation; it does not motivate replacement at this scale.

#### Format C-5: Verification specifications (cat 5, HOW with referenced WHAT)

**Inherited.** Python `unittest` with `assertIn` substring assertions on protocol prose. 28 assertions in `tests/test_submit_protocol.py`. The pattern is the residual of a substrate where workflow contracts cannot be parsed: behavior verification is unavailable, so substring-presence is the only available test.

**Mistakes possible.**
- Tests verify the wrong property (substring presence rather than behavioral conformance).
- Tests drift: the protocol's prose changes, the test still passes (substring still present), but the protocol's behavior has changed.
- Tests are brittle to refactoring: rewording the prose breaks tests without changing semantics.
- Tests duplicate effort: each protocol gets per-protocol substring assertions, none of which compose.

**Format candidates.**

The format question for verification depends on what's verified. Two distinct verification kinds (Layer B):

**(a) Conformance verification — does a methodology unit match its shape's grammar?**

- **Format candidate A1: parser as verification.** The Layer C-1 through C-4 format choices already define parsers (frontmatter validators, JSON Schema validators, TOML+schema validators). Conformance verification IS the parser running. No separate verification spec is needed for conformance.
- **Format candidate A2: separate linter spec.** The body-pattern linter for disciplines (Layer C-1) and the cross-reference resolver for workflow contracts (Layer C-2) live as separate executable code. The "verification spec" is the linter code itself.

These together cover conformance verification: parsers + linters. No separate test specifications needed for shape-conformance.

**(b) Behavior verification — when this protocol is fired with these inputs, does the declared outcome occur?**

- **Format candidate B1: behavior test specifications.** Each workflow contract step's `outcomes` field declares what the step achieves. Behavior verification fires the protocol in a sandboxed environment, observes the artifacts produced (or other observable outcomes), and validates against the declared outcomes. The verification spec format: TOML or YAML test cases (input artifacts, fired protocol, expected outcomes), with a test runner that executes them. Pros: behavior-level verification at the methodology layer; outcomes declared in workflow contracts are the contract being verified. Cons: requires methodology-execution infrastructure (a runa-like interpreter or a build-step interpreter that can fire workflow contracts). The infrastructure may not exist yet.
- **Format candidate B2: integration tests at the project layer.** Methodology-using projects exercise the methodology end-to-end; behavior tests live there, not in the methodology. The methodology itself does not specify behavior tests. Pros: simpler methodology; no test-runner infrastructure required. Cons: no methodology-level behavior verification; outcomes go unverified across projects.
- **Format candidate B3: mixed — conformance only.** Behavior verification doesn't happen at the methodology level. The methodology exposes conformance verification (parser-level) and trusts integration testing in user projects to verify behavior. This is what #255 reckons: documentation-only protocols may need no behavior verification at all.

**Cost-benefit for (b).** B1 is the strong-tier answer if methodology-execution infrastructure exists. The infrastructure cost is significant (runa or a build-step interpreter that fires workflow contracts and observes outcomes). At the current methodology scale (10 protocols, none of which have direct behavior outcomes beyond producing artifacts that runa already validates), the marginal benefit may not justify the infrastructure cost. B3 is the pragmatic answer: conformance only at the methodology level; behavior verification happens at integration time in user projects (or via the Phase-E babbie test for `v0.1.2`).

**The #255 question, addressed.** #255 asks whether documentation-only protocols should have automated behavioral tests. Per this categorization, the answer is: the methodology specifies *conformance verification* (does this unit conform to its shape?) but does *not* specify *behavior verification* at the methodology layer. Behavior verification, where it happens, is integration-level (project-using-the-methodology runs the methodology end-to-end and observes outcomes). The substring-presence pattern in `tests/test_submit_protocol.py` is replaced by structural conformance verification (the workflow contract for submit parses; its references resolve; its declared outcomes are well-formed). The verification surface shifts from "phrases present in prose" to "shape conforms to grammar."

**Surviving format C-5.**

- **Conformance verification:** code (Python or otherwise) that runs the parsers + linters across the methodology. The verification "spec" IS the code. Output: structured pass/fail report per unit. Format: depends on language; for Python implementation, `pytest`-style test runner with one test per unit.
- **Behavior verification:** at the methodology layer, **not specified**. The methodology declares outcomes in workflow contracts (Layer C-2's `outcomes` field per step); user projects exercise the methodology and may verify outcomes integration-style. Where the methodology produces a runa-readable spec, runa's existing artifact validation covers per-step artifact correctness.

**Named tier.**
- Conformance verification: **structural-impossibility for shape; after-the-fact detection for body-patterns**. Tier mirrors C-1 through C-4 per-category.
- Behavior verification: **deferred to integration layer (user projects); not specified at methodology layer**. This is an explicit non-specification; the rationale is that methodology-execution infrastructure does not exist at the granularity required, and the cost of building it exceeds the benefit at current scale.

**Tradeoff acknowledged.** Behavior verification at the methodology level is genuinely useful (some protocols may have observable outcomes worth testing), and not specifying it leaves a real gap. The deferral is pragmatic: at current scale, conformance verification carries the load; behavior verification can be added later when the methodology grows past where shape-conformance suffices. The surviving non-specification is reviewable: future governance reckonings may decide to specify behavior verification when scale or risk increases.

#### Format C-6: ADRs and foundational principles (cat 6, WHAT, prose)

**Inherited.** Markdown with conventional frontmatter (Status, Date, Traces to). No JSON Schema validation on frontmatter; conventions are observed by authors but not enforced.

**Mistakes possible.**
- ADR frontmatter drifts (numbering inconsistency, missing required fields).
- ADR body doesn't actually trace to the principle the frontmatter cites.
- Principles drift from the substrate they describe.
- Principles document new principles silently (ecosystem-level commission missed).

**Format candidates.**

Same family as Layer C-1 (disciplines): Markdown + structured frontmatter + body inspection.

**Surviving format C-6.** **Markdown body + JSON Schema-validated frontmatter.** Frontmatter shape: ADRs require status, date, traces-to (one or more principle numbers); principles require number, name, topology-position. Body is prose; no body-pattern linter required (ADRs and principles do not have HOW-shape leakage as a class — the substrate witness shows zero ADRs/principles with embedded shell or tool syntax).

**Named tier.** **Structural-impossibility for frontmatter; inspection-only for body.** ADR/principle bodies are foundational reasoning that does not lend itself to mechanical body validation. Inspection by reviewers (governance, contributors during PR review) carries the body-quality load.

**Tradeoff acknowledged.** Inspection-only for body means principle-articulation drift is caught at review time, not at parse time. This is acceptable because principle-articulation is a low-frequency, high-attention activity (each principle commission is itself a governance reckoning); the inspection load is manageable.

### Format choice summary

| Category | Format | Tier (frontmatter / body) |
|---|---|---|
| C-1 Disciplines | Markdown + JSON Schema frontmatter + body-pattern linter | structural-impossibility / after-the-fact detection |
| C-2 Workflow contracts | TOML + JSON Schema | structural-impossibility |
| C-3 Mechanics | TOML + JSON Schema | structural-impossibility |
| C-4 Artifact schemas | JSON Schema 2020-12 | structural-impossibility |
| C-5 Verification (conformance) | Code (parsers + linters as test runners) | structural-impossibility for shape; after-the-fact for body |
| C-5 Verification (behavior) | Not specified at methodology layer | deferred to integration |
| C-6 ADRs and principles | Markdown + JSON Schema frontmatter | structural-impossibility / inspection |

Five of six categories operate at structural-impossibility tier for the dominant shape constraint. The exceptions are:

- **C-1 disciplines body**: prose body is inherently flexible; body-pattern linter is the strongest realistic tier.
- **C-6 ADR/principle body**: foundational reasoning; inspection is the strongest realistic tier.
- **C-5 behavior verification**: deferred entirely; the cost of methodology-execution infrastructure exceeds benefit at current scale.

Where structural-impossibility doesn't apply, the cost-benefit reckoning is named explicitly per-category, not concealed.

### Inheritances retained at this layer

- **JSON Schema 2020-12 for artifact schemas (C-4).** Retained because the diagnostic question's answer is the same as the inherited choice. First-principles ratification, not blind inheritance.
- **TOML for typed declarations (C-2 workflow contracts, C-3 mechanics).** Inherited from `manifest.toml`'s precedent. Retained because TOML is fit-for-purpose for typed declarative data with comments and human readability.
- **`manifest.toml` as canonical topology declaration.** Retained per the commission's affirmed insight ("manifest.toml IS the canonical workflow topology"); per-protocol contract files reference it without duplicating its content.
- **Markdown for cognitive prose categories (C-1, C-6).** Retained because cognitive prose is inherently prose-shaped; constraining sections beyond what the principle requires costs more than it saves.

### Inheritances rejected at this layer

- **Markdown as the format for workflow narratives (currently `protocols/*/PROTOCOL.md`).** Rejected. Replaced by C-2 typed TOML contracts. The substrate observation (submit's 404 lines) is the rejection's evidence.
- **Embedded HOW in workflow narratives.** Rejected (and replaced by extracted mechanic units in C-3).
- **Substring-presence verification (`tests/test_submit_protocol.py` pattern).** Rejected. Replaced by C-5 conformance verification (parsers + linters); behavior verification deferred to integration layer.

### What this layer requires of subsequent layers

- **Layer D** must specify the conformance verification mechanism per category. For C-1 through C-4 and C-6, parsers/validators do most of the work; for C-5, the verification spec format is itself the verification mechanism (recursive). Acid tests apply per-category.
- **Layer E** must address the migration consequence: existing `protocols/*/PROTOCOL.md` files do not parse as C-2 workflow contracts. They cannot be in-place converted; they must be authored fresh against the new format. This rules out the prior cycle's in-place conversion strategy at this layer.

---

## 4. Conformance verification

### Inherited verification approach

Two extant patterns operate in current substrate:

1. **Substring-presence assertions** (`tests/test_submit_protocol.py`, 28 of them): tests verify that specific phrases exist in protocol prose. The pattern catches phrase-deletion regressions but does not verify behavioral or structural conformance. Issue #255 questions whether this approach has any earned place.
2. **Schema-vendoring discipline** (`tests/test_request_schema_vendoring.py`): one focused test that verifies the canonically-vendored `request` schema matches its commons source. Behavior-level test of a structural invariant. The pattern is fit-for-purpose for what it tests; it does not generalize to methodology-conformance.

The prior cycle (issue #256) proposed: **a "conformance linter" plus inspection rubric**, with the acid test "any protocol or skill admits a 'does this conform?' answer that is either machine-checkable or reducible to a small, well-defined inspection procedure."

The acid test framing itself is inherited. The commission named four additional candidate acid tests worth reckoning on:

- **Authoring obviousness:** can a new contributor produce a correct methodology unit on first attempt?
- **Composition cleanliness:** does the methodology compose cleanly with extensions and overlays?
- **Graceful degradation:** does the methodology degrade gracefully under partial implementation?
- **Substrate longevity:** does the methodology resist the accumulation pattern that produced submit's 404 lines?

### Diagnostic question

What conformance verification mechanism would I propose per-category if no precedent existed, and which acid tests should the redesign be measured against?

### Reckoning

**Verification tier follows format tier.** Layer C established per-category format tiers. Verification mechanism matches:

- Where the format is **structural-impossibility** (C-2 workflow contracts, C-3 mechanics, C-4 artifact schemas, C-6 frontmatter): the parser IS the conformance verifier. Conformance verification reduces to "does this unit parse?" — a mechanical, pass/fail, P7-aligned check. No separate verification spec is needed; the parser's existence verifies.
- Where the format is **after-the-fact detection** (C-1 discipline body, C-6 ADR body): a linter operates on the body content, applying pattern catalogs to detect violations. The verification mechanism is the linter; conformance is "does the linter find any cataloged violation?"
- Where the format is **inspection-only** (C-6 ADR body — for substantive content beyond pattern detection): conformance is verified by review at PR time. The verification mechanism is a structured inspection procedure (a rubric the reviewer applies).
- Where the format is **deferred** (C-5 behavior verification): conformance at the methodology layer is not specified; behavior verification happens at integration time in user projects.

**Two distinct verification kinds, not one.** The Layer B distinction (conformance verification vs. behavior verification) matters for the acid-test fit. Conformance verification is mechanical at the unit level; behavior verification is integration-level (or absent at the methodology layer).

**The four candidate acid tests, applied to the redesign.**

1. **Authoring obviousness.** Can a new contributor produce a correct methodology unit on first attempt? The redesign's per-category format tiers help here: parser errors at parse time are *immediate, mechanical, specific* — a contributor authoring a workflow contract gets parse errors that point to the offending field. The cost is the tier's authoring complexity (TOML+schema is more constraining than free Markdown). A new contributor authoring their first workflow contract needs an authoring guide (Layer C's deliverable shape) and a working example (the migration produces these). With these in place, authoring obviousness is a pass: the parser's errors are clear, the schema documents required fields, examples illustrate correct structure.

2. **Composition cleanliness.** Does the methodology compose with extensions and overlays? Two relevant compositions:
   - **Forge-specific overlays:** the C-3 mechanics format's `forge_tag` field structurally supports forge-substitution. A Forgejo overlay adds Forgejo-tagged mechanics alongside GitHub-tagged ones; workflow contracts reference operations (e.g., `create-pr`) that resolve to whichever forge-tagged mechanic the methodology config selects. Composition is structurally clean.
   - **Domain-specific overlays:** a domain methodology (e.g., security-review) extends groundwork by adding new disciplines, new workflow contracts, new mechanics. The category boundaries (Layer B) and per-category formats (Layer C) accept extensions natively: new disciplines drop into a discipline registry; new workflow contracts add manifest entries; new mechanics add to the mechanic registry. Cross-references resolve at parse time across registries, including across overlays. Composition is structurally clean.
   - **Successor methodologies:** if a later methodology supersedes groundwork at any layer (e.g., a stricter principle articulation at Layer A; a richer category cut at Layer B; a different format at Layer C), the redesign's per-category tier-named structure makes the migration legible — the acid test for THAT migration is "does the new tier dominate the current tier?" Composition with successor methodologies is cleaner because the current methodology is itself layered and named.

3. **Graceful degradation.** Does the methodology degrade gracefully under partial implementation? This is the migration-state question: while some protocols are converted to C-2 workflow contracts and others remain in current `protocols/*/PROTOCOL.md` form, does the methodology still function? The clean answer: the parser-checks operate per-unit. Units that don't parse (because they're still in old form) are flagged but don't break units that do parse. The methodology functions in mixed-state. *However*, the strong-tier choice at C-2 means the old `PROTOCOL.md` files cannot be parsed as C-2 contracts — they're a different format entirely. Graceful degradation requires that the methodology distinguish "old format" from "new format" during migration, with both formats coexisting until conversion completes. Layer E reckons; this acid test fires on the migration strategy more than on the format choices themselves.

4. **Substrate longevity.** Does the methodology resist the accumulation pattern that produced submit's 404 lines? This is the redesign's *whole point*. Submit's 404 lines accumulated because the format admitted accumulation — every review round added a substantively-correct fix into the prose, and the cumulative result was unparseable for behavior. The C-2 workflow contract format makes this accumulation structurally impossible: the typed structure has no free-prose section that can absorb new fixes. A new fix to submit's behavior either:
   - Updates a step's `intent`, `disciplines`, `mechanics`, or `outcomes` declarations (typed, parser-checked) — accumulation is bounded by the number of meaningful step-shape changes.
   - Updates the workflow contract's `failure_modes` (typed, parser-checked) — accumulation is bounded by enumerable failure cases.
   - Updates a referenced mechanic (forge-specific recipe) — accumulation is bounded by the mechanic's own shape.
   - Adds a new discipline reference or mechanic reference — accumulation is bounded by registry size.
   No fix can land as "another bullet in step X's prose narrative" because step X has no prose narrative. The substrate longevity acid test is structurally satisfied by the format choice. *This is the redesign's primary success condition.*

**The prior cycle's acid test, evaluated.** "Machine-checkable OR reducible to a small, well-defined inspection procedure." The redesign is more specific: per-category tier (structural-impossibility / after-the-fact detection / inspection-only / deferred) is named explicitly. The prior cycle's binary (machine-checkable / inspection-procedure) is subsumed under the more granular tier specification.

### Surviving verification mechanism

#### Per-category specification

| Category | Verification mechanism | Tier | Acid test fit |
|---|---|---|---|
| C-1 Disciplines | (i) frontmatter parser; (ii) body-pattern linter | structural-impossibility / after-the-fact detection | substrate longevity ✓ (linter catches HOW-leak accumulation); authoring obviousness ✓ (pattern catalog documents disallowed forms) |
| C-2 Workflow contracts | TOML+schema parser; cross-reference resolver against discipline / mechanic / artifact-schema / manifest registries | structural-impossibility | substrate longevity ✓ (no free-prose accumulation surface); authoring obviousness ✓ (parser errors are specific) |
| C-3 Mechanics | TOML+schema parser; cross-reference resolver | structural-impossibility | composition cleanliness ✓ (forge-tag enables overlay) |
| C-4 Artifact schemas | JSON Schema validator | structural-impossibility | (legacy choice; verification was already strong-tier) |
| C-5 Conformance verification | The verification code itself: a build step or CI hook that runs C-1 through C-4, C-6 verifications across all units | structural-impossibility for shape | substrate longevity ✓ (composes the per-category checks) |
| C-5 Behavior verification | Not specified at methodology layer | deferred | (acid tests fire at integration layer in user projects) |
| C-6 ADRs and principles | Frontmatter parser + body inspection rubric (review-time) | structural-impossibility for frontmatter / inspection-only for body | (low-frequency category; inspection load is manageable) |

**Tooling shape for C-5 conformance runner.** A single build step (Python script, runa-invoked, or methodology-vendored) walks the methodology's directory tree, dispatches each unit to its per-category verifier, aggregates pass/fail, and reports. The runner exits non-zero on any failure. This composes with CI naturally: a methodology PR runs the conformance runner; failures block merge.

**Tooling shape for the inspection rubric (C-6 body content).** A documented inspection procedure: when reviewing an ADR or principle PR, the reviewer applies a checklist of questions (does the decision trace cleanly to the cited principle? are consequences operationalized? is the framing consistent with prior ADRs/principles?). The rubric is not machine-checked; it's a small, well-defined inspection procedure (the prior cycle's acid test framing) for content that admits no mechanical check.

### Acid tests, chosen

The redesign is measured against three primary acid tests:

1. **Substrate longevity** (primary; the redesign exists to satisfy this). Met by C-2's structural-impossibility tier. The format admits no free-prose section into which fixes accumulate.
2. **Authoring obviousness** (operational). Met by per-category parser feedback + authoring guides + working examples.
3. **Composition cleanliness** (architectural). Met by C-3's forge-tagging, the per-category registry shape, and parse-time cross-reference resolution.

**Graceful degradation** is deferred to Layer E (migration concern); the format choices admit mixed-state coexistence at the substrate level, but whether they're invoked simultaneously is a migration-strategy question.

The prior cycle's acid test (machine-checkable OR inspection-procedure) is subsumed: the per-category tier is more specific.

### Verification mechanism integration

*(Revised post-first-review per governance request: v1 specified a third "activation time" tier where runa loads methodology and runs conformance verification. That tier implied a runa-side change, which is out of scope per locked governance. The revision drops the activation-time tier. The deployment surface is two-tier; the substrate-first reading on whether the third tier was load-bearing concluded it was not.)*

The conformance runner (C-5 above) operates at two points in the methodology lifecycle:

1. **Authoring time.** A methodology contributor running `verify` (or equivalent local command) on their working tree gets per-unit pass/fail with specific errors. Authoring obviousness fires here.
2. **PR time.** CI runs the conformance runner on the PR's tree. Substrate longevity is enforced here — units that violate shape (HOW-leakage, broken cross-references, unparseable contracts) block merge.

These two points correspond to two distinct sovereignty interfaces:

- Authoring time: contributor ↔ methodology source (the contributor owns HOW-of-authoring; the methodology declares WHAT must hold).
- PR time: project ↔ shared methodology (the project's CI enforces methodology invariants).

This two-tier verification deployment is a fractal application of P1 sovereignty — each interface has a clean ownership boundary, and verification operates at each independently. The deployment is groundwork-internal; no runa-side change is implied.

**Why the third tier was dropped.** The v1 articulation specified a third "activation time" tier where runa loads methodology and runs conformance verification before activating any protocol. The substrate-first reading: if the methodology is conformant when merged (PR-time gating), and the methodology is self-contained (no external registry dependencies that could change post-merge), drift between merge and activation is structurally rare. Authoring-time + PR-time gating cover the substantive verification surface. The third tier added elegance (a fractal at three interfaces matching three of ADR-0001's enumerated interfaces) but not load-bearing verification value, and it imposed a runa-side coupling that locked governance excludes from this commission's scope. Day-one stance: drop the elegance, keep the foundation. The two-tier deployment is honest about what the methodology can verify within its own sovereignty boundary.

### Inheritances retained at this layer

- **Schema-vendoring discipline test pattern** (`test_request_schema_vendoring.py`): retained as a pattern. The redesign's conformance runner extends this pattern (per-unit parser checks); the schema-vendoring test is the seed example of the pattern's correct application.
- **Acid test as a deliberate framing** (the prior cycle's articulation): retained. The redesign just specifies more granular tiers within the framing.
- **CI-as-enforcement-surface**: retained. CI was already the verification execution surface; the redesign extends it with the conformance runner.

### Inheritances rejected at this layer

- **Substring-presence assertions on prose** (`test_submit_protocol.py` pattern): rejected. The format choice at C-2 makes them obsolete; the prose they verify does not exist in the new format. Replacement: workflow-contract parser (C-2) + cross-reference resolver.
- **The implicit "all-units mode" linter** (the prior cycle's acceptance criterion): rejected as the *only* enforcement surface. Replaced by per-category verification dispatched by the conformance runner; the runner operates per-unit, with type-specific checks, and aggregates rather than running a monolithic "all-units" pass.
- **Behavior verification at the methodology layer**: rejected (deferred to integration). The cost of methodology-execution infrastructure exceeds the benefit at current scale; behavior verification happens in user projects, not in the methodology itself.

### What this layer requires of subsequent layers

- **Layer E** addresses migration: the conformance runner cannot operate on units that don't conform during transition. Migration must produce conformant units before activating the runner; until then, the runner operates on whatever subset is conformant and reports the rest as in-flight migration.
- **Layer E** also addresses graceful degradation: with strong-tier formats, mixed-state requires explicit format coexistence (old-format and new-format units side by side, with the runner ignoring old-format units until they convert).

---

## 5. Migration strategy

### Inherited migration shape

The prior cycle (issue #256) assumed **in-place conversion**: existing `protocols/*/PROTOCOL.md` files would be edited to extract HOW into separate mechanic units, leaving WHAT-only protocol bodies. The decomposition was six phases (A: Substrate; B: Discipline tesserae; C: take conversion; D: Full execution-cascade; E: Architecture documentation; F: Integration verification), with 26 child issues across the phases.

Two structural assumptions in this shape are inherited heuristics:

1. **In-place conversion as the migration mode.** Existing files are edited to conform; nothing is fresh-authored.
2. **Per-phase decomposition into 26 issues.** One epic, one milestone (or unmilestoned), 26 children executed in order.

### Diagnostic question

What migration shape would I propose if no precedent existed, given Layer C's strong-tier format choices?

### Reckoning

**The Layer C consequence: in-place conversion is infeasible.** Layer C's surviving format for workflow contracts (C-2) is TOML+JSON Schema with structural-impossibility tier. Existing `protocols/*/PROTOCOL.md` files are Markdown prose; they are not TOML, they are not parser-checkable, they are not editable into the new format. They are *replaceable*, not editable. The prior cycle's in-place conversion does not survive the strong-tier format choice.

**Migration shape candidates.**

- **Shape A: Big bang.** All protocols converted in one PR. Substrate (parsers, schemas), all 10 workflow contracts, all extracted mechanics, conformance runner — one merge. Pros: clean cutover; no mixed-state to manage. Cons: PR size is infeasible to review; risk of design errors discovered post-merge is high; no incremental governance feedback. Day-one stance: the foundation is moved in one step, which sounds aligned, but in practice the lack of incremental feedback exceeds the cost of a longer migration timeline. Rejected.

- **Shape B: Per-protocol incremental, no phasing.** Each protocol gets its own conversion PR. Substrate is whatever exists at the start. Cons: parsers / conformance infrastructure needed before any contract can be authored; if substrate isn't ready, the first protocol's PR has to bring substrate with it, ballooning to substrate+protocol scope. Also: no phase structure means no point at which "discipline conformance" or "mechanic library" is independently reviewable. Rejected as too unstructured.

- **Shape C: Phased category rollout, fresh-authored per protocol.** Substrate first (Phase 1: schemas, parsers, conformance runner). Then disciplines (Phase 2: lift-and-fit; skills are already WHAT-pure under the loose format, so this phase adds frontmatter schemas + body-pattern linter). Then artifact schemas (Phase 3: minor — JSON Schema 2020-12 retained; substantive refinement only where invariants exceed the inherited expressivity). Then mechanic library (Phase 4: identify and extract reusable HOW recipes from current protocol bodies; author as C-3 mechanic units with forge-tagging). Then per-protocol workflow contracts (Phase 5: one PR per protocol; each PR authors the new C-2 contract referencing existing skills + extracted mechanics + artifact schemas; old PROTOCOL.md is removed in same PR). Then verification migration (Phase 6: replace `tests/test_submit_protocol.py` substring assertions with conformance runner coverage). Then cleanup (Phase 7: removed unused old-format scaffolding; documentation regenerated; manifest cross-validated). Pros: each phase is independently reviewable and produces a deliverable; substrate exists before contracts depend on it; mechanic library exists before contracts reference mechanics; per-protocol PRs are bounded in scope. Cons: longer timeline; methodology is in mixed-state (some protocols converted, others not) for the duration of Phase 5. Mixed-state is acceptable because the conformance runner operates per-unit (not all-units monolithically), so unconverted protocols don't break converted protocols' verification.

- **Shape D: Generative.** Author a higher-level spec from which existing units derive; replace existing units with generated outputs. Pros: single source of truth; mechanical translation. Cons: requires the higher-level spec to *exist*, and authoring it is itself a methodology design problem of equal complexity. The spec would need to express disciplines, workflow contracts, mechanics, and schemas in a unified higher-level language. The cost is tantamount to designing a meta-methodology before applying it. Day-one stance: the foundation is shifted to a not-yet-built spec, which is foundation-shifting in the wrong direction (away from concrete substrate, toward abstraction whose own foundation is unverified). Rejected as overinvestment for the current scale.

- **Shape E: Fresh authoring informed by what was learned, with parallel substrate during transition.** New methodology directory tree (`groundwork-v2/` or similar) is authored fresh; old methodology continues to operate until new is complete. At cutover, old is removed and new takes its place. Pros: no in-place migration mess; both versions reviewable; cutover is a single act. Cons: substrate duplication during transition (two methodology trees); coordination cost is high; runa/agentd integration must support both during transition. Rejected for groundwork specifically (the duplication cost is high; coordination across `tesserine/runa` and `tesserine/agentd` integration is out of commission scope).

**Shape C survives.** Phased category rollout with fresh-authored per-protocol contracts. The phases mirror the principle's category cuts: substrate, then per-category conformance, then per-protocol contract authoring. Per-phase governance review (each phase is itself a sub-epic the governance reviews and accepts before the next phase begins).

**The phase sequence, specified.**

- **Phase 1 — Substrate.** Define JSON Schemas for: discipline frontmatter, workflow contract, mechanic, ADR/principle frontmatter. Build parsers (TOML+schema for contracts and mechanics; YAML+schema for frontmatter). Build the body-pattern linter for cognitive prose categories (C-1, C-6). Build the conformance runner that dispatches per-category checks. CI integration: runner blocks merge on any failure.
- **Phase 2 — Discipline conformance.** Add frontmatter schema validation for the six existing skills (`contract`, `debug`, `orient`, `reckon`, `research`, `resolve`); apply body-pattern linter; resolve any violations (skills are observably WHAT-pure today, so violations should be minimal). Add per-skill registry entry that exposes the discipline by name to workflow contracts. The skill bodies themselves are largely unchanged; this phase is structural validation, not content rewrite.
- **Phase 3 — Artifact schema review.** JSON Schema 2020-12 inheritance survived Layer C; this phase reviews each schema for any clean-up required by the new architecture (cross-references between schemas; conformance with the registry shape). Likely a small phase.
- **Phase 4 — Mechanic library.** Identify reusable HOW recipes across existing protocol bodies: forge operations (create-pr, fetch-issue, list-prs, edit-pr-body), MCP-tool invocations (patch, test-evidence, completion-evidence, etc.), git operations (rev-parse, merge-base, fetch, push variants), shell patterns (heredoc body transport, branch derivation patterns). Extract into C-3 mechanic units with forge-tagging where applicable. Cross-reference resolution: each mechanic's `parameters` and `outcome` fields reference artifact schemas where applicable. Output: a mechanic registry that workflow contracts can reference. The mechanic library exists before any workflow contract is authored against it (Phase 5 depends on Phase 4).
- **Phase 5 — Per-protocol workflow contracts.** One PR per protocol, ordered by complexity (smallest/cleanest first; submit last). Each PR: (a) authors the new C-2 workflow contract for the protocol; (b) verifies cross-references resolve; (c) removes the old `protocols/X/PROTOCOL.md`; (d) updates manifest if the protocol's interface changed in any way. Recommended order (smallest workflow narrative to largest, allowing learning to compound): `verify` → `take` → `plan` → `specify` → `document` → `survey` → `decompose` → `land` → `implement` → `submit`. Submit last because it is the verified-broken witness; converting it last allows the entire mechanic library to be ready and the contract format to be exercised on simpler protocols first.
- **Phase 6 — Verification migration.** Replace `tests/test_submit_protocol.py` substring assertions with the conformance runner's per-unit checks. Per-protocol behavior tests (where they meaningfully exist) move to integration-level testing in user projects, NOT in the methodology. The conformance runner is the methodology's only verification surface (per Layer D).
- **Phase 7 — Cleanup.** Remove unused scaffolding (old `protocols/X/PROTOCOL.md` files, deprecated test patterns, any transitional shims). Regenerate documentation: `docs/architecture/connecting-structure.md` is updated to reference the new architecture (likely a substantial rewrite given the document's 1152-line current size); `docs/authoring/` gains per-category authoring guides (workflow contract authoring guide, mechanic authoring guide, ADR authoring guide); README's "What Groundwork Believes" section (currently around line 67, naming sovereignty in inter-protocol form) is updated to extend the principle articulation to intra-unit form per Layer A.

**Graceful degradation during phases.** The conformance runner operates per-unit. During Phase 5 (per-protocol contract authoring, multiple PRs over time), the substrate is in mixed-state: some protocols have new C-2 contract files; others still have old `PROTOCOL.md` files. The runner verifies the new contract files (parser-checked, cross-references resolved); it ignores or warns on old `PROTOCOL.md` files (they're not the new format, so the runner doesn't treat them as workflow contracts). The methodology continues to function in mixed-state because runa's existing protocol firing operates against `manifest.toml`'s typed interfaces, which are unchanged across the migration.

**Dependency on `v0.1.2` closure.** The locked governance constraint is that implementation begins after `v0.1.2` (Phase E babbie test for #220) closes. The phased migration does not begin until then. This commission's first-return acceptance + governance gating produces the design; implementation phases proceed once `v0.1.2` is closed.

**Per-phase governance reckoning.** Each phase is a governance arc:
- Phase 1's deliverable (substrate) is reviewed for: schema correctness, parser correctness, conformance runner integration. Acceptance gates Phase 2.
- Phase 2's deliverable (discipline conformance) is reviewed for: schema validity per skill, linter violations resolved. Acceptance gates Phase 3.
- ... and so on through Phase 7.

Per-phase acceptance prevents the prior cycle's failure mode (the entire architecture filed at once, with frame-blindness invisible until post-filing reckoning).

### Surviving migration strategy

**Phased category rollout (Shape C), fresh-authoring per protocol, seven phases sequenced with per-phase governance acceptance.** In-place conversion does not survive the format choice; existing `protocols/*/PROTOCOL.md` files are replaced, not edited.

The substantive divergence from the prior cycle:
- Prior: 6 phases, 26 child issues, in-place conversion.
- Surviving: 7 phases, decomposition deferred to vehicle proposal (Section 6), fresh-authoring per protocol.

The phase number difference (6 vs. 7) reflects: the prior cycle bundled "Architecture documentation" as Phase E and "Integration verification" as Phase F; the surviving migration distributes architecture documentation across all phases (each phase updates docs as needed) and isolates verification migration as a distinct phase (Phase 6). The added Phase 7 (Cleanup) covers the substrate hygiene that the prior cycle's Phase F implicitly assumed but did not isolate.

---

## 6. Vehicle proposal

### Inherited vehicle

The prior cycle (issue #256): **filed GitHub epic with 26 milestone-tagged child issues** under the `tesserine/groundwork` repository. The epic itself was unmilestoned; children were milestoned per phase (with the locked governance milestone displacement specifying epic = v0.2.0 if this cycle's vehicle is also a filed epic).

### Diagnostic question

What vehicle would I propose if no precedent existed?

### Reckoning

**Vehicle candidates.**

- **(i) Filed GitHub epic with milestone-tagged child issues.** Inherited shape. Pros: native to GitHub; uses existing issue-craft skill; milestones track progress; milestone displacement chain is already locked. Cons: GitHub-coupled; the methodology's project management becomes coupled to GitHub.
- **(ii) Methodology rewrite (single PR replacing entire methodology).** Pros: clean cutover; no migration state. Cons: PR size infeasible to review; high risk; rejected at Layer E.
- **(iii) Design memo with implementation deferred to a separate governance arc.** This first-return deliverable IS a design memo. On acceptance, a follow-on commission produces the implementation. Pros: separates design acceptance from implementation execution. Cons: the implementation arc still needs a vehicle, so this defers the question rather than answering it.
- **(iv) Research spike that produces design recommendations.** Pros: evidence-gathering before commitment. Cons: the substrate problem is verified, not hypothetical; further evidence-gathering is unnecessary.
- **(v) Phased epic structure.** A top-level architectural epic plus one sub-epic per migration phase. Each sub-epic is independently reviewable and decomposable. Pros: structured rollout; per-phase governance review (acceptance gating); composable with locked governance milestone displacement. Cons: GitHub-issue-graph complexity; coordination across sub-epics requires explicit dependency tracking.

**Cost-benefit.** Vehicle (i) treats the architecture as a single decomposable unit; vehicle (v) treats each phase as independently governance-reviewable. The migration strategy (Shape C, phased) requires per-phase governance acceptance — the failure mode this prevents is the prior cycle's "filed at once with frame-blindness invisible." Per-phase acceptance is structurally easier under vehicle (v) than under vehicle (i): vehicle (i) bundles all child issues under one epic, with no natural acceptance gate between phases; vehicle (v) has a sub-epic per phase, with sub-epic acceptance as the gate.

The diagnostic answer: vehicle (v) phased epic structure. The vehicle follows the migration strategy's shape; per-phase acceptance is the structural correlate of phased migration.

**Vehicle (v), specified.**

- **Top-level architectural epic.** Single epic in `tesserine/groundwork`. Title: working title `epic(architecture): methodology sovereignty per-unit shape constraint` (final wording per issue-craft when the skill is located). Body: this first-return deliverable's content, condensed to issue-craft proportions; full reckoning lives in a docs path (e.g., `docs/architecture/decisions/0002-methodology-sovereignty.md`) referenced from the epic. Milestone: `v0.2.0` per locked governance.
- **Sub-epic per phase.** Seven sub-epics: Phase 1 Substrate, Phase 2 Discipline conformance, Phase 3 Artifact schema review, Phase 4 Mechanic library, Phase 5 Per-protocol workflow contracts, Phase 6 Verification migration, Phase 7 Cleanup. Each sub-epic is filed as a child of the top-level architectural epic. Each sub-epic has its own decomposition into work units (via `decompose` protocol).
- **Decomposition timing.** Each sub-epic's child work units are decomposed when the prior phase's acceptance gate clears. Phase 1's work units are decomposed at architecture-epic acceptance time (now, on first-return acceptance); Phase 2's are decomposed when Phase 1 closes; etc. This avoids the prior cycle's pattern of decomposing all phases up-front (which is itself a frame-inheritance — assuming the later phases will be the same shape regardless of what the earlier phases produce).
- **Milestone displacement.** Top-level architectural epic = `v0.2.0` (per locked governance). Sub-epic milestones distribute across `v0.2.0` and onward:
  - `v0.2.0`: top-level epic + Phase 1 + Phase 2 + Phase 3 (substrate + discipline + schema)
  - `v0.3.0`: Phase 4 (mechanics) + first half of Phase 5 (smaller protocols' contract conversions)
  - `v0.4.0`: second half of Phase 5 (larger protocols including submit) + Phase 6 (verification migration) + Phase 7 (cleanup)
  - `#243` (review epic) defers to `v0.5.0` or later — three milestones past current.
  - `#233` + children (escalation epic) defer to `v0.6.0` or later — four milestones past current.
  - The locked governance specified `v0.3.0` and `v0.4.0` for `#243` and `#233` respectively under a single-epic shape; the multi-phase shape requires deeper deferral. Surface to governance for explicit acceptance.

**Why decomposition timing matters.** The prior cycle decomposed all 26 child issues at filing time. Several were authored against assumptions that earlier phases would later contradict (specific format choices, specific mechanic shapes). The withdrawal exposed this: the entire decomposition was elaborated within a frame that didn't survive review. Decomposing per-phase, after the prior phase's deliverable is in hand, prevents this. Each sub-epic's decomposition reflects what was learned from the prior phase, not assumptions about it.

**Per-phase decomposition discipline (added per first-review).** Per-phase decomposition deferral prevents the all-at-once frame-blindness, but does not on its own prevent the same failure at smaller scale: a later phase decomposed against design-memo assumptions that the current phase's deliverable should have updated but didn't. To prevent this, every phase's acceptance includes an explicit **decomposition-assumption update** for the next phase. The update either (a) names what the current phase's deliverable changed about how the next phase should be decomposed (specific format choices that altered Phase 5 contract authoring; mechanic-library coverage that altered Phase 4's scope; verification-runner architecture that altered Phase 6's test-replacement strategy), or (b) explicitly confirms that nothing changed and the design memo's assumptions for the next phase still hold.

The update is a per-phase deliverable, not an optional addendum. A phase's acceptance gate cannot close without the update. This makes the decomposition discipline visible at every gate: governance can read the update and decide whether the next phase's decomposition needs revision before sub-epic children are filed. The discipline is the structural correlate of "diagnostic question fires recursively" (preamble) — applied to decomposition rather than to format choice, but the same shape: don't carry inheritances across phase boundaries without naming them and asking whether they survive.

**Issue-craft compliance.** All filed issues conform to the issue-craft discipline. The commission's reference to `/mnt/skills/user/issue-craft/SKILL.md` does not resolve in this environment; per governance's confirmation, "if the vehicle settles to filed issues, governance provides the issue-craft discipline directly at that point." This deliverable settles the vehicle to filed issues; on acceptance, governance transmits the issue-craft discipline content (or per-issue review serves as the equivalent). Issues are not filed before issue-craft discipline is available.

**The first-return deliverable's relationship to the vehicle.** This deliverable is the design memo. Its acceptance produces the architectural epic + Phase 1 sub-epic + Phase 1 decomposition. It is not itself filed as an issue; it lives at `/home/pentaxis93/.claude/plans/commission-v2-first-return.md` (this path) until governance accepts; on acceptance, its content informs the epic body and may be moved to a `docs/architecture/decisions/` ADR for persistence.

### Surviving vehicle

**Phased epic structure (vehicle v).** Top-level architectural epic + seven sub-epics (one per phase). Sub-epic decomposition deferred until prior phase's acceptance gate clears. Milestone displacement extends per multi-phase shape; locked governance milestones for `#243` and `#233` defer further than originally specified — explicit governance acceptance required.

The substantive divergence from the prior cycle:
- Prior: single epic, 26 child issues filed at once, milestoned per phase under one milestone (or unmilestoned epic).
- Surviving: top-level epic + seven sub-epics; per-phase decomposition; per-phase acceptance gating; deeper milestone displacement.

The deeper milestone displacement is a real cost (the `#243` review epic and `#233` escalation epic wait longer). The benefit is per-phase acceptance gating, which is the structural correlate of the migration strategy and the prior-cycle-failure-mode-preventing mechanism.

### Inheritances retained at this layer

- **Filed GitHub epic as the vehicle category.** Retained — the methodology lives in `tesserine/groundwork` (per-substrate observation), GitHub is the forge, epics are the native unit.
- **Milestone displacement chain as the sequencing structure.** Retained — locked governance specifies it, and it composes with the multi-phase shape (with adjusted milestone targets).
- **Acceptance-gating as the governance discipline.** Inherited from the cadence the commission specified (research → first return → governance reckoning → revision OR acceptance); extended to per-phase gating in this layer.

### Inheritances rejected at this layer

- **Single epic with all child issues filed at once.** Rejected. Replaced by phased epic structure with per-phase decomposition deferral.
- **In-place conversion as the migration mode.** Rejected at Layer E (inherited from prior cycle, replaced by fresh-authoring under the strong-tier format choice).
- **The prior cycle's specific 6-phase decomposition (A: Substrate, B: Discipline, C: take POC, D: Full execution, E: Architecture docs, F: Integration verification).** Rejected. Replaced by the 7-phase decomposition specified in Layer E.

### What this layer requires from governance

Three items require explicit governance acceptance beyond the locked decisions:

1. **Vehicle change from single-epic to phased-epic structure.** The locked governance assumed single-epic; the multi-phase migration requires multi-epic. Confirmation requested.
2. **Deeper milestone displacement for `#243` and `#233`.** Locked governance specified `v0.3.0` and `v0.4.0` respectively; the multi-phase shape pushes them to `v0.5.0` and `v0.6.0` respectively. Confirmation requested.
3. **Issue-craft discipline transmission timing.** Governance pre-committed: "if the vehicle settles to filed issues, governance provides the issue-craft discipline directly at that point." This deliverable settles vehicle to filed issues. Issue-craft transmission is the next governance act on acceptance.

---

## 7. Inheritances retained

The seven inheritances named in the plan, each given the diagnostic question at its relevant layer. Retentions below are first-principles ratifications, not blind carry-forwards. The comparison that supported retention is named per inheritance.

### Inheritance 7.1 — "Methodology rests on commons P1/ADR-0001 sovereignty"

**Layer where tested:** Layer A.

**Comparison.** Tested against P3 grounding alone, P6 transmission alone, P7 verifiable completion alone, and a composite stack. The substrate observation (WHAT/HOW mixing in submit/PROTOCOL.md) traces cleanly to P1 sovereignty applied at the methodology-document interface; the other principles fire as downstream consequences (P3: temporal, P6: audience, P7: verification), not as alternative roots. The substrate witness for separation-achievable (skills observably WHAT-pure today, 1300 lines with zero tool syntax) confirms the principle is realizable, not aspirational.

**Retained because:** the substrate observation traces cleanly to P1; the principle is realizable in current substrate (skills); commons' fractal claim covers the methodology-document interface natively without ADR-0001 enumeration extension.

**Refinement, not pure retention.** The inheritance survives with a stricter unit boundary (per-document, not per-section) and with fractal-coverage rather than enumeration-extension framing. Both are named divergences from the prior cycle's articulation; the principle itself is retained.

### Inheritance 7.2 — "Skills are cognitive content, separation from HOW is achievable"

**Layer where tested:** Layer A (separation-achievable proof) and Layer B (Disciplines as Category 1).

**Comparison.** The substrate witness is direct: `skills/{contract,debug,orient,reckon,research,resolve}/SKILL.md` collectively contain 1300 lines of cognitive prose with zero shell syntax, zero MCP-tool invocations, zero JSON field paths. This is observably true in commit `e0f32ff` and is structurally distinct from the broken pattern in `protocols/*/PROTOCOL.md`.

**Retained because:** the substrate already implements the principle for skills. No alternative was considered because there is no reason to displace what already works.

### Inheritance 7.3 — "manifest.toml IS the canonical workflow topology"

**Layer where tested:** Layer C (Format C-2 workflow contracts).

**Comparison.** Two alternatives considered: (a) per-protocol typed-interface files, with manifest auto-generated; (b) manifest stays canonical, per-protocol contract files reference it. The commission affirmed the manifest's canonical status as a surviving insight from the prior cycle. Layer C confirmed (b) — manifest as the canonical topology declaration with per-protocol contract files referencing it without duplicating typed interface declarations. The single-source-of-truth principle is honored: the topology lives in one place; per-protocol contracts add the workflow specification (steps, intents, references) without redeclaring the typed interface.

**Retained because:** centralized topology is operationally clean (single file for runa to read; cross-cutting view for humans); the alternative's mechanical duplication risk outweighs the gain from per-protocol distribution at this scale.

### Inheritance 7.4 — "JSON Schema 2020-12 for artifact schemas"

**Layer where tested:** Layer C (Format C-4 artifact schemas).

**Comparison.** Tested against CUE (stronger type system), JTD (simpler), Protobuf (binary). JSON Schema 2020-12 is fit-for-purpose at current scale; expressivity gaps (cross-field invariants) don't fire on artifact shapes the methodology currently uses; tooling ubiquity is the dominant factor. The diagnostic question's answer ("what would I propose if no precedent existed?") is the same as the inheritance: JSON Schema 2020-12.

**Retained because:** first-principles ratification — the inheritance IS the right answer at this scale, not an inherited default.

### Inheritance 7.5 — "TOML for typed declarative content"

**Layer where tested:** Layer C (Format C-2 workflow contracts, Format C-3 mechanics).

**Comparison.** TOML is inherited from `manifest.toml`'s precedent. Tested against YAML (structurally similar, less explicit type system, more whitespace-ambiguity), JSON (no comments, less human-friendly for hand-authoring), and a custom DSL with parser-generator (overinvestment at current scale). TOML is fit-for-purpose: human-readable, supports comments, has standard parsers across languages, expressive enough for the workflow contract and mechanic shapes.

**Retained because:** no alternative provides a cost-benefit improvement at current scale; the inherited choice was correct, not defaulted-to.

### Inheritance 7.6 — "Markdown for cognitive prose"

**Layer where tested:** Layer C (Format C-1 disciplines body, Format C-6 ADR/principle body).

**Comparison.** Tested against typed prose DSL (over-engineered for inherently flexible cognitive content); tested against weak-tier prose-only (under-engineered for catching pattern leaks). Strong-ish tier (Markdown body + JSON Schema-validated frontmatter + body-pattern linter for disciplines) and inspection-only body (for ADRs/principles) survive.

**Retained because:** cognitive prose is inherently prose-shaped; the principle's per-unit shape constraint applies to whether the document is WHAT or HOW (it is WHAT for these categories), not to whether the prose has internal structure beyond what the principle requires.

### Inheritance 7.7 — "Methodology lives in tesserine/groundwork"

**Layer where tested:** Layer E (vehicle proposal); implicit at every layer. *(Revised post-first-review per governance request: the v1 of this section gave operational reasoning rather than running the diagnostic question. This revision applies the diagnostic question with named alternatives.)*

**Diagnostic question (now actually applied).** If no precedent existed — no `tesserine/groundwork` repository, no inherited assumption that methodology has a single home — where would the methodology live? At what locus, in how many places, with what sovereignty boundaries?

**Reckoning.**

The substrate observation (404-line submit) requires an authored, version-controlled, human-readable substrate to address. That requirement constrains some options but not all. Five candidate loci, each tested against the principle, the substrate observation, and ecosystem coherence:

- **Option A — Single dedicated repo (`tesserine/groundwork`).** Methodology principle articulation, categorization, formats, contracts, mechanics, schemas, ADRs, verification all live in one repository. This is the inheritance.
- **Option B — Methodology primitives move to commons.** Principles, ADRs, foundational interface contracts (P1/ADR-0001's enumeration, the WHAT/HOW interface) move to `tesserine/commons`. Methodology-specific content (workflow contracts, mechanics, the conformance runner) stays in groundwork. The principle layer is shared at ecosystem level; the operational layer is methodology-specific.
- **Option C — Methodology specifications consumed directly by runa.** The methodology becomes a runa-internal specification rather than authored artifacts. Schemas, workflow contracts, mechanics live as runa types; the methodology IS the runtime configuration. No separate authored substrate.
- **Option D — Methodology distributed across multiple repositories with different sovereignty.** Principle articulation in commons; protocol contracts in groundwork; mechanics per-forge in forge-specific repositories (`groundwork-github`, `groundwork-forgejo`); discipline-encoding skills in a methodology-shared repo. Cross-repo references resolve at registry-load time.
- **Option E — Runtime-only specification.** No authored substrate. The methodology exists only as runtime behavior; runa's loaded configuration encodes the methodology directly.

**Cost-benefit per option.**

- **Option A.** Pros: established locus; visible sovereignty boundary; methodology iterates independently of commons and runa; clear governance arc; substrate observation (404-line submit) is *in* this repo and is addressed *in* this repo, which is structurally clean. Cons: forge-specific mechanics live in the same repo as forge-neutral principle articulation, which couples them at the repository layer even though they're decoupled at the unit layer (Layer C-3's forge-tagging supports unit-level decoupling).

- **Option B.** Pros: ecosystem-level sharing of foundational principles (other future methodologies could ground in the same principles without re-articulating them); commons becomes the principle-substrate for the ecosystem. Cons: commons currently is principle + ADR + interface-contract scope; pulling methodology primitives expands commons substantially. The expansion is itself an ecosystem-level commission requiring its own governance arc. Methodology iteration becomes coupled to commons iteration (every principle refinement requires commons coordination); current pace of commons evolution and groundwork evolution may not align. The substrate observation (404-line submit) is in groundwork, not commons; addressing it in commons requires an artificial partition between "the principle layer (in commons)" and "the application of the principle (in groundwork)" — a partition the substrate doesn't motivate.

- **Option C.** Pros: methodology becomes runtime-validated by definition; no authored-substrate-vs-runtime drift; methodology IS runa. Cons: tight runtime coupling — methodology cannot iterate independently of runa; per-methodology customization (groundwork vs other future methodologies) requires runa-internal extensibility points; principle articulation has no place to live (runa is a runtime, not a principle substrate); contributor learning has no entry point; verification has no concrete substrate to operate on. Locked governance excludes runa-side changes from this commission's scope, which independently rules this option out for *this* commission, but the substantive case against it stands beyond the scope constraint.

- **Option D.** Pros: forge-specific mechanics genuinely belong in forge-specific repos (clean sovereignty boundary by domain); principle layer separated from execution layer at the repository granularity; multiple-forge support is structurally encouraged. Cons: cross-repo coordination cost (a workflow contract in groundwork referencing a mechanic in `groundwork-github` requires registry-load-time cross-repo resolution); multiple governance arcs needed (each repo has its own arc); release coordination across repos. The benefit (clean forge sovereignty) is achievable at the unit layer via Layer C-3's `forge_tag` field without repository distribution; the repository-distribution adds coordination cost beyond what unit-level forge-tagging requires. Distribution may become motivated later if forge-specific mechanic libraries grow large enough that single-repo cohesion breaks down, but at current scale (10 protocols, ~10–20 forge-specific mechanics) the unit-level decoupling is sufficient.

- **Option E.** Pros: minimal substrate. Cons: no human-readable specification; principle articulation has no home; contributor learning has no entry point; verification has nothing concrete to operate on; substrate observation (404-line submit) cannot be addressed because there's no substrate to redesign. This option is incompatible with the substrate observation that motivates the commission.

**Comparison.**

The substrate observation rules out Option E (no authored substrate to redesign).

The locked governance rules out Option C (runa-side change out of scope), and the substantive case against C (loss of principle articulation, contributor entry point, iteration independence) holds beyond the scope constraint.

Option B introduces ecosystem-level expansion that exceeds what the substrate observation motivates; the partition between "principle in commons" and "application in groundwork" is artificial relative to the unified body of work the commission is addressing. Future ecosystem reckonings may relocate principle layer to commons if multiple methodologies emerge that share the principle stack; at current single-methodology scale, the relocation is not motivated.

Option D's benefits are achievable at the unit layer (Layer C-3's forge-tagging) without repository distribution. Repository distribution adds coordination cost beyond unit-level decoupling. May become motivated later at higher scale; not now.

Option A retains methodology iteration independence, keeps the substrate observation in the same locus as its remediation, keeps principle articulation co-located with operational substrate (admitting cross-section coherence checks at parse time), and admits future relocation of the principle layer if ecosystem-scale motivates it. Operational simplicity is real and aligned with day-one stance: the foundation is movable in the future without being prematurely partitioned now.

**Surviving answer:** Option A — single dedicated repo (`tesserine/groundwork`).

**Retained because:** Option A dominates B, C, D, E for the observed substrate problem at current ecosystem maturity. The reckoning is documented per option above. Future relocation (some Layer A content moving to commons; some forge-specific mechanics moving to forge-specific repos) is admissible at higher scale and is not foreclosed by Option A; Option A is the foundation, not the terminus.

**Note on the v1 framing.** The first version of this section ("no alternative was substantively considered... surfacing without challenge is itself a discipline") was operationally truthful but methodologically thin: it named the inheritance without running the diagnostic question. Governance flagged this as the one place where the cycle's discipline relaxed. The revision above runs the diagnostic question with named alternatives and first-principles cost-benefit. The discipline now operates uniformly across all named inheritances.

### Inheritance 7.8 — "Acid test as a deliberate framing"

**Layer where tested:** Layer D.

**Comparison.** The prior cycle's framing (machine-checkable OR inspection-procedure) is binary; the surviving redesign uses a more granular tier specification (structural-impossibility / after-the-fact detection / inspection-only / deferred). The framing as "the methodology has a deliberately-chosen acid test" is retained; the specific binary is not.

**Retained because:** acid-test discipline is itself sovereign-supportive — the methodology declares what success means at the verification surface, and the principle of declaring is what survives. The specific tier is per-category.

### Inheritance 7.9 — "CI-as-enforcement-surface"

**Layer where tested:** Layer D (verification mechanism integration).

**Comparison.** No alternative substantively considered. CI is the established forge-side enforcement surface; the methodology's conformance runner integrates with CI as a build step. Alternative enforcement surfaces (pre-commit hooks, runtime-only validation) were not explored because CI is the dominant forge convention and the conformance runner runs deterministically (no need for runtime-only).

**Retained because:** existing forge convention; the methodology composes with it without re-inventing.

---

## 8. Inheritances rejected

For each rejected inheritance, the named replacement and the layer where the rejection happened.

### Inheritance 8.1 — "Use what exists, fully — current protocol/skill decomposition is fix-point"

**Layer where rejected:** Layer A (per-document unit boundary stricter than per-level), Layer B (categorization adds artifact schemas, verification, ADRs as their own categories), Layer C (workflow narratives in Markdown rejected; mechanics extracted from protocol bodies).

**Replacement.** Substrate retained where it already conforms (skills, manifest, artifact schemas); replaced where the principle requires different shape (workflow narratives, mechanics, verification specifications, partial categorization).

**Rejection rationale.** The substrate inherits from incremental evolution, not from first-principles design. Some elements correctly satisfy the principle by accident; others don't. Treating "current substrate" monolithically as fix-point or replace-fully is wrong; the right move is per-element evaluation against the principle, with retention where the principle is satisfied and replacement where it isn't.

### Inheritance 8.2 — "Markdown is the methodology format universally"

**Layer where rejected:** Layer C (Format C-2 workflow contracts, Format C-3 mechanics).

**Replacement.** TOML+JSON Schema for workflow contracts and mechanics. Markdown retained for cognitive prose categories (C-1, C-6).

**Rejection rationale.** Markdown is fit-for-purpose for prose-shaped content; it is unfit-for-purpose for typed structured content where shape-violations must be parser-rejected. The inheritance was a category error: applying one format universally regardless of category needs. Per-category format reckoning produces per-category answers; the universal-Markdown inheritance does not survive.

### Inheritance 8.3 — "Markdown + linter as the workflow specification format" (the prior cycle's specific choice)

**Layer where rejected:** Layer C (Format C-2).

**Replacement.** TOML + JSON Schema with structural-impossibility tier.

**Rejection rationale.** The prior cycle chose the weakest format-as-discipline tier (detect-after-expression). Day-one stance argues for the strongest available tier; the cost-benefit reckoning at C-2 confirms structural-impossibility is achievable at acceptable cost (authoring complexity is local to ~10 protocols; benefit is preventing the dominant defect — the 404-line accumulation pattern). The prior cycle's specific choice is the named instance of inheritance 8.2's category error; rejected for the same reason but at a more specific layer.

### Inheritance 8.4 — "Single epic with all child issues filed at once is the vehicle"

**Layer where rejected:** Layer E.

**Replacement.** Phased-epic structure: top-level architectural epic + seven sub-epics (one per migration phase) + per-phase decomposition deferral.

**Rejection rationale.** The prior cycle's all-at-once decomposition assumed later phases would be the same shape regardless of earlier phases' deliverables; this is a frame-inheritance (assuming the design is right before the first phase's substrate is in hand). Per-phase decomposition deferral lets each sub-epic's children reflect what was learned from the prior phase, not assumptions about it.

### Inheritance 8.5 — "In-place conversion is the migration shape"

**Layer where rejected:** Layer E.

**Replacement.** Fresh-authoring per protocol. Existing `protocols/*/PROTOCOL.md` files are replaced by new `protocols/X/contract.toml` (or similar) files; not edited.

**Rejection rationale.** Layer C's strong-tier format choice (TOML+schema for workflow contracts) makes existing Markdown files unparseable as new-format contracts; they cannot be edited into conformance. The format change forces the migration shape: fresh-authoring is the only viable option under structural-impossibility. The inheritance survived in the prior cycle only because the prior cycle's format choice was weak-tier; under the stronger format, the inheritance does not survive.

### Inheritance 8.6 — "Substring-presence is the test surface"

**Layer where rejected:** Layer D (verification mechanism); Layer C-5 (verification specifications format).

**Replacement.** Per-category conformance runner that dispatches to category-specific parsers and validators. Behavior verification deferred to integration layer (user projects).

**Rejection rationale.** Substring-presence assertions on prose were the residual when prose was the methodology format and behavior verification was unavailable. The format change (Layer C-2) makes the prose itself disappear; substring tests have nothing left to assert against. The replacement (conformance runner) verifies what the formats now structurally require: shape conformance, cross-reference resolution, registry consistency.

### Inheritance 8.7 — "Topology is its own content category"

**Layer where rejected:** Layer B.

**Replacement.** Topology is workflow contracts' interface-typed sub-shape (as expressed in `manifest.toml`); not a category co-equal with workflow contracts. The graph emerges from the typed interfaces; it is a view, not an authored category.

**Rejection rationale.** The single-source-of-truth principle (commons-affirmed insight) implies topology should not be authored separately from per-protocol typed interfaces. Whether per-protocol typed interfaces centralize (current `manifest.toml`) or distribute is a representation question; the topology graph is a derived view either way. Promoting topology to a category co-equal with workflow contracts is a category error that the prior cycle inherited from substrate convenience.

### Inheritance 8.8 — "All-units monolithic linter mode" (the prior cycle's verification execution surface)

**Layer where rejected:** Layer D.

**Replacement.** Per-unit verification dispatched by the conformance runner. Each unit gets type-specific checks; the runner aggregates per-unit pass/fail.

**Rejection rationale.** A monolithic "all-units mode" runs a uniform check across all units regardless of category. The categorization at Layer B requires category-specific checks (workflow contracts get contract validation; mechanics get mechanic validation; disciplines get body-pattern linting). Per-unit dispatch is the structural correlate of per-category formats; the monolithic mode is a mismatch.

---

## 9. New substrate-first findings

Findings discovered during the reckoning that were not in the commission body and that future governance reckoning should track.

### Finding 9.1 — ADR-0007 "Traces to" numbering inconsistency

**Substrate observation.** Commons `adr/0007-day-one-stance.md` line "Traces to" cites "Principle 2 (Everything Earns Its Place), Principle 4 (Compound Improvement)" as the principles ADR-0007 traces to. The current `tesserine/commons/PRINCIPLES.md` numbers the principles: 1 = Sovereignty, 2 = Sequence, 3 = Grounding, 4 = Obligation to Dissent, 5 = Recursive Improvement, 6 = Transmission, 7 = Verifiable Completion. ADR-0007's "Traces to" line uses a different numbering (where 2 = "Everything Earns Its Place" and 4 = "Compound Improvement").

**Interpretation.** Either (a) PRINCIPLES.md was renumbered after ADR-0007 was authored and the ADR's "Traces to" line was not updated, or (b) ADR-0007 was authored against an earlier principle ordering that was never reconciled, or (c) the authored ADRs use principle names rather than numbers internally and the numbers are an editorial slip.

**Disposition.** Out of scope for this commission (groundwork-only). Surface as ecosystem-level finding for `tesserine/commons` to track. Governance has confirmed: "Real ecosystem-level defect; surface in item 9 of first-return as proposed. Governance will track this as a separate commons commission, independent of the groundwork architectural reckoning. Not a blocker for this work."

### Finding 9.2 — Issue-craft skill path absent in current environment

**Substrate observation.** The commission references `/mnt/skills/user/issue-craft/SKILL.md` as the issue-craft discipline source. This path does not exist in the agent's current environment. Searching `/home/pentaxis93/.claude` and `/mnt/skills` (not present) yielded plugin marketplace skills but no `issue-craft` skill at the referenced location.

**Interpretation.** The path was likely a leak from governance's environment into the commission body — a path that exists for governance but not for the agent producing this deliverable.

**Disposition.** Governance has confirmed: "if the vehicle settles to filed issues, governance provides the issue-craft discipline directly at that point — either by transmitting the discipline content, or by per-issue governance review. Defer until vehicle settles, as proposed." This deliverable settles vehicle to filed issues (Layer 6). Issue-craft transmission is the next governance act on acceptance.

### Finding 9.3 — Skills are observably WHAT-pure today

**Substrate observation.** `skills/{contract,debug,orient,reckon,research,resolve}/SKILL.md` collectively contain 1300 lines of cognitive prose. Direct reading of `debug/SKILL.md` (307 lines) and `orient/SKILL.md` (171 lines) confirms zero shell syntax, zero MCP-tool invocations expressed as code, zero JSON field paths, zero git refspec patterns. The discipline-encoding skills are uniformly cognitive content addressed to agent reasoning.

**Significance.** This is the separation-achievable proof at the substrate level: the principle the redesign rests on is realizable because half the substrate already implements it. The proof is not theoretical or aspirational; it is observable. Phase 2 of the migration (discipline conformance) is therefore a lift-and-fit operation — adding frontmatter schema validation and body-pattern linter — not a content rewrite. This finding lowers Phase 2's scope estimate and informs the migration timeline.

### Finding 9.4 — Substring-test count as the sharpest substrate witness for #255

**Substrate observation.** `tests/test_submit_protocol.py` contains 28 `self.assertIn(<phrase>, protocol)` assertions across 25 test methods (some methods have multiple assertions). Direct reading confirms the pattern: every test verifies that specific phrases appear in submit's prose. The phrases include `"open PR exists and deliverable local work exists"`, `"git fetch <base-repo-remote> pull/<number>/head"`, `"PR head SHA"`, `"GitHub-shaped commitment"`, `"first-push semantics"`. The tests catch phrase-deletion regressions but verify nothing about whether the encoded behavior is correct.

**Significance.** This is the symptom #255 questions, in concrete count. The substrate forces this test pattern: the protocol's behavior is entangled in mixed-shape prose; behavior-level verification is unavailable; substring-presence is the residual. The redesign at Layer C-2 + Layer D dissolves this by replacing the prose with parser-checkable structure; the conformance runner verifies what the new format requires (shape, references, registry consistency); the 28 assertions are removed in Phase 6.

**For #255 specifically:** the question "should documentation-only protocols have automated tests" is answered by Layer D: yes, conformance verification (parser-level) is the right verification surface; behavior verification at the methodology layer is deferred to integration time in user projects.

### Finding 9.5 — The composition graph between categories is a DAG

**Substrate observation.** From Layer B's composition graph: Disciplines, Workflow contracts, Mechanics, Artifact schemas, ADRs/Principles, Verification specifications relate through references that form a DAG. No cycles. Composition is consistent: workflow contracts reference disciplines, mechanics, schemas; mechanics reference schemas only; disciplines reference each other and workflow contracts; verification references everything except principles; ADRs reference principles; principles reference nothing.

**Significance.** The methodology has its own internal topology that mirrors the protocol topology in `manifest.toml`. This is itself a fractal: the methodology is composed of categories whose composition graph is structurally similar to the protocol composition graph the methodology produces. Future governance reckonings may consider whether the methodology's category-graph deserves explicit declaration (a manifest of the methodology's *self-structure*, not just its protocol-structure).

### Finding 9.6 — Two-tier verification deployment manifests P1 fractally

*(Revised post-first-review: v1 of this finding claimed three-tier fractality, with activation-time as the third tier. The activation-time tier was dropped because it implied a runa-side change out of locked-governance scope, and its load-bearing verification value did not stand under substrate-first reading. The fractal observation reduces to two tiers; both are real and both manifest the principle structurally.)*

**Substrate observation.** Layer D specifies two deployment points for the conformance runner: authoring time, PR time. Each corresponds to a distinct sovereignty interface:
- Authoring time: contributor ↔ methodology source (operator↔agent in commons enumeration)
- PR time: project ↔ shared methodology (producer↔validator in commons enumeration)

**Significance.** The verification deployment is a P1 fractal application at two interfaces. The methodology's verification surfaces operate at two commons-enumerated interfaces (operator↔agent at authoring; producer↔validator at PR). This is structural alignment between the verification deployment and the principle the methodology rests on — independent emergence of the same fractal pattern at a different scale.

**The dropped third tier as a finding-of-its-own.** The original v1 articulation claimed a third interface (runtime↔methodology at activation time) where runa loads methodology and runs verification. This was elegant but not load-bearing, and it imposed runa-side coupling. The lesson: fractal observations are tempting (the pattern matches the principle), but a tier that doesn't carry verification value is decorative, not structural. Decorative fractality risks elaborating an aesthetic match into an operational commitment that doesn't serve the foundation. Day-one stance applies: preserve the foundation; drop decorative elaboration. The two-tier deployment is what the methodology's sovereignty surface actually supports.

### Finding 9.7 — Workflow narrative WHAT-content is much smaller than current PROTOCOL.md sizes suggest

**Substrate observation.** Submit's `PROTOCOL.md` is 404 lines. Direct examination of the WHAT-content (per-step intents, declared outcomes, references to mechanics) suggests the WHAT-portion is approximately 100 lines if extracted cleanly. The remaining ~300 lines are HOW (specific gh JSON paths, git refspec patterns, shell heredoc syntax, MCP-tool invocation syntax) plus prose explaining why the HOW is shaped the way it is.

**Significance.** The format change at Layer C-2 produces dramatic line-count reduction at the workflow contract layer. Submit's workflow contract is approximately 100 lines as a TOML document with 8 typed steps; the extracted mechanics live in their own ~10 mechanic units totaling perhaps 250 lines (still less than current substrate because mechanics are deduplicated across protocols). Total methodology line-count after migration is likely substantially smaller than current substrate, despite the new architecture's higher structural density. This finding informs scope estimates for Phase 5 (per-protocol contracts) and Phase 4 (mechanic library).

### Finding 9.8 — Phase 5 protocol ordering is itself a method-design finding

**Substrate observation.** Layer E specified the recommended Phase 5 ordering: `verify` → `take` → `plan` → `specify` → `document` → `survey` → `decompose` → `land` → `implement` → `submit`. This orders by current PROTOCOL.md size (smallest to largest), with submit last.

**Significance.** Authoring the new format on the smallest protocol first allows the format to be exercised, the authoring guide (Layer C deliverable) to be refined, and the cross-reference resolver to be validated before the broken witness (submit) is converted. The ordering is itself a method-design choice: the simplest case validates the format; the most complex case stress-tests it. This is a per-phase governance acceptance opportunity — after the first protocol's contract is written, a governance check on whether the format is fit-for-purpose (does authoring obviousness hold? does substrate longevity hold for this protocol?) can gate the next protocol's conversion.

### Finding 9.9 — `research-record` is the sole skill-produced artifact, and the bridge is `may_produce`

**Substrate observation.** From `manifest.toml`: four protocols (`survey`, `decompose`, `specify`, `plan`) declare `may_produce = ["research-record"]`. The `research` skill, not a protocol, is the artifact's actual producer; the skill produces `research-record` as a side-channel artifact during protocol execution. The skill/protocol artifact-production semantics are mediated by `may_produce`.

**Significance.** This was a finding the prior cycle surfaced and the commission flagged; the new categorization at Layer B carries it through. The `research-record` is the one artifact-producing skill — a structural exception to the otherwise-clean "protocols produce artifacts; skills are cognitive disciplines" pattern. The exception survives because research is genuinely a side-channel: research output is artifact-shaped (sources, findings, topic) but the act of producing it is cognitive (a discipline). The new architecture preserves this by allowing disciplines to reference artifact schemas (Layer B Disciplines composition rules) and by allowing workflow contracts to declare `may_produce` artifacts that disciplines (rather than protocols) actually produce. This is not a defect; it is the architectural acknowledgment that research is a hybrid cognitive-artifactual activity.

### Finding 9.10 — README's beliefs section names sovereignty in inter-protocol form

**Substrate observation.** `README.md`'s "What Groundwork Believes" section (around line 67 area) currently names sovereignty in inter-protocol terms: "every handoff passes outcomes — what must be true — never implementation steps." This is the inter-protocol form of P1: each protocol-to-protocol handoff is a sovereignty boundary.

**Significance.** The new architecture extends this principle to intra-unit form (within a single methodology document). Phase 7 (Cleanup) updates the README beliefs section to articulate sovereignty at both the inter-protocol and intra-unit levels. This is a small but visible update that signals the principle's extended reach. The prior cycle named this finding; it carries through unchanged.

### Finding 9.11 — No protocol-authoring guide currently exists

**Substrate observation.** `docs/authoring/` contains only `skills.md` (138 lines, landed `8ce3ea8`). No protocol-authoring guide. No mechanic-authoring guide (mechanics don't yet exist as a category). No ADR-authoring guide.

**Significance.** Phase 7 (Cleanup) produces per-category authoring guides: workflow contract authoring guide, mechanic authoring guide, ADR authoring guide (or extension to existing patterns). The current `skills.md` extends to cover the discipline category's frontmatter schema and body-pattern conventions. The authoring guides are part of "authoring obviousness" (Layer D acid test) — without them, parser errors are unactionable for new contributors. This finding informs Phase 7 scope and motivates per-category guide authoring at the same milestone as the format itself.

### Finding 9.12 — Recursive sub-structural diagnostic discipline

*(Surfaced from this cycle's first-revision round.)*

**Substrate observation.** The v1 of this deliverable applied the diagnostic question at every layer (A through E). It did *not* apply the diagnostic question at sub-structural layers within layer-level choices. Specifically: at Layer C-2 the format-choice question ("TOML+JSON Schema vs alternatives") was reckoned, and the surviving format admitted an internal structure (the workflow contract's internal shape: `steps` as ordered array, with per-step record fields). The internal structure was inherited from "ordered array of steps" as the structural default; the diagnostic question was not asked of that sub-structural choice.

Governance flagged this in the first review. The substrate observation: workflows in current methodology are *not* flat ordered sequences. Submit, land, implement, survey, decompose all have branching, convergence, loops, or terminal-state distinctions that the flat-array shape cannot encode as first-class structure. The flat-array inheritance would have forced flow-mixing into step intent prose or untyped failure_modes responses — the same accumulation pathway the redesign exists to close, reopened at the sub-structural layer.

**Significance.** This is a class-level methodological finding: **the diagnostic question must apply recursively at sub-structural layers within any choice that survives at the layer above**. Inheritance can ride into a layer through:
- The top-level format choice (caught by the layer-level diagnostic question).
- The internal structure that format admits (caught only by sub-structural diagnostic questioning — missed in v1).
- The further sub-structural shapes within the internal structure (would require recursive application until atomic primitives).

The discipline updates from "diagnostic at every layer" to "**diagnostic at every layer AND every sub-structural layer within every layer's surviving answer, recursively until atomic primitives**." The recursion bottoms out when:
- The choice is atomic (no further internal structure to reckon on).
- The alternatives are clearly equivalent in cost-benefit (e.g., field-name choices like `name` vs `id` don't materially differ).
- The choice has been settled at a higher principle layer that the sub-structural choice inherits consistently (e.g., a sub-structural shape consistent with the principle's WHAT/HOW separation does not need re-reckoning at the sub-structural layer for principle compliance — but other dimensions of the choice still get the diagnostic question).

**Practical procedure.** Before committing to a chosen format at any layer, identify all substantive structural choices the format implies, and apply the diagnostic question to each. Treat each "natural" sub-structural shape (the shape that would be inherited as a default) as an explicit candidate among alternatives, not as the format's pre-determined shape.

**Carry-forward.** This discipline updates the cycle's running articulation. The preamble of this deliverable (revised) reflects the recursive sub-structural application as the cycle's discipline. Future commissions reckoning on methodology, ecosystem, or any layered design should apply the discipline from the start.

**Failure-mode pattern named.** The failure mode this discipline catches: **layer-level diagnostic question applied; sub-structural inheritance ridden in invisibly through the chosen format's natural internal shape**. The v1 of this deliverable had this failure at C-2 (workflow contract internal shape). The prior commission cycle had a different version of the same class of failure: layer-level frame inheritance ridden in invisibly through the prior cycle's "use what exists" heuristic. Both are the same class — inheritance riding through unflagged at a layer the diagnostic question didn't reach. The discipline update extends the diagnostic question's reach into sub-structural layers; the principle is the same; only the recursion depth changes.

---

## At a glance — surviving choices

For governance navigation. The reckoning lives in the body sections; this index restates the surviving answers in compact form.

**Principle.** Methodology Sovereignty: each methodology unit specifies content of a single shape (WHAT or HOW), not both. Per-document unit boundary. P1/ADR-0001 fractal at the methodology-document interface; covered by commons' fractal claim, no enumeration extension required.

**Categories.** Six. Two top-level shapes: WHAT (Disciplines, Workflow contracts, Artifact schemas, ADRs/Principles) and HOW (Mechanics, Verification specifications). Topology is workflow contracts' interface-typed sub-shape (derived view, not authored category).

**Formats.** Five at structural-impossibility tier; two with named tradeoffs:
- C-1 Disciplines: Markdown + JSON Schema frontmatter + body-pattern linter (mixed tier).
- C-2 Workflow contracts: TOML + JSON Schema (strong tier).
- C-3 Mechanics: TOML + JSON Schema, forge-tagged (strong tier).
- C-4 Artifact schemas: JSON Schema 2020-12 (strong tier; first-principles ratification of inheritance).
- C-5 Verification: parsers + linters as the conformance surface (strong tier for shape); behavior verification deferred to integration layer.
- C-6 ADRs/principles: Markdown + JSON Schema frontmatter + inspection rubric for body (mixed tier).

**Verification.** Per-category dispatched by a conformance runner. Deployed at two points: authoring time (contributor feedback) and PR time (CI gating). Two-tier deployment is a P1 fractal at two interfaces (operator↔agent at authoring; producer↔validator at PR). The v1 articulation included a third "activation time" tier that implied runa-side coupling; it was dropped in revision 2 because its load-bearing verification value did not stand under substrate-first reading and the runa-side change was out of locked-governance scope.

**Acid tests.** Three primary: substrate longevity (the redesign's central success condition); authoring obviousness (operational); composition cleanliness (architectural). Graceful degradation deferred to migration layer. The prior cycle's binary acid test (machine-checkable OR inspection-procedure) is subsumed under the per-category tier specification.

**Migration.** Phased category rollout with fresh-authoring per protocol (in-place conversion infeasible under strong-tier C-2 format). Seven phases: Substrate → Discipline conformance → Artifact schema review → Mechanic library → Per-protocol contracts → Verification migration → Cleanup. Phase 5's per-protocol PRs ordered smallest-first with submit last.

**Vehicle.** Phased-epic structure: top-level architectural epic + seven sub-epics (one per phase). Per-phase decomposition deferred until prior phase clears acceptance. Top-level epic = `v0.2.0` per locked governance; deeper milestone displacement for `#243` and `#233` than originally specified.

---

## Cycle history

This ADR was produced by a substrate-aware reckoning cycle in May 2026, following the withdrawal of the prior architectural epic (`tesserine/groundwork#256`) on 2026-05-02 for frame-inheritance. The cycle's first return surfaced nine architectural answers; governance's first-review accepted them in substance and requested four revisions: (1) actually apply the diagnostic question at Inheritance 7.7; (2) drop the activation-time verification tier (out-of-scope runa coupling); (3) add a per-phase decomposition-assumption-update discipline at Section 6; (4) re-reckon flow-control encoding at the sub-structural layer within Format C-2. The four revisions landed; the second-return was accepted.

The cycle's deepest methodological output was the recursive sub-structural diagnostic discipline (Finding 9.12), which emerged from the first-review's revision 4. Governance accepted this discipline as the cycle's running articulation; future commissions reckoning on methodology or ecosystem-level designs apply it from the start.

The cycle proceeded with a mutual discipline: governance probed frame-soundness; the produced work surfaced frame-inheritance by name. The discipline was visible at every revision boundary — the Inheritance 7.7 closing acknowledgment ("v1 was operationally truthful but methodologically thin") is a worked example of the discipline naming its own relaxation rather than glossing it.

---

## Consequences

This ADR's acceptance produces a phased-epic implementation arc:

1. **Top-level architectural epic** in `tesserine/groundwork`, milestone `v0.2.0`. The epic body summarizes this design and links to this ADR. Acceptance criteria reference the seven migration phases and per-phase governance gates, including the per-phase decomposition-assumption-update discipline.
2. **Phase 1 sub-epic** filed concurrently, decomposed into specific work units: define schemas (per category); build parsers; build conformance runner; CI integration. Phase 1's work units are decomposable on epic filing because Phase 1's deliverables are well-specified by Section 3 (Layer C formats) and Section 4 (Layer D verification).
3. **Phases 2–7 filed as sub-epic placeholders.** Each carries phase-level acceptance criteria but no child work units yet; per-phase decomposition is deferred until the prior phase's acceptance gate clears (the discipline added in revision 3).
4. **Existing milestone deferrals adjusted:** `#243` (review epic) to `v0.5.0` or unmilestoned; `#233` (escalation epic) and children `#234`–`#238` to `v0.6.0` or unmilestoned. The deeper deferral is the cost of the multi-phase architectural arc; governance accepted it in first-review.
5. **Issue-craft per-issue at review time:** each filed child issue receives governance review at filing time, with issue-craft discipline applied during that review rather than transmitted in advance (per first-review disposition).
6. **Implementation start gate:** Phase 1 implementation begins after `v0.1.2` (#220) closes per locked governance.
7. **Per-phase governance reckoning rhythm:** each phase's deliverable is reviewed by governance; the per-phase acceptance includes a decomposition-assumption-update for the next phase. The discipline operates structurally at every gate.

The ADR's body — Sections 1 through 9 plus the at-a-glance summary — is the design substrate for the implementation arc. The per-phase work units trace their acceptance criteria back to this ADR's surviving answers.

### Good

- The accumulation pathway that produced submit's 404-line `PROTOCOL.md` becomes structurally impossible under the C-2 directed-graph workflow contract format.
- Methodology Sovereignty operates as P1 fractal at the methodology-document interface, with corollary derivations from P3, P6, P7. The principle is realizable in current substrate (skills are already WHAT-pure today) and structurally enforced in the new substrate.
- The recursive sub-structural diagnostic discipline (Finding 9.12) catches a class of failure (sub-structural inheritance riding through invisibly) that the layer-only diagnostic question missed in the first revision round.
- Per-phase decomposition deferral + per-phase decomposition-assumption-update prevents the prior cycle's failure mode (decomposing all child issues at once with frame-inheritance invisible) and the same failure at smaller scale (decomposing a later phase against design-memo assumptions the current phase's deliverable should have updated).
- Forge-neutrality is structurally enabled at the unit layer (Layer C-3 mechanic forge-tagging) without forcing repository distribution; future ecosystem reckonings may relocate forge-specific mechanics to forge-specific repos if scale motivates it.

### Neutral

- Existing protocols' `PROTOCOL.md` files cannot be in-place converted; they must be re-authored as TOML workflow contracts. The migration is fresh-authoring per protocol (Section 5).
- Behavior verification at the methodology layer is deferred to integration in user projects (Section 3 Format C-5). The methodology specifies conformance verification (parser-level) only; the deferral is named explicitly.

### Bad

- Authoring complexity at the workflow contract layer (Section 3 C-2) is higher than free-prose authoring. Contributors author a directed graph with typed conditions instead of writing prose narrative. Authoring guides (Phase 7 deliverable) carry the authoring-obviousness load.
- Deeper milestone displacement for `#243` and `#233`+children pushes their delivery further than the locked-governance baseline. The displacement is the cost of doing the architectural redesign correctly at this scale; the trade is named.
- Activation-time conformance verification (the dropped third tier from revision 2) is a real verification surface the methodology now does not cover. If methodology drift between PR-merge and runtime activation becomes a real defect, a future ADR may revisit the question; the current ADR explicitly defers.

---

*End of ADR-0002. The reckoning lives in the body sections; the preamble and at-a-glance index are navigation aids. The discipline is mutual: governance probes frame-soundness; the produced work surfaces frame-inheritance by name. Future reckonings on methodology, ecosystem, or any layered design apply the diagnostic question at every layer and every sub-structural layer (Finding 9.12) as the cycle's running articulation.*
