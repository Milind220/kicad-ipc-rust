# Introduction

`kicad-ipc-rs` is an async-first Rust client for KiCad IPC.

Project goals:

- Rust-native API for KiCad IPC commands.
- Typed models for common board/editor operations.
- Blocking wrapper parity via `feature = "blocking"`.
- Maintainer-friendly release and proto-regeneration flow.

Current scope:

- KiCad API proto snapshot pinned in repo (`src/proto/generated/`).
- 56/56 wrapped command families from the current snapshot.
- Runtime compatibility verified against KiCad `10.0.0-rc1`.

Core entrypoints:

- Async: `kicad_ipc_rs::KiCadClient`
- Blocking: `kicad_ipc_rs::KiCadClientBlocking` (`blocking` feature)
- Error type: `kicad_ipc_rs::KiCadError`

Related docs:

- Crate README: [README.md](https://github.com/Milind220/kicad-ipc-rs/blob/main/README.md)
- CLI runbook: [docs/TEST_CLI.md](https://github.com/Milind220/kicad-ipc-rs/blob/main/docs/TEST_CLI.md)
- API docs: [docs.rs/kicad-ipc-rs](https://docs.rs/kicad-ipc-rs)
