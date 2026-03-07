---
name: ground
description: >-
  First-principles cognitive discipline for all generative work. Use when
  creating specs, architectures, processes, solutions, methodologies, problem
  framings — any task requiring original design. Use for migration, upgrade,
  and technology selection decisions. Use before defaulting to the existing
  approach. Establishes what the work must enable before decomposing to
  verified constraints, then builds from what is actually true.
metadata:
  version: "2.3.0"
  updated: "2026-03-07"
  origin: >-
    Successor to clean-slate. The predecessor caught migration failures
    (fabricated switching costs, compatibility layering) but missed the
    broader pattern: agents default to inherited thinking in ALL generative
    work. The same instinct that fabricates switching costs also accepts
    problem statements uncritically, copies categories from adjacent systems,
    and preserves complexity out of fear.
  replaces: "clean-slate"
---

# Ground

*What must this enable? What is actually required? Build from that.*

## The Move

Six steps. Always the same.

0. **Orient.** Before touching anything, establish the need. What must this enable? Who does it serve? What do they need to accomplish? These answers are the actual constraints. Everything else — existing code, existing patterns, existing implementations — is evidence about one attempt to meet those constraints, not the constraints themselves.

1. **Decompose.** Strip the problem to its actual constraints. What must be true? What is assumed? What was inherited from the prompt, the existing system, or the adjacent example? The Assumed-Constraint Patterns below are the common forms these inherited assumptions take.

2. **Verify.** For each constraint: is this real (physics, contract, measured need) or inherited (convention, precedent, comfort)? If you cannot point to evidence, it is assumed.

3. **Reconstruct.** Build from verified constraints only. What emerges may resemble existing solutions or may not. Both are fine. What matters is that every element earns its place.

4. **Compare.** Does the grounded design match what exists? If yes, the existing approach is validated. If no, weigh real migration cost against carrying cost.

5. **Default to the grounded design.** Inherited assumptions compound. When grounded and existing designs diverge, the grounded design wins unless migration cost is concrete and measured.

**Why Orient comes first.** 80% of solving a problem is defining the problem. Orient exists because without it, decomposition has no anchor — you will decompose whatever is in front of you, which is usually the existing system. Orient points decomposition at the right target: the need, not the implementation.

**The descriptive/normative distinction.** There are two kinds of truth relevant to design work. *Descriptive truth* is what currently exists — the code, the configuration, the running system. *Normative truth* is what's actually needed — the requirements, the capabilities, the outcomes that matter. For design work, you ground in normative truth. Descriptive truth is evidence about one implementation, useful for gap analysis after the design exists, but never the starting point. Confusing these — treating what the system currently does as the definition of what it should do — is the most common grounding failure.

**Grounding vs. analogy.** Analogy copies solutions and their embedded assumptions. Grounding derives solutions from constraints and discovers which assumptions were load-bearing.

**Why this matters more for agents:** LLMs exaggerate human cognitive biases — anchoring, confirmation, primacy, status quo. Research shows effect sizes "unusually large," behaving as "caricatures of human cognitive behavior." First information in context disproportionately shapes all subsequent reasoning. Without Orient, the first information is typically the existing system, and everything flows from there. Orient ensures the first information is the need.

---

## Assumed-Constraint Patterns

These fire on all generative work. When you notice any of these, stop and ground.

### 1. Problem-as-Given

Accepting the problem statement without questioning scope, framing, or premises.

**Recognition:** You are optimizing within the frame you were handed. You have not asked whether the frame is correct.
**Corrective:** "What problem are we actually solving? Is this the right question, or the question we were given?"

### 2. Description as Design

Treating what the current system does as the definition of what it should do — documenting what is instead of designing what's needed.

**Recognition:** Your "requirements" are descriptions of existing behavior. Every claim traces to existing code or configuration; none trace to user needs. You read the implementation and organized your findings instead of asking what it must enable.
**Corrective:** "If this implementation did not exist, what would users need?" Start from the need. Use the existing implementation for gap analysis only after the design exists.

### 3. Borrowed Structure

Importing categories, patterns, or architecture from adjacent systems without verifying fit.

**Recognition:** Your design's structure mirrors an adjacent system, a familiar pattern, or the categories in the request. You did not derive them from requirements — you inherited them. The analogy feels natural, perhaps too natural.
**Corrective:** "What structure would emerge from the requirements alone?" Design fresh. Compare with the inherited structure afterward. Identify where the analogy breaks.

### 4. Precedent as Constraint

Treating past decisions, existing implementations, or prior investment as requirements.

