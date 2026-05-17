# Collaborative Code Editor

A real-time collaborative code editor where multiple users can simultaneously edit code files in shared rooms.

## Tech Stack

**Backend:** Rust, Axum, Tokio, SQLite/sqlx, WebSocket, y-crate (CRDT), argon2
**Frontend:** React + Vite + TypeScript, CodeMirror 6, Tailwind CSS

## Features

- Real-time multi-user collaboration via CRDT-based conflict resolution
- WebSocket document sync with cursor position sharing
- User authentication with session cookies
- Room-based workspaces with language-aware linting
- Activity feed and user presence indicators

## Project Structure

```
Cargo.toml
src/          # Rust backend (auth, rooms, collaboration, linter, db)
docs/plan/    # Detailed planning, tasks, UX design
frontend/     # React + Vite + TypeScript frontend
```

## Implementation Phases

1. **Foundation** — Project scaffolding, auth, room CRUD
2. **Collaboration** — WebSocket hub, CRDT sync, CodeMirror 6 integration
3. **Linting** — Linter service, debounced diagnostics, editor gutter display
4. **Polish** — Persistence, error pages, tests

## Status

Planning phase. See [`docs/plan/`](docs/plan/) for the detailed task breakdown and UX designs.
