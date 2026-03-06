# Strict Issue Sync Gate in `groundwork init` Design

## Objective
`groundwork init` must only report success when issue sync has completed at least one full pull. Operationally, `gh-issue-sync status` must report `Last full pull:` with a non-`never` value.

## Scope
- Change `init` behavior from best-effort issue sync bootstrap to strict verification gate.
- Preserve `update` behavior as non-blocking (current best-effort mode).
- Keep diagnostics explicit and actionable for auth/scope/connectivity failures.

## Design
1. Introduce an init-mode gate in issue sync bootstrap.
- On `init`, after install/init/pull attempt, run `gh-issue-sync status` and parse `Last full pull:`.
- If status cannot be read, or it reports `never`, fail `init` with non-zero exit.

2. Keep remediation inline and deterministic.
- Emit scope/auth/network hints from existing classifier.
- Always include concrete next commands:
  - `gh auth refresh -h github.com -s project`
  - `gh-issue-sync pull`
  - `gh-issue-sync status`

3. Ensure failure propagates.
- Replace silent warning path for init with an error path returned to caller.
- Keep lock writing and success output only after gate passes.

## Error Handling
- `gh-issue-sync init` failure during `init` is fatal.
- `gh-issue-sync pull` non-zero during `init` is fatal.
- `gh-issue-sync status` missing/invalid/`never` during `init` is fatal.
- For `update`, all above remain warnings.

## Testing Strategy
- Add unit tests for a new verifier helper:
  - success when status includes timestamp
  - failure when status includes `never`
  - failure when status command errors/fails
- Add integration-style test by introducing injectable command runner for issue sync flow where needed, or isolate logic into pure helper functions and test those deterministically.

## Non-Goals
- No retries/backoff in this change.
- No new CLI flags.
- No behavior change for `groundwork update`.
