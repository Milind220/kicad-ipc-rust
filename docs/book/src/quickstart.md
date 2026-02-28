# Quickstart

## Prereqs

1. KiCad running on the same machine.
2. IPC socket available (default discovery, or `KICAD_API_SOCKET`).
3. Optional auth token in `KICAD_API_TOKEN` if your setup requires it.

## Async API (default)

`Cargo.toml`:

```toml
[dependencies]
kicad-ipc-rs = "0.3.1"
tokio = { version = "1", features = ["macros", "rt"] }
```

```rust,no_run
use kicad_ipc_rs::KiCadClient;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), kicad_ipc_rs::KiCadError> {
    let client = KiCadClient::builder()
        .client_name("quickstart-async")
        .connect()
        .await?;

    client.ping().await?;
    let version = client.get_version().await?;
    println!("KiCad: {}", version.full_version);
    Ok(())
}
```

## Blocking API

`Cargo.toml`:

```toml
[dependencies]
kicad-ipc-rs = { version = "0.3.1", features = ["blocking"] }
```

```rust,no_run
use kicad_ipc_rs::KiCadClientBlocking;

fn main() -> Result<(), kicad_ipc_rs::KiCadError> {
    let client = KiCadClientBlocking::builder()
        .client_name("quickstart-blocking")
        .connect()?;

    client.ping()?;
    let version = client.get_version()?;
    println!("KiCad: {}", version.full_version);
    Ok(())
}
```

## Environment Variables

| Variable | Purpose | Used by |
| --- | --- | --- |
| `KICAD_API_SOCKET` | Explicit IPC socket URI/path override | async + blocking |
| `KICAD_API_TOKEN` | IPC auth token | async + blocking |

## Next Steps

- Use [`kicad-ipc-cli`](https://github.com/Milind220/kicad-ipc-rs/blob/main/test-scripts/kicad-ipc-cli.rs) for rapid command checks.
- Follow [Validation and Testing](validation.md) before CI/release.
