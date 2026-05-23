# P1-T02 — Module Directory Structure

**Status:** Done

## Goal

Create the complete module directory tree and placeholder `mod.rs` files.

## Description

Organize the Rust source code into a modular architecture following clean separation of concerns. Each module directory represents a distinct domain of the application:

- `src/auth/` — authentication logic (registration, login, session management)
- `src/rooms/` — room CRUD operations and room management
- `src/collaboration/` — WebSocket handling, room state, CRDT sync
- `src/linter/` — LSP server integration and lint service
- `src/db/` — database connection, migrations, query helpers
- `src/models/` — shared domain structs
- `src/error/` — unified error types

Each directory gets a `mod.rs` that exports the public API of that module. The `src/lib.rs` re-exports all modules so consumers can access them via `collab_editor::auth::register`.

The `migrations/` directory gets a placeholder `00000000000000_initial.sql` with a `-- Placeholder` comment.

## Acceptance Criteria

- AC1: All seven module directories exist under `src/`
- AC2: Each `mod.rs` contains `pub mod` declarations for the module's public items
- AC3: `src/lib.rs` re-exports all modules with `pub mod`
- AC4: `src/main.rs` imports from `collab_editor::lib` (or uses the module directly)
- AC5: `migrations/00000000000000_initial.sql` exists as placeholder
- AC6: `cargo check` still passes after structure changes

## Technical Hints

- Use idiomatic Rust module organization: directories with `mod.rs` or nested `mod` files
- In `lib.rs`: `pub mod auth { pub mod mod; }` or `pub mod auth;` for flat modules
- Keep `main.rs` minimal: just `fn main()` and app builder call
- The module structure should support adding new submodules without circular dependencies
- Refer to: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html
