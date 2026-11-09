# Tests

This directory holds cross-cutting integration tests and shared test fixtures
for UpdrageGuard.

## Layout

- `fixtures/` — sample contract WASM blobs and golden data used by integration
  tests.
- `integration/` — end-to-end tests that exercise multiple crates together
  (e.g. analyze → compare → assess → report).

## Running tests

Unit tests live alongside their crates and are run with:

```bash
cargo test --workspace
```

Workspace-level integration tests can be run with:

```bash
cargo test --workspace --test '*'
```
