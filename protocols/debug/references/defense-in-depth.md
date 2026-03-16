# Defense-in-Depth Validation

After finding and fixing a root cause, add validation at every layer the data
passes through. A single check can be bypassed by different code paths,
refactoring, or mocks. Multiple layers make the bug structurally impossible.

## The Four Layers

### Layer 1: Entry Point Validation

Reject obviously invalid input at the API boundary.

```
function createProject(name, workingDirectory):
    if workingDirectory is empty:
        throw "workingDirectory cannot be empty"
    if workingDirectory does not exist:
        throw "workingDirectory does not exist: {workingDirectory}"
```

### Layer 2: Business Logic Validation

Ensure data makes sense for this specific operation.

```
function initializeWorkspace(projectDir, sessionId):
    if projectDir is empty:
        throw "projectDir required for workspace initialization"
```

### Layer 3: Environment Guards

Prevent dangerous operations in specific contexts.

```
function gitInit(directory):
    if running in test environment:
        if directory is not under temp directory:
            throw "Refusing git init outside temp dir during tests"
```

### Layer 4: Debug Instrumentation

Capture context for forensics when other layers fail.

```
function gitInit(directory):
    log.debug("About to git init", {
        directory: directory,
        cwd: currentWorkingDirectory(),
        stack: captureStackTrace()
    })
```

## Applying the Pattern

When you fix a bug:

1. **Map the data flow.** List every point the data passes through from
   entry to the error site.
2. **Add validation at each layer.** Entry, business logic, environment,
   instrumentation.
3. **Test each layer independently.** Try to bypass layer 1 — verify layer 2
   catches it. Try to bypass layers 1 and 2 — verify layer 3 catches it.

Single validation: "We fixed the bug."
Multiple layers: "We made the bug impossible."
