# /StandardChecks

Use for any feature/change in this repo.

## Required Flow

1. Confirm low-level proto fields and comments for new/changed behavior.
2. Map into typed model structs (async path) with explicit absence handling (`Option`) and unknown-enum handling (`Unknown(...)` / `UNKNOWN_LAYER(...)`).
3. Update sync/blocking parity (`KiCadClientBlocking`) for any new async methods.
4. Update CLI surface (`test-scripts/kicad-ipc-cli.rs`) so feature is observable from terminal.
5. Add regression tests:
   - decode/mapping test(s)
   - CLI arg parse test(s) for new command/flags
   - detail/format test(s) if output changed
6. Update docs touched by behavior/API (`README.md`, `docs/TEST_CLI.md`, runbooks as needed).
7. Run checks:
   - `cargo fmt --all`
   - `cargo test`
   - `cargo test --features blocking`
8. Live verify with KiCad open (real socket): run CLI command(s) showing new data path and confirm expected fields.
9. Ship:
   - `git status`
   - commit with Conventional Commit
   - push branch
   - open PR
   - request review with `@codex review`.

## Output Expectations

- Include exact file refs + what changed.
- Include command outputs summary for tests/live run.
- Call out any skipped checks with reason.
