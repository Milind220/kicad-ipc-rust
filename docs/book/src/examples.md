# Examples

## Quick Version Probe (Async)

```rust,no_run
use kicad_ipc_rs::KiCadClient;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), kicad_ipc_rs::KiCadError> {
    let client = KiCadClient::connect().await?;
    let version = client.get_version().await?;
    println!("{:?}", version);
    Ok(())
}
```

## Open Board Detection (Blocking)

```rust,no_run
use kicad_ipc_rs::KiCadClientBlocking;

fn main() -> Result<(), kicad_ipc_rs::KiCadError> {
    let client = KiCadClientBlocking::connect()?;
    let has_board = client.has_open_board()?;
    println!("open board: {}", has_board);
    Ok(())
}
```

## CLI-first Smoke Testing

Runbook commands:

```bash
cargo run --features blocking --bin kicad-ipc-cli -- ping
cargo run --features blocking --bin kicad-ipc-cli -- version
cargo run --features blocking --bin kicad-ipc-cli -- board-open
```

Full command catalog: [docs/TEST_CLI.md](https://github.com/Milind220/kicad-ipc-rs/blob/main/docs/TEST_CLI.md)
