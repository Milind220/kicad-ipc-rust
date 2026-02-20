# CONTRIBUTIONS

## Scope

This crate uses a two-layer workflow:

1. Upstream source of truth: KiCad proto files in the `kicad` submodule.
2. Published crate input: checked-in generated Rust in `src/proto/generated/`.

Consumers should not need `protoc` or submodules.

## When To Regenerate Protos

Regenerate when:

- KiCad API `.proto` changed upstream.
- You bump the `kicad` submodule commit.
- You need newly added/changed proto messages/enums/services.

Do **not** regenerate when:

- You only change handwritten Rust API/client code.
- You only add docs/tests unrelated to proto schema.
- You only refactor internals without schema updates.

## Maintainer Regen Flow

```bash
git submodule update --init --recursive
./scripts/regenerate-protos.sh
```

This updates:

- `src/proto/generated/*.rs`
- `src/kicad_api_version.rs`

## Commit Prefix Convention

Use this commit prefix when committing generated proto refresh output:

- `chore(proto-gen): ...`

Examples:

- `chore(proto-gen): refresh bindings from KiCad rev-b5121435`
- `chore(proto-gen): regenerate after submodule update`

Use normal conventional commit types for handwritten changes (`feat`, `fix`, `refactor`, `test`, etc.).

## Suggested PR Structure

Prefer splitting into two commits:

1. `chore(proto-gen): ...` (generated output only)
2. Handwritten API/client updates (`feat|fix|refactor|test|docs`)

This keeps review diff clean.

## Pre-PR Checks

Run before opening/updating PR:

```bash
cargo fmt --all --check
cargo test
cargo check --all-features
cargo package
```
