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

Phase 1 (Foundation) and Phase 2 (Collaboration) are complete. See [`docs/plan/`](docs/plan/) for the detailed task breakdown and UX designs.

---

## Prerequisites — Ubuntu 26.04 (Jammy)

### 1. System packages

```bash
sudo apt update
sudo apt install -y curl git pkg-config libssl-dev sqlite3
```

### 2. Rust toolchain (≥ 1.85)

```bash
# Install Rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Reload your shell
 source "$HOME/.cargo/env"

# Verify
 rustc --version  # ≥ 1.85
 cargo --version
```

### 3. Node.js 18+ and npm

```bash
# Option A — via NodeSource (recommended)
curl -fsSL https://deb.nodesource.com/setup_22.x | sudo -E bash -
sudo apt install -y nodejs

# Verify
 node --version  # ≥ 18
 npm --version
```

### 4. SQLite (usually pre-installed)

```bash
sqlite3 --version  # Verify
```

---

## Building

### Backend (Rust)

```bash
# From the project root:
cd collab-editor
cargo build --release
```

Binary is at `target/release/collab-editor`.

### Frontend (React + Vite)

```bash
cd collab-editor/frontend
npm install
npm run build
```

Output is at `frontend/dist/`.

---

## Running Locally

### 1. Start the backend

```bash
# Use SQLite database in the project root
cd collab-editor
DATABASE_URL="sqlite:collab.db" cargo run --release
```

The backend starts on **`http://localhost:3000`**.

### 2. Start the frontend dev server (development)

```bash
cd collab-editor/frontend
npm run dev
```

The frontend starts on **`http://localhost:5173`** and proxies API calls to the backend.

### 3. Serve the frontend in production

Build the frontend first (see Building), then:

```bash
# From project root
python3 -m http.server 8080 --directory frontend/dist
```

Access the app at **`http://localhost:8080`**. All `/api/*` requests will need a proxy or CORS setup.

---

## Running on a Local Network

### Backend

```bash
cd collab-editor
# The backend already binds to 0.0.0.0:3000 by default.
# Ensure your firewall allows port 3000:
sudo ufw allow 3000/tcp

DATABASE_URL="sqlite:collab.db" cargo run --release
```

Find your machine's local IP:

```bash
ip addr show  # Look for inet under eth0 or wlan0
# or
hostname -I
```

### Frontend (dev server)

```bash
cd collab-editor/frontend
npm run dev -- --host
# The --host flag binds to 0.0.0.0 instead of 127.0.0.1
```

The frontend will print the network URL (e.g., `http://192.168.1.10:5173`).

### Frontend (production)

After building, serve from a machine on your network:

```bash
python3 -m http.server 8080 --directory frontend/dist
```

Then access at `http://<IP>:8080`.

### Firewall considerations

| Port | Service    | Command                    |
|------|------------|----------------------------|
| 3000 | Backend    | `sudo ufw allow 3000/tcp` |
| 5173 | Frontend   | `sudo ufw allow 5173/tcp` |
| 8080 | Frontend   | `sudo ufw allow 8080/tcp` |

---

## Configuration

### Environment variables

| Variable        | Default            | Description                        |
|-----------------|--------------------|------------------------------------|
| `DATABASE_URL`  | `sqlite:collab.db` | SQLite database connection string   |

### CORS

The backend CORS layer (in `src/lib.rs`) allows these methods for development:
- `GET`, `POST`, `DELETE`, `OPTIONS`
- Content-Type: `application/json`

For production, restrict the `Allow-Origin` header to your frontend URL.

---

## Troubleshooting

### Rust build fails

```bash
# Ensure you have the latest Rust stable
 rustup update stable

# Clean and rebuild
 cargo clean && cargo build --release
```

### SQLite errors

Ensure `libsqlite3-dev` is installed:

```bash
sudo apt install -y libsqlite3-dev
```

### Frontend proxy errors

If API calls fail with CORS errors during development, check that:
- The backend is running on port 3000
- The Vite dev server `server.proxy` config points to `http://localhost:3000`
- Your firewall isn't blocking local connections

### WebSocket connection failures

- Verify the backend is running
- Check browser console for WebSocket upgrade errors
- Ensure the room exists (create via the UI first)
