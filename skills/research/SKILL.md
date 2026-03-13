---
name: research
description: Systematic multi-source research with citations and synthesis using 6-phase workflow and Tavily
license: MIT
metadata:
  version: "1.3.0"
  source: internal
  updated: "2026-03-07"
  workflow: 6-phase (Clarify → Decompose → Gather → Evaluate → Resolve → Synthesize)
requires: []
accepts: []
produces: ["research-record"]
may_produce: []
trigger:
  on_signal: "evidence-needed"
---

# Research Skill

This file is the **how** — the deep methodology for structured, multi-source research. Any sk-compatible agent can load this skill to execute the workflow below.

## Core Principles

### 1. Start Simple, Add Complexity When Needed
Begin with focused searches before expanding scope. Complex multi-agent research is only warranted when simpler approaches fail to produce adequate results.

### 2. Context Engineering is Critical
The quality of research depends on providing the right information, in the right format, at the right time. Structure queries precisely and validate sources systematically.

### 3. Three-Source Rule
Never trust a single source. Cross-reference with at least two independent sources before treating information as reliable.

### 4. Version Everything
Most source conflicts dissolve when versions/dates are explicit. Always anchor findings to specific versions and timestamps.

### 5. Empirical Verification Over Authority
When stakes are high, test claims directly rather than trusting even authoritative sources. Code behavior beats documentation.

---

## Research Workflow

### Phase 1: Clarification

Before researching, clarify the query:

1. **Identify the core question**: What specifically needs to be answered?
2. **Determine scope**: Breadth vs depth? Historical context needed?
3. **Specify output format**: Report, comparison table, decision recommendation?
4. **Note constraints**: Time period, specific technologies, geographic scope?

If the query is vague, ask clarifying questions before proceeding. Do not research a vague question — you'll waste cycles on wrong targets.

**When to abbreviate:** Not every query needs full clarification. If the question is specific and unambiguous — "What's the latest stable version of React?" or "Does library X support Python 3.12?" — move directly to Phase 2 or even Phase 3. Clarification earns its time when a vague query would send you searching in the wrong direction. A good heuristic: if you can write the sub-questions without asking anything, the query was clear enough.

### Phase 2: Query Decomposition

Break the research question into 3-5 specific sub-questions that together form a complete picture:

```
Main: "Should we use PostgreSQL or MongoDB for this project?"

Sub-questions:
1. What are the data modeling requirements? (structured vs flexible)
2. What are the scaling requirements? (read-heavy, write-heavy, both)
3. What's the team's existing expertise?
4. What are the operational complexity tradeoffs?
5. What do production users of similar scale report?
```

Sub-questions should be independently searchable. Each should map to at least one source.

**Tavily integration:** Each sub-question becomes a separate `tavily-search` call. Design queries to be specific and independently searchable. When sub-questions are independent, fire them in parallel for speed.

### Phase 3: Source Gathering

Consult sources in this priority order, adapting to the domain:

| Source Type | Strengths | Best For | Tavily Strategy |
|-------------|-----------|----------|-----------------|
| **Official Docs** | Authoritative, maintained | API signatures, core concepts | `include_domains` targeting official site |
| **GitHub Issues/PRs** | Real problems, maintainer input | Edge cases, bugs, workarounds | `include_domains: ["github.com"]` |
| **Stack Overflow** | Curated answers, voting signal | Common problems, quick fixes | `include_domains: ["stackoverflow.com"]` |
| **Source Code** | Ground truth | When docs are unclear | Read source directly |
| **Blog Posts** | Deep dives, tutorials | Learning workflows, context | General search, then `tavily-extract` |
| **Discord/Forums** | Cutting-edge, insider knowledge | Latest changes, community consensus | `include_domains` targeting community sites |

**For each source, record:**
- URL and access date
- Version/date of the information
- Author credibility indicators
- Key claims with direct quotes when significant

---

### Tool Strategy: Tavily

Tavily is the primary research engine. It has two tools with distinct roles:

#### `tavily-search` — Discovery

Use for finding sources across the web. Start with `search_depth: "basic"` — the tool schema documents all parameters.

**Parallelization pattern:** Sub-questions from Phase 2 are independent. Fire multiple `tavily-search` calls simultaneously, each with different queries and domain filters:

```
# Parallel execution — all at once
Search 1: sub-question 1, include_domains: ["docs.python.org"]
Search 2: sub-question 2, include_domains: ["github.com"]
Search 3: sub-question 3, search_depth: "advanced"
Search 4: sub-question 4, topic: "news", days: 30
```

**Escalation pattern:** Start Simple applies to search depth:
1. First pass: `search_depth: "basic"`, `max_results: 10`
2. If results are thin or off-target: `search_depth: "advanced"`, `max_results: 15`
3. If still inadequate: narrow with `include_domains` or broaden the query

#### `tavily-extract` — Deep Reading

Use for pulling full content from specific URLs. The tool schema documents parameters; the strategic question is *when* to extract.

**When to extract vs. when search is enough:**
- Search result snippets answer the question → don't extract, move on
- Snippet is promising but incomplete → extract that URL
- Need to verify a specific claim in context → extract the source
- Comparing multiple in-depth sources → batch extract all candidate URLs

**Batch extraction pattern:** After search surfaces promising URLs, extract them in a single call:

```
tavily-extract(urls: [url1, url2, url3, url4])
```

This is faster than sequential `WebFetch` calls and returns cleaned content.

#### When to Use `WebFetch` Instead

Tavily handles most cases, but fall back to `WebFetch` when:
- You need a specific URL that Tavily extract can't parse (rare edge cases)
- The URL was provided directly by the user (not discovered via search)
- You need markdown-formatted output for readability

