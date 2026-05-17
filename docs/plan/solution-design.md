# Solution Design — Collaborative Code Editor

## 1. System Overview

A real-time collaborative code editor with Rust backend (Axum + Tokio) and React frontend (CodeMirror 6). Multiple users edit documents simultaneously in rooms, with CRDT-based conflict resolution via Yjs (yrs crate), real-time cursor sharing, and per-language linting.

```mermaid
flowchart TB
    subgraph Browser["Client Browser"]
        FE["React + Vite + CodeMirror 6"]
        AuthPages["Login / Register pages"]
        RoomPage["Room editor page"]
        WSClient["WebSocket client"]
        RESTClient["REST client"]
    end

    subgraph Backend["Rust Backend (Axum + Tokio)"]
        AuthMod["Auth module"]
        SessionStore["Session store"]
        RoomMod["Room module"]
        CollabMod["Collaboration module"]
        RM["Room manager"]
        DocStore["In-memory Yrs docs"]
        Broadcast["Broadcast engine"]
        Persist["Persistence worker"]
        LinterMod["Linter service"]
        DB["SQLite (sqlx)"]
    end

    AuthPages --> RESTClient
    RoomPage --> RESTClient
    RoomPage --> WSClient
    RESTClient --> AuthMod
    RESTClient --> RoomMod
    RESTClient --> LinterMod
    WSClient --> CollabMod
    AuthMod --> SessionStore
    AuthMod --> DB
    RoomMod --> DB
    CollabMod --> RM
    RM --> DocStore
    RM --> Broadcast
    Persist --> DB
    LinterMod --> DB
```

## 2. Component Architecture

### 2.1 Backend Module Boundaries

```mermaid
flowchart TD
    subgraph "src/"
        main["main.rs"]
        auth["auth/"]
        rooms["rooms/"]
        collab["collaboration/"]
        linter["linter/"]
        db["db/"]
        models["models/"]
        error["error/"]
    end

    main --> auth
    main --> rooms
    main --> collab
    main --> linter
    auth --> db
    rooms --> db
    collab --> db
    linter --> db
    auth --> models
    rooms --> models
    collab --> models
    linter --> models
    auth --> error
    rooms --> error
    collab --> error
    linter --> error
```

### 2.2 Room Manager Shared State

The Room Manager is the core concurrency primitive. It holds the in-memory state for all active rooms, coordinates CRDT document updates, and broadcasts changes to connected WebSocket clients.

```mermaid
flowchart TD
    subgraph "Core Types"
        RM["RoomManager"]
        RS["RoomState"]
        CI["ClientInfo"]
        AE["ActivityEntry"]
    end

    subgraph "Key Methods"
        M1["createRoom"]
        M2["getRoom"]
        M3["addClient"]
        M4["removeClient"]
        M5["broadcastUpdate"]
        M6["saveSnapshots"]
    end

    RM --> RS
    RS --> CI
    RS --> AE
    RM --> M1
    RM --> M2
    RM --> M6
    RS --> M3
    RS --> M4
    RS --> M5
```

### 2.3 Concurrency Model

Each WebSocket connection gets its own Tokio task. The Room Manager uses Arc/Mutex to coordinate access to shared room state.

```mermaid
flowchart LR
    subgraph "Client Side"
        Browser["Client Browser"]
    end

    subgraph "Server Side"
        Axum["Axum Router"]
        Handler["WS Handler Task"]
        RM["Room Manager"]
        Doc["Yrs Document"]
        BC["Broadcast Engine"]
    end

    Browser -->|WebSocket upgrade| Axum
    Axum -->|"spawn task"| Handler
    Handler -->|"acquire lock"| RM
    RM -->|"room ref"| Handler
    Handler -->|"connected"| Browser

    Browser -->|"CRDT update"| Handler
    Handler -->|"acquire lock"| RM
    Handler -->|"apply update"| Doc
    Doc -->|"transformed update"| Handler
    Handler -->|"fan-out"| BC
```

### 2.4 WebSocket Message Protocol

```mermaid
flowchart LR
    subgraph ClientToServer
        A["YSyncUpdate"]
        B["CursorMove"]
        C["JoinRoom"]
    end
    subgraph ServerToClient
        D["YSyncState"]
        E["YSyncUpdateAck"]
        F["CursorBroadcast"]
        G["UserJoined"]
        H["ActivityUpdate"]
    end

    A --> E
    A --> D
    B --> F
    C --> G
```

## 3. Data Flow

### 3.1 Authentication Flow

