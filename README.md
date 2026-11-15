# SigmaOS-tools-hermes-agent

> **Hermes Agent — AI Runtime** — Developer / end-user application integration for SigmaOS.

Part of the [SigmaOS](https://github.com/AaryanSinghChauhan09/SigmaOS) ecosystem.  
Local Hermes LLM agent and reasoning runtime — Nous Research

## Quickstart

```bash
cargo test    # run deterministic AAA unit tests
cargo check   # verify compilation
```

## Component Role

This crate implements the `hermes_agent` subsystem for SigmaOS:
- Fully real Rust implementation (no stubs)
- Fail-fast validation with typed errors
- Deterministic unit tests (Arrange-Act-Assert pattern)
- Single Responsibility, Modular, Zero-Copy where applicable

## API

```rust
use sigmaos_tools_hermes_agent::*;
// See src/lib.rs for full public API
```

## Documentation

Full documentation is in the [SigmaOS GitHub Wiki → Tools-Ecosystem](https://github.com/AaryanSinghChauhan09/SigmaOS/wiki/Tools-Ecosystem).

## License

MIT
