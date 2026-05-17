# AGENTS.md — Collaborative Code Editor

## Repo state

Planning phase. `collab-editor-spec.md` is the source of truth for architecture, tech stack, and phased plan. See `docs/plan/tasks/` for the detailed task breakdown.

## When scaffolding

Target structure:

```
Cargo.toml
src/
  main.rs  auth/  rooms/  collaboration/  linter/  db/  models/  error/
migrations/
frontend/
  package.json  src/components/  src/hooks/  src/pages/
```

## Key constraints from spec

- Backend: Rust, Axum, Tokio, SQLite/sqlx, tokio-tungstenite (WS), y-crate (CRDT), serde, argon2
- Frontend: React + Vite + TS, CodeMirror 6, Tailwind
- 4 phased implementation plan defined in spec (Foundation → Collaboration → Linting → Polish)