#### When to Use `WebSearch` Instead

`WebSearch` provides a second search engine perspective. Use it when:
- Tavily results seem thin or off-target for a particular topic
- You want independent corroboration for the Three-Source Rule
- The query benefits from different ranking signals

---

**Codebase grounding:** When the research topic intersects with the current codebase, read relevant source code for context before or during source gathering.

### Phase 4: Source Evaluation

Evaluate sources on relevance, credibility, currency, and specificity — but the practical question is *which sources to trust when they disagree*.

**Trust hierarchy (highest to lowest):**
1. Source code behavior (empirical test)
2. Official docs with version tags
3. Maintainer statements in issues/PRs
4. Highly upvoted + recently active Stack Overflow
5. Blog posts with working code examples
6. Unverified forum posts

**Red flags:**
- No version/date mentioned
- Code examples without imports
- "This worked for me" with no context
- Confidently stated but lacks detail
- Circular references between sources

### Phase 5: Conflict Resolution

When sources disagree:

1. **Check versions**: Conflict often means different versions, not factual disagreement. Use `tavily-search` with `time_range` to find version-specific information
2. **Find the maintainer**: Their comment trumps community answers. Search `include_domains: ["github.com"]` for maintainer statements in issues/PRs
3. **Deep-read both sides**: Use `tavily-extract` on the conflicting URLs to get full context — snippets often make sources seem more contradictory than they are
4. **Test empirically**: Check codebase evidence when available, or note the limitation
5. **Apply consensus weighting**: Run a broader search (`max_results: 20`) to gauge which position has more independent support
6. **Note the disagreement**: If unresolved, report both positions with evidence in the Conflicts section of output

### Phase 6: Synthesis

Key quality standards:

- Every factual claim has a citation
- Quantitative data preferred over qualitative assertions
- Multiple perspectives on controversial topics
- Conclusions are specific, not vague generalizations
- The Synthesis section gives a direct answer — no hedging with "it depends"
- The Confidence section honestly assesses source quality and consensus level
- Conflicts are surfaced, not hidden

**Output template:** Structure research output to be immediately actionable. Adapt the template to scope — a quick version lookup doesn't need every section, while a technology comparison needs all of them.

````markdown
## [Direct Answer to the Research Question]

[1-3 sentence answer. Be specific. If the answer is "use X," say why.]

### Key Findings

1. **[Finding]** — [Detail with citation] ([source](URL), [date])
2. **[Finding]** — [Detail with citation] ([source](URL), [date])
3. **[Finding]** — [Detail with citation] ([source](URL), [date])

### Conflicts and Disagreements

[Where sources disagreed and what that means. If no conflicts, say so
explicitly — it's a signal of high consensus.]

### Confidence Assessment

- **Overall confidence**: [High/Medium/Low]
- **Reasoning**: [What makes the evidence strong or weak]
- **Source agreement**: [Did sources converge or diverge?]

### Limitations

- [What this research did NOT cover]
- [Conditions under which findings might not apply]
- [Time-sensitivity: when these findings should be re-verified]
````

---

## Capturing Practical Wisdom

Official docs miss the "clever hacks" and "power user moves." Target them with queries like these (combine with `include_domains` for precision):

| Pattern | Query Template | Domain Filter |
|---------|---------------|---------------|
| GitHub issue archaeology | `[tech] workaround OR hack OR trick` | `github.com` |
| Maintainer statements | `[tech] [problem] fix OR resolved` | `github.com`, `search_depth: "advanced"` |
| Stack Overflow deep cuts | `[tech] [problem] note that OR also need to` | `stackoverflow.com` |
| Recent developments | `[tech] [feature] announcement OR release` | Use `time_range: "month"` |
| Breaking changes | `[tech] breaking change OR migration guide` | Use `topic: "news"` |
| Config wisdom | `[tool] dotfiles OR awesome-[tool] config` | `github.com` |

**Emotional language signals:** Hard-won knowledge often comes with markers like "Finally!", "After hours of debugging...", "The trick is...", "What the docs don't tell you..." When you find such knowledge, capture it with context: `[Problem] / [Source + date] / [Version] / [Solution] / [Caveats]`.

---

## Anti-Patterns to Avoid

| Anti-Pattern | Why It Fails | Instead |
|--------------|--------------|---------|
| **Single-source answers** | No verification | Cross-reference 3+ sources |
| **Skipping clarification** | Vague query -> poor results | Ask questions first |
| **Trusting first results** | Confirmation bias | Seek disconfirming evidence |
| **Copy-paste without understanding** | Cargo culting | Understand the "why" |
| **Ignoring version/date** | Outdated info applied incorrectly | Always note versions |
| **Vague conclusions** | "It depends" helps no one | Make a specific recommendation |
| **Hiding uncertainty** | Overconfidence misleads | State confidence levels |

---

## Quality Checklist

Before concluding research, verify:

- [ ] Core question answered directly and specifically
- [ ] 3+ diverse sources consulted
- [ ] All factual claims have citations with URLs
- [ ] Version/date noted for time-sensitive information
- [ ] Conflicting sources investigated and explained (not hidden)
- [ ] Quantitative data included where available
- [ ] Practical wisdom captured (not just official docs)
- [ ] Conclusion is concrete, not hedged
- [ ] Confidence level stated with reasoning
- [ ] Limitations and caveats acknowledged
- [ ] Output is structured, cited, and directly answers the question

---

## Persisting Findings

Research outputs stay in conversation by default. When findings need to persist beyond the session, use the project's preferred knowledge management approach — save to designated reference locations, create issues for actionable findings, or archive insights as project conventions dictate.

Persistence is the user's decision, not automatic.
