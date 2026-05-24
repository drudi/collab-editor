# AGENTS.md — Collaborative Code Editor

## Repo state

**Current status:** Phase 1, Tasks 1–2 complete. Rust project scaffolded with auth registration endpoint, HTTP server on port 3000, DB connection, and CORS support.

**Completed tasks:**
- ✅ P1-T01: Rust project scaffolding (`Cargo.toml` + `src/main.rs` with `build_app()` → `Router::new()`)
- ✅ P1-T10: Auth registration (`POST /api/auth/register` — input validation, argon2id password hashing, session creation, HttpOnly cookie)
- ✅ P1-T11: Auth login (`POST /api/auth/login` — user lookup, argon2 password verification, session creation, HttpOnly cookie)

**Remaining:** 50 tasks across 4 phases. See `docs/plan/tasks/` for the full task breakdown with statuses.

## Rust toolchain

- **Required: Rust 1.95+** (updated from 1.83 during setup)
- Some transitive dependencies (e.g. `getrandom 0.4`) require Rust 1.85+ (edition2024)
- Update with: `rustup update stable`

## Key dependencies (all at latest compatible versions)

| Dependency | Version | Notes |
|---|---|---|
| axum | 0.8 | Latest 0.8.9 |
| tokio | 1 | Full features |
| sqlx | 0.8 | runtime-tokio-rustls + sqlite |
| yrs | 0.26 | **features: `sync` only** (no `serde` feature exists) |
| tokio-tungstenite | 0.29 | **features: `rustls-tls-webpki-roots`** (not `rustls-tls`) |
| tower-http | 0.6 | fs + trace + cors |
| argon2 | 0.5 | Stable (0.6 is rc) |
| uuid | 1 | v4 + serde |
| rand | 0.9 | Random byte/token generation |
| tracing / tracing-subscriber | 0.1 / 0.3 | env-filter feature |

**After any dependency change:** run `cargo update` then verify with `cargo check`.

## Known pitfalls (learned during setup)

1. **yrs has no `serde` feature** — `yrs` only offers features: `default`, `small-client`, `sync`, `weak`. Use external serde for serializing CRDT data.
2. **tokio-tungstenite TLS feature** — Use `rustls-tls-webpki-roots`, not `rustls-tls`.
3. **Rust version** — Must be ≥ 1.95. Older versions (1.83) fail on transitive deps requiring edition2024.
4. **`cargo run` on empty Router** — The skeleton prints "built successfully" but has no listener. Add HTTP server in a later task.
5. **sqlx compile-time checks need `.sqlx/` cache** — `query!()` and `migrate!()` macros verify SQL at build time. After any migration change, run `cargo sqlx prepare --database-url <url>` to regenerate the cache. If the cache is stale, builds fail with compile errors — that's by design.

## Project structure (target)

```
Cargo.toml
src/
  main.rs           # Entry point, build_app() → Router
  lib.rs            # Module re-exports
  auth/             # Registration, login, session middleware
  rooms/            # Room CRUD, membership
  collaboration/    # WebSocket hub, CRDT sync, broadcast
  linter/           # LSP server integration and lint service
  db/               # Database connection, migrations, query helpers
  models/           # Shared domain structs (User, Session, Room, etc.)
  error/            # Unified error types (AppError, IntoResponse)
migrations/         # SQL migration files (001_create_users.sql, etc.)
frontend/
  package.json
  src/
    components/     # Editor, UserList, RoomBrowser, etc.
    hooks/          # useWebSocket, useAuth, useLinting, etc.
    pages/          # Login, Register, Room, Home
    contexts/       # AuthContext, etc.
    collab/         # Yjs binding, CodeMirror extensions, cursor rendering
```

## When scaffolding new modules

1. Create directory under `src/` matching the domain (auth, rooms, collaboration, linter, db, models, error)
2. Add `mod.rs` with public API exports
3. Add `pub mod` in `src/lib.rs` (or `src/main.rs` if using binary-only mode)
4. Run `cargo check` to verify

## Key constraints from spec

- **Backend:** Rust, Axum, Tokio, SQLite/sqlx, tokio-tungstenite (WS), y-crate (CRDT), serde, argon2
- **Frontend:** React + Vite + TS, CodeMirror 6, Tailwind
- **4 phased plan:** Foundation → Collaboration → Linting → Polish
- **Database schema:** users, sessions, rooms, room_members, documents

## Implementation order (critical path)

```
P1-T01 → P1-T02 → P1-T04 → P1-T05/P1-T06/P1-T07/P1-T08 → P1-T09 → P1-T11 ✅ → P1-T12 → P1-T14 → P1-T15
                             → P1-T10 → P1-T13
P1-T16 → P1-T17, P1-T18 → P1-T19 → P1-T20, P1-T21

P2-T01 → P2-T02 → P2-T03 → P2-T04 → P2-T05
P2-T06 → P2-T07 → P2-T08 → P2-T09 → P2-T12 → P2-T13
P2-T10 → P2-T11

P3-T01 → P3-T02 → P3-T03 → P3-T04
P3-T04 ← P2-T08

P4-T01 → P4-T02
P4-T03, P4-T04, P4-T05, P4-T06
```

## Task tracking

Each task file in `docs/plan/tasks/` has:
- `Status:` field (TODO/Done)
- Acceptance criteria (AC1, AC2, etc.)
- Technical hints

When completing a task:
1. Update its `Status` to `Done`
2. Update `docs/plan/tasks/README.md` table with ✅
3. Check `cargo check` passes