```mermaid
flowchart TD
    subgraph "Registration"
        R1["POST /api/auth/register"]
        R2["argon2.hash password"]
        R3["INSERT users"]
        R4["generate session token"]
        R5["INSERT sessions"]
        R6["Set-Cookie session_token"]
        R7["navigate to home"]
    end

    subgraph "Login"
        L1["POST /api/auth/login"]
        L2["SELECT users by username"]
        L3["argon2.verify password"]
        L4["valid?"]
        L5["generate session token"]
        L6["INSERT sessions"]
        L7["Set-Cookie session_token"]
        L8["401 Unauthorized"]
    end

    subgraph "Logout"
        LO1["POST /api/auth/logout"]
        LO2["DELETE FROM sessions"]
        LO3["clear session cookie"]
    end

    R1 --> R2 --> R3 --> R4 --> R5 --> R6 --> R7
    L1 --> L2 --> L3 --> L4
    L4 -->|"yes"| L5 --> L6 --> L7
    L4 -->|"no"| L8
    LO1 --> LO2 --> LO3
```

### 3.2 Room Lifecycle

```mermaid
flowchart TD
    subgraph "Create Room"
        C1["POST /api/rooms"]
        C2["validate session"]
        C3["INSERT rooms"]
        C4["INSERT room_members"]
        C5["create in-memory room state"]
        C6["201 {roomId, roomCode}"]
    end

    subgraph "Get Room"
        G1["GET /api/rooms/:id"]
        G2["SELECT rooms JOIN users"]
        G3["200 {metadata, members}"]
    end

    subgraph "Join Room"
        J1["WebSocket /ws/:room_id"]
        J2["add client to room"]
        J3["return YState"]
    end

    C1 --> C2 --> C3 --> C4 --> C5 --> C6
    G1 --> G2 --> G3
    J1 --> J2 --> J3
```

### 3.3 Collaborative Edit Flow

```mermaid
flowchart LR
    subgraph "Alice"
        A1["YSyncUpdate: insert text"]
        A2["YSyncUpdate: insert at pos 5"]
    end

    subgraph "Server"
        S1["apply to Yrs document"]
        S2["compute diff update"]
        S3["fan-out to other clients"]
    end

    subgraph "Bob"
        B1["receive YSyncUpdateAck"]
        B2["YSyncState ack"]
    end

    A1 --> S1 --> S2 --> B1
    B1 --> B2 --> S2 --> S3 --> A2
    S3 --> B1
```

### 3.4 Linting Flow

```mermaid
flowchart TD
    subgraph "Request"
        L1["GET /api/rooms/:id/lint"]
        L2["check debounce state"]
        L3["debounce active?"]
    end

    subgraph "Cache Hit"
        L4["304 Not Modified"]
    end

    subgraph "Cache Miss"
        L5["spawn linter subprocess"]
        L6["parse stdout/stderr output"]
        L7["return diagnostics"]
    end

    L1 --> L2 --> L3
    L3 -->|"yes"| L4
    L3 -->|"no"| L5 --> L6 --> L7
```

## 4. Database Design

### 4.1 Schema

| Table | Columns | Notes |
|-------|---------|-------|
| `users` | `id PK`, `username UK`, `password_hash`, `created_at` | argon2id hash |
| `sessions` | `id PK`, `user_id FK`, `token UK`, `expires_at` | TTL-based expiry |
| `rooms` | `id PK`, `code UK`, `name`, `language`, `owner_id FK`, `created_at` | unique room code |
| `room_members` | `room_id FK`, `user_id FK`, `joined_at` | composite PK |
| `documents` | `id PK`, `room_id FK`, `content_snapshot BLOB`, `version`, `saved_at` | latest snapshot only |

```mermaid
flowchart TD
    USERS["users"]
    SESSIONS["sessions"]
    ROOMS["rooms"]
    ROOM_MEMBERS["room_members"]
    DOCUMENTS["documents"]

    USERS -->|"has"| SESSIONS
    USERS -->|"owns"| ROOMS
    USERS -->|"joins"| ROOM_MEMBERS
    ROOMS -->|"contains"| ROOM_MEMBERS
    ROOMS -->|"has"| DOCUMENTS
```

### 4.2 Migration Strategy

sqlx migrations in `migrations/` directory, versioned with timestamps.

- `001_create_users.sql` — users table
- `002_create_sessions.sql` — sessions table with TTL index
- `003_create_rooms.sql` — rooms, room_members tables
- `004_create_documents.sql` — documents table for snapshots

## 5. Frontend Architecture

### 5.1 Component Hierarchy

