# Collaborative Code Editor — Project Specification

## 1. Overview

A real-time collaborative code editor where multiple users can simultaneously edit code files in shared rooms. The backend is written in **Rust** (learning ownership, async, concurrency, error handling). The frontend is **React + TypeScript** with **CodeMirror 6** as the editor component.

## 2. Tech Stack

| Layer | Choice | Why (Rust learning value) |
|---|---|---|
| Backend framework | **Axum** | Modern, Tower-based, exercises Rust trait systems and middleware |
| Async runtime | **Tokio** | Learn async/await, task spawning, channels |
| Database | **SQLite + sqlx** | Learn compile-time query validation, connection management |
| Real-time | **WebSocket (tokio-tungstenite)** | Learn bidirectional async streams |
| CRDT | **y-crate (yrs)** | Rust port of Yjs — learn unsafe FFI, shared state patterns |
| Auth | **Sessions + argon2** | Learn password hashing, secure cookie handling, middleware |
| Serialization | **serde** | Core Rust serialization patterns |
| Frontend | **React + Vite + TypeScript** | Familiar tooling |
| Editor | **CodeMirror 6** | Lightweight, has Collaboration extension built-in |
| Styling | **Tailwind CSS** | Rapid UI development |

## 3. Core Features

### Authentication
- User registration with password hashing (argon2)
- Login/logout with secure HTTP-only session cookies
- Auth middleware protecting routes
- Session expiry and cleanup

### Rooms
- Create a room (returns unique room code)
- Join a room via code or URL
- Leave a room
- Room metadata: name, created_at, owner, language mode

### Collaborative Editing
- Multiple users edit the same file simultaneously
- Real-time cursor position sharing (see where others are typing)
- CRDT-based conflict resolution (no lock contention)
- Document history / undo support

### Linting
- Per-language linting configuration in the room
- Backend runs linter on document changes (debounced)
- Linting errors displayed in the editor gutter
- Support for at least: JavaScript/TypeScript (ESLint), Python (pylint/flake8), Rust (rustfmt/clippy)

### UI
- User list showing who's in the room
- User presence indicators (cursor color per user)
- Activity feed (file saved, user joined/left)

## 4. Architecture

```
Frontend (React + Vite)
├── /          → Landing page, room browser
├── /login     → Auth forms
├── /register  → Signup
├── /room/:id  → Editor page
│   ├── CodeMirror 6 editor
│   ├── WebSocket client → documents + cursors
│   └── REST API → room management, linting results
│
Backend (Axum + Tokio)
├── REST API (HTTP)
│   ├── POST   /api/auth/register
│   ├── POST   /api/auth/login
│   ├── POST   /api/auth/logout
│   ├── POST   /api/rooms
│   ├── GET    /api/rooms/:id
│   └── GET    /api/rooms/:id/lint
├── WebSocket hub
│   ├── /ws/:room_id  → document sync (CRDT updates)
│   └── Per-connection handler tasks
├── Room manager (shared state)
│   ├── Active room documents (in-memory Yrs state)
│   ├── Broadcast engine (fan-out to connected clients)
│   └── Persistence worker (periodic save to SQLite)
└── Linter service
    ├── Spawn linter process per request
    ├── Parse linter output
    └── Return structured diagnostics
```

## 5. Rust Learning Goals

| Concept | Where it appears |
|---|---|
| **Ownership & borrowing** | Passing document state between WebSocket handlers, room manager |
| **Lifetimes** | Struct fields referencing shared state, middleware |
| **Async/await** | All I/O, WebSocket message loops, linter spawning |
| **Concurrency** | Tokio tasks per connection, broadcast channels, Arc/Mutex |
| **Error handling** | `Result` types, custom error enums, `thiserror`/`anyhow` |
| **Traits** | Middleware, extractor patterns, linter abstraction |
| **Serialization** | serde derive macros, WebSocket message types |
| **FFI / unsafe** | y-crate bindings, subprocess spawning for linters |
| **Testing** | Unit tests, integration tests with `axum::TestClient` |

## 6. Database Schema

```sql
users (id, username, password_hash, created_at)
sessions (id, user_id, token, expires_at)
rooms (id, code, name, language, owner_id, created_at)
room_members (room_id, user_id, joined_at)
documents (id, room_id, content_snapshot, version, saved_at)
```

## 7. Project Structure

```
rust-arch-training/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── auth/           # Registration, login, session middleware
│   ├── rooms/          # Room CRUD, membership
│   ├── collaboration/  # WebSocket hub, CRDT sync, broadcast
│   ├── linter/         # Linter service, diagnostics
│   ├── db/             # sqlx models, migrations
│   ├── models/         # Shared types
│   └── error/          # Custom error types
├── migrations/
├── frontend/
│   ├── package.json
│   ├── src/
│   │   ├── components/  # Editor, UserList, RoomBrowser
│   │   ├── hooks/       # useWebSocket, useAuth, useLinting
│   │   └── pages/       # Login, Register, Room, Home
│   └── ...
└── README.md
```

## 8. Phased Implementation Plan

### Phase 1 — Foundation
- Rust project scaffolding, Axum setup, SQLite with sqlx
- User registration and login with session cookies
- Room creation and joining

### Phase 2 — Real-time Collaboration
- WebSocket server, connection handling
- CRDT integration (yrs), document sync
- Cursor position broadcasting
- CodeMirror 6 frontend with collaboration extension

### Phase 3 — Linting
- Linter service with subprocess spawning
- Debounced lint requests
- Display diagnostics in the editor

### Phase 4 — Polish
- Document persistence (periodic snapshots)
- Room activity feed
- Error pages, loading states
- Tests (unit + integration)
