# Testing Anti-Patterns

**Load this reference when:** writing or changing tests, adding mocks, or
tempted to add test-only methods to production code.

## Overview

Tests must verify real behavior, not mock behavior. Mocks are a means to
isolate, not the thing being tested.

**Core principle:** Test what the code does, not what the mocks do.

**Following strict test-first discipline prevents these anti-patterns.**

## The Iron Laws

```
1. NEVER test mock behavior
2. NEVER add test-only methods to production classes
3. NEVER mock without understanding dependencies
```

## Anti-Pattern 1: Testing Mock Behavior

**The violation:**
```python
# BAD: Testing that the mock exists
def test_renders_sidebar(mock_sidebar):
    page = render_page()
    assert page.find("sidebar-mock") is not None
```

**Why this is wrong:**
- You are verifying the mock works, not that the component works
- Test passes when mock is present, fails when it is not
- Tells you nothing about real behavior

**Recognition:** The assertion checks for mock elements, not real behavior.

**The fix:**
```python
# GOOD: Test real component or do not mock it
def test_renders_sidebar():
    page = render_page()  # Do not mock sidebar
    assert page.find(role="navigation") is not None

# OR if sidebar must be mocked for isolation:
# Do not assert on the mock — test the page's behavior with sidebar present
```

### Gate Function

```
BEFORE asserting on any mock element:
  Ask: "Am I testing real component behavior or just mock existence?"

  IF testing mock existence:
    STOP — Delete the assertion or unmock the component

  Test real behavior instead
```

## Anti-Pattern 2: Test-Only Methods in Production

**The violation:**
```python
# BAD: destroy() only used in tests
class Session:
    def destroy(self):  # Looks like production API!
        if self._workspace_manager:
            self._workspace_manager.destroy_workspace(self.id)
        # ... cleanup

# In tests
def teardown():
    session.destroy()
```

**Why this is wrong:**
- Production class polluted with test-only code
- Dangerous if accidentally called in production
- Violates YAGNI and separation of concerns
- Confuses object lifecycle with entity lifecycle

**The fix:**
```python
# GOOD: Test utilities handle test cleanup
# Session has no destroy() — it is stateless in production

# In test_utils.py
def cleanup_session(session):
    workspace = session.get_workspace_info()
    if workspace:
        workspace_manager.destroy_workspace(workspace.id)

# In tests
def teardown():
    cleanup_session(session)
```

### Gate Function

```
BEFORE adding any method to production class:
  Ask: "Is this only used by tests?"

  IF yes:
    STOP — Do not add it
    Put it in test utilities instead

  Ask: "Does this class own this resource's lifecycle?"

  IF no:
    STOP — Wrong class for this method
```

## Anti-Pattern 3: Mocking Without Understanding

**The violation:**
```python
# BAD: Mock breaks test logic
def test_detects_duplicate_server(monkeypatch):
    # Mock prevents config write that test depends on!
    monkeypatch.setattr(
        "tool_catalog.discover_and_cache_tools",
        lambda: None
    )

    add_server(config)
    add_server(config)  # Should raise — but will not!
```

**Why this is wrong:**
- Mocked method had a side effect the test depended on (writing config)
- Over-mocking to "be safe" breaks actual behavior
- Test passes for wrong reason or fails mysteriously

**The fix:**
```python
# GOOD: Mock at correct level
def test_detects_duplicate_server(monkeypatch):
    # Mock the slow part, preserve behavior test needs
    monkeypatch.setattr("server_manager.start", lambda *a: None)

    add_server(config)  # Config written
    with pytest.raises(DuplicateServerError):
        add_server(config)  # Duplicate detected
```

### Gate Function

```
BEFORE mocking any method:
  STOP — Do not mock yet

  1. Ask: "What side effects does the real method have?"
  2. Ask: "Does this test depend on any of those side effects?"
  3. Ask: "Do I fully understand what this test needs?"

  IF depends on side effects:
    Mock at lower level (the actual slow/external operation)
    OR use test doubles that preserve necessary behavior
    NOT the high-level method the test depends on

  IF unsure what test depends on:
    Run test with real implementation FIRST
    Observe what actually needs to happen
    THEN add minimal mocking at the right level

  Red flags:
    - "I'll mock this to be safe"
    - "This might be slow, better mock it"
    - Mocking without understanding the dependency chain
```

## Anti-Pattern 4: Incomplete Mocks

**The violation:**
```python
# BAD: Partial mock — only fields you think you need
mock_response = {
    "status": "success",
    "data": {"user_id": "123", "name": "Alice"}
    # Missing: metadata that downstream code uses
}

# Later: breaks when code accesses response["metadata"]["request_id"]
```

**Why this is wrong:**
- Partial mocks hide structural assumptions
- Downstream code may depend on fields you did not include
- Tests pass but integration fails
- False confidence — test proves nothing about real behavior

**The fix:**
```python
# GOOD: Mirror real API completeness
mock_response = {
    "status": "success",
    "data": {"user_id": "123", "name": "Alice"},
    "metadata": {"request_id": "req-789", "timestamp": 1234567890}
    # All fields real API returns
}
```

### Gate Function

```
BEFORE creating mock responses:
  Check: "What fields does the real API response contain?"

  Actions:
    1. Examine actual API response from docs/examples
    2. Include ALL fields system might consume downstream
    3. Verify mock matches real response schema completely

  Critical:
    If you are creating a mock, you must understand the ENTIRE structure
    Partial mocks fail silently when code depends on omitted fields

  If uncertain: Include all documented fields
```

## Anti-Pattern 5: Integration Tests as Afterthought

**The violation:**
```
Implementation complete
No tests written
"Ready for testing"
```

**Why this is wrong:**
- Testing is part of implementation, not optional follow-up
- Test-first discipline would have caught this
- Cannot claim complete without tests

**The fix:**
```
Test-first cycle:
1. Write failing test
2. Implement to pass
3. Refactor
4. THEN claim complete
```

## When Mocks Become Too Complex

**Warning signs:**
- Mock setup longer than test logic
- Mocking everything to make test pass
- Mocks missing methods real components have
- Test breaks when mock changes

**Consider:** Integration tests with real components are often simpler than
complex mocks. If you are mocking so much that the mock is harder to
understand than the real code, use the real code.

## Test-First Discipline Prevents These Anti-Patterns

1. **Write test first** — forces you to think about what you are actually
   testing
2. **Watch it fail** — confirms test checks real behavior, not mocks
3. **Minimal implementation** — no test-only methods creep in
4. **Real dependencies** — you see what the test actually needs before mocking

If you are testing mock behavior, you violated the test-first discipline — you
added mocks without watching the test fail against real code first.

## Quick Reference

| Anti-Pattern | Fix |
|--------------|-----|
| Assert on mock elements | Test real component or unmock it |
| Test-only methods in production | Move to test utilities |
| Mock without understanding | Understand dependencies first, mock minimally |
| Incomplete mocks | Mirror real API completely |
| Tests as afterthought | Test-first — tests before implementation |
| Over-complex mocks | Consider integration tests |

## Red Flags

- Assertion checks for mock test IDs
- Methods only called in test files
- Mock setup is more than half the test
- Test fails when you remove the mock
- Cannot explain why mock is needed
- Mocking "just to be safe"