```mermaid
flowchart TD
    App["App Router"]

    subgraph "Public Routes"
        Login["LoginPage"]
        Register["RegisterPage"]
    end

    subgraph "Protected Routes"
        Home["HomePage"]
        Room["RoomPage"]
        RoomBrowser["RoomBrowser"]
        Editor["EditorPanel"]
        UserList["UserList"]
        ActivityFeed["ActivityFeed"]
        LintGutter["Lint Gutters"]
    end

    App --> Login
    App --> Register
    App --> Home
    App --> Room
    Home --> RoomBrowser
    Room --> Editor
    Room --> UserList
    Room --> ActivityFeed
    Room --> LintGutter
```

### 5.2 State Management

```mermaid
flowchart TD
    subgraph "Global State (React Context)"
        AuthCtx["AuthContext"]
        RoomCtx["RoomContext"]
    end

    subgraph "Editor State"
        YDoc["Y.Doc CRDT"]
        Provider["y-websocket Provider"]
        CMState["CodeMirror State"]
    end

    AuthCtx --> RoomCtx
    YDoc --> Provider --> CMState
```

### 5.3 Custom Hooks

| Hook | Responsibility |
|------|---------------|
| `useAuth` | Session management, protected route guards |
| `useWebSocket` | WebSocket lifecycle, reconnect logic, message framing |
| `useYDoc` | Yjs document creation, sync with server, initial load |
| `useLinting` | Debounced lint requests, diagnostic state |
| `useActivityFeed` | Room activity events, scroll management |

## 6. Security Design

```mermaid
flowchart TD
    subgraph "Auth Layer"
        Cookie["HttpOnly Secure Lax cookie"]
        Middleware["Auth middleware"]
    end

    subgraph "Authorization"
        RoomOwner["Room owner check"]
        RoomMember["Room member check"]
    end

    subgraph "Data Protection"
        Argon2["argon2 password hashing"]
        TTL["Session TTL + cleanup"]
    end

    Cookie --> Middleware
    Middleware --> RoomOwner
    Middleware --> RoomMember
    Argon2 --> Cookie
    TTL --> Cookie
```

Key decisions:
- Passwords hashed with argon2id (memory-hard, GPU-resistant)
- Sessions stored in SQLite with `expires_at`; cleanup on startup + periodic interval
- Room access gated by membership check (owner always allowed; others must be in `room_members`)
- WebSocket connections require valid session cookie before upgrade

## 7. Persistence Strategy

```mermaid
flowchart LR
    subgraph "In-Memory (fast path)"
        YDoc["Yrs document per room"]
        Broadcast["Broadcast engine"]
    end

    subgraph "SQLite (durability)"
        Snapshots["document snapshots"]
        Cleanup["Session cleanup"]
    end

    YDoc -.->|"periodic save"| Snapshots
    Snapshots -.->|"on room load"| YDoc
    Cleanup -.->|"startup + interval"| Snapshots
```

- **Save interval**: 30 seconds (configurable)
- **Save strategy**: Full document snapshot (Yrs state vector + content)
- **Restore**: On room join, load latest snapshot, apply any pending updates
- **Session cleanup**: Delete expired sessions on startup + every 5 minutes

## 8. Error Handling Strategy

```mermaid
flowchart TD
    subgraph "error/"
        AppErr["AppError enum"]
        AuthErr["AuthenticationError"]
        RoomErr["RoomError"]
        CollabErr["CollaborationError"]
        LintErr["LintError"]
        DbErr["DatabaseError"]
        IoErr["IoError"]
    end

    subgraph "Response"
        IntoResp["IntoResponse trait"]
    end

    AppErr --> AuthErr
    AppErr --> RoomErr
    AppErr --> CollabErr
    AppErr --> LintErr
    AppErr --> DbErr
    AppErr --> IoErr
    AppErr --> IntoResp
```

- Single `AppError` enum with `thiserror` derives
- Implements `IntoResponse` for Axum
- Categories: auth failures, room not found, WS protocol errors, lint failures, DB errors, I/O errors
- Debug mode returns detailed messages; release mode sanitizes

## 9. Configuration

```mermaid
flowchart TD
    Env["Environment variables"]
    Config["Config struct"]
    DB["Database URL"]
    Port["Server port"]
    Secret["Session secret"]
    Save["Snapshot interval"]

    Env --> Config
    Config --> DB
    Config --> Port
    Config --> Secret
    Config --> Save
```

- `.env` file (not committed, referenced in `.gitignore`)
- Config loaded at startup via `dotenv` + `serde`
- Required: `DATABASE_URL`, `SESSION_SECRET`, `PORT` (default 3000)
- Development defaults: `DATABASE_URL=sqlite://collab_editor.db`, `PORT=3000`, `SNAPSHOT_INTERVAL=30s`