**Recognition:** "We did it this way before," "the existing system does X," or "we already invested in Y" appears in your reasoning as constraint rather than data point. The existing solution is your design starting point rather than a comparison target.
**Corrective:** Precedent is evidence, not constraint. Past investment is irrelevant to future value. "If this precedent did not exist, what would we build?"

### 5. Complexity Preservation

Maintaining complexity because removing it seems risky.

**Recognition:** You are preserving structure not because it serves current requirements, but because removing it might break something you do not understand — or adding structure for requirements that do not yet exist.
**Corrective:** "What is the simplest design that meets requirements?" If simpler than what exists, the complexity needs justification or removal.

### 6. Audience Assumption

Designing for the requester, yourself, or an imagined user rather than verified actual users.

**Recognition:** You have not identified who this serves. You are designing for the voice in the prompt.
**Corrective:** "Who actually uses this? What do they actually need?" Ground the audience before grounding the design.

### 7. Abstraction Gravity

Defaulting to the abstraction level of adjacent or existing systems.

**Recognition:** Your design operates at the same abstraction level as the system it replaces or resembles. You adopted this level implicitly — you cannot articulate why it is correct for this problem. The decision was inherited, not made.
**Corrective:** "What abstraction level does this problem actually require?" Derive the level from the problem's actual structure. The existing system's level is a data point, not a default.

### 8. Local Coherence

A detail follows a valid pattern and sounds right in isolation, but contradicts the purpose established in Orient.

**Recognition:** Your output would be correct in a generic context — it follows established engineering conventions, uses appropriate technical language, and reads as a reasonable decision. But you have not checked it against the specific purpose of this work. The fluency of the output masks the contradiction. Common vectors: failure-mode defaults that negate the feature ("if the safety check fails, skip it"), error paths that undo the work, fallbacks to the behavior the feature exists to replace.
**Corrective:** Restate the purpose from Orient. Re-evaluate the detail: "If this fires, does the feature still accomplish its goal?" If no, the detail defeats the feature.

### Preservation Variants

These fire specifically when existing state creates gravitational pull — migration, upgrades, technology selection. Migration cost trends toward zero as tooling improves; carrying cost compounds. Evaluate both sides.

**Fabricated Costs** — Presenting migration costs that do not exist. *Recognition:* You claimed a migration cost without verifying it — the cost appeared in your reasoning before it appeared in evidence. *Corrective:* Before claiming any cost, verify it. Run the command. Read the docs.

**Compatibility Layering** — "Support both old and new." *Recognition:* Your design maintains two parallel systems or translation layers. *Corrective:* Two systems is almost always worse than one clean migration. Pick one. Migrate fully.

**Risk Asymmetry** — Treating change as risky and stasis as safe. *Recognition:* You evaluated the risks of changing but not the risks of staying. *Corrective:* Stasis accumulates hidden debt. Evaluate both risks explicitly.

**"It Works" as Sufficient** — Working is the minimum bar. *Recognition:* Your defense of the current approach is that it functions, with no comparison to alternatives. *Corrective:* "It works and there is no materially better approach today" is the actual defense.

---

## When to Ground

**The trigger:** You are about to create something. Ask: "Have I established what this must enable, or am I starting from what already exists?"

## When NOT to Ground

- **Mid-execution.** Finish the current step, then reassess. Grounding fires at decision points.
- **Verified external constraints.** Users at scale, contracts, and regulations are ground truth — they survive decomposition.
- **Diminishing returns.** If grounding produces the same design as the inherited approach, the approach was correct. Grounding is verification, not contrarianism.

---

## Corruption Modes

**Skipped Orient.** Jumping straight to decomposition without establishing what this must enable. The need shapes everything. If you decomposed without it, your decomposition targeted the wrong thing.

**Performative grounding.** Going through the motions without questioning. "I considered the requirements and they match what was given." If decomposition always confirms the inherited frame, you are rationalizing, not decomposing.

**Implementation survey as design.** Thorough research of the existing system presented as a design document. The research is valuable — for gap analysis. But organizing implementation facts is not designing. If your output would be equally true as a README for the current system, you have not designed anything.

**Infinite decomposition.** Using grounding to delay decisions. Decomposition serves reconstruction. If you are decomposing without rebuilding, you have stalled.

**Rejection as reflex.** Dismissing all inherited structure because it is inherited. Some precedents are correct. Grounding is verification, not contrarianism.

---

*The default is to float — in inherited frames, borrowed categories, precedent as constraint, descriptions of what is. Orient returns you to what is needed. Grounding returns you to what is true. Build from there.*
