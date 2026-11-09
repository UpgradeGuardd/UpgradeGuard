# UpdrageGuard Documentation

Project documentation lives here.

## Contents

- `architecture.md` — high-level architecture and data flow across the crates.
- `crates/` — per-crate design notes.
- `development.md` — local setup and contribution guide.

## Overview

UpdrageGuard is a modular monorepo. The Rust crates under `crates/` form the
analysis pipeline, the `apps/api` binary exposes it over HTTP, `apps/cli`
exposes it on the command line, and `apps/web` provides a React UI.

The analysis pipeline flows roughly:

```
WASM analysis → version comparison → storage compatibility
                 ↓
        migration simulation → test generation → risk assessment → report
```
