# P1-T01 — Rust Project Scaffolding

**Status:** Done

## Goal

Create the Rust project skeleton with all dependencies and entry point.

## Description

Initialize the Cargo workspace as the foundation of the entire application. This sets up the package configuration with all required dependencies and provides the `main.rs` entry point that will be wired up as the project grows.

The project uses:
- **Axum** for HTTP routing
- **tokio** with full features for async runtime
- **sqlx** with `runtime-tokio-rustls` and `sqlite` features for database access
- **yrs** with `sync` and `serde` features for CRDT document management
- **argon2** for password hashing
- **serde / serde_json** for serialization
- **tokio-tungstenite** for WebSocket support
- **tower-http** with `fs`, `trace`, and `cors` for static files, logging, and CORS
- **uuid** with `v4` and `serde` features for room code generation
- **tracing / tracing-subscriber** for structured logging

This task only creates the skeleton — no routing logic or business logic is implemented yet.

## Acceptance Criteria

- AC1: `cargo.toml` exists with all required dependencies listed above
- AC2: `src/main.rs` has `#[tokio::main] fn main()` calling a `build_app()` function
- AC3: `build_app()` returns `axum::Router` and compiles successfully
- AC4: `cargo check` passes with zero errors
- AC5: `cargo run` starts the process (will exit quickly since no server is listening yet)

## Technical Hints

- Use `cargo init --name collab-editor` to scaffold
- For sqlx, set `SQLX_OFFLINE=true` and run `cargo sqlx migrate run` after migrations are created
- Enable yrs `sync` and `serde` feature flags: `yrs = { version = "0.26", features = ["sync", "serde"] }`
- Use `tracing-subscriber::fmt().init()` in main for default logging
- Refer to Axum docs: https://docs.rs/axum/latest/axum/
