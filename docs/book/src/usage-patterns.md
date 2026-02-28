# Usage Patterns

This chapter targets repeatable integration patterns for tool builders and code generators.

## Pattern: Cheap Health Check

Use at process startup to validate socket + auth + server liveness.

```rust,no_run
use kicad_ipc_rs::KiCadClient;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), kicad_ipc_rs::KiCadError> {
    let client = KiCadClient::connect().await?;
    client.ping().await?;
    Ok(())
}
```

## Pattern: Read-only Query Pipeline

Recommended order for board-aware reads:

1. `get_open_documents()`
2. `get_nets()`
3. `get_items_by_net(...)` or `get_items_by_type_codes(...)`

Reason: fail fast on document state before expensive item traversal.

## Pattern: Safe Write Session

Use begin/end commit around mutating commands.

1. `begin_commit(...)`
2. `create_items(...)` / `update_items(...)` / `delete_items(...)`
3. `end_commit(..., CommitAction::Commit, ...)`

If errors mid-flight: close with `CommitAction::Abort`/`Drop` per flow.

## Common Pitfalls

| Pitfall | Symptom | Avoidance |
| --- | --- | --- |
| Assume KiCad always running | connect errors at startup | explicit prereq check + `ping()` |
| Skip open-document check | downstream command failures | call `get_open_documents()` first |
| Mix sync + async API unintentionally | duplicate runtime ownership | pick one surface per process |
| Fire write commands without commit session | partial or rejected mutations | always bracket writes with commit APIs |
| Hardcode unsupported commands | `AS_UNHANDLED` at runtime | map/handle `RunActionStatus` and runtime flags |

## Async vs Blocking Selection

| Requirement | Preferred API |
| --- | --- |
| Tokio app / async daemon | `KiCadClient` |
| Existing sync binary | `KiCadClientBlocking` |
| Lowest integration friction for scripts | `KiCadClientBlocking` + CLI |

## Reliability Checklist

- Set explicit `client_name` for traceability.
- Keep request timeout defaults unless measured need.
- Handle transport + protocol errors as recoverable boundary.
- Use typed wrappers when available; drop to raw only when needed.
