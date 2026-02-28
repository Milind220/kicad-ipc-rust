# Validation and Testing

Before handoff or release:

```bash
cargo fmt --all
cargo test
cargo test --features blocking
```

## Evidence Pointers

- Unit tests across client/model/blocking/CLI parser paths:
  - [`src/client.rs`](https://github.com/Milind220/kicad-ipc-rs/blob/main/src/client.rs)
  - [`src/blocking.rs`](https://github.com/Milind220/kicad-ipc-rs/blob/main/src/blocking.rs)
  - [`src/model/common.rs`](https://github.com/Milind220/kicad-ipc-rs/blob/main/src/model/common.rs)
  - [`src/model/board.rs`](https://github.com/Milind220/kicad-ipc-rs/blob/main/src/model/board.rs)
  - [`test-scripts/kicad-ipc-cli.rs`](https://github.com/Milind220/kicad-ipc-rs/blob/main/test-scripts/kicad-ipc-cli.rs)
- Runtime command coverage matrix:
  - [README coverage section](https://github.com/Milind220/kicad-ipc-rs#kicad-v10-rc11-api-completion-matrix)
- Runtime CLI verification flow:
  - [docs/TEST_CLI.md](https://github.com/Milind220/kicad-ipc-rs/blob/main/docs/TEST_CLI.md)

## CI Notes

- API/release pipeline: `.github/workflows/release-plz.yml`
- Book deploy pipeline: `.github/workflows/mdbook.yml`
