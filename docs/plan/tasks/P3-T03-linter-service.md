# P3-T03 — Linter Service

**Status:** TODO

## Goal

Create the lint service that connects LSP process with room collaboration.

## Description

Create `src/linter/service.rs` with the `LintService` that manages LSP processes per room and bridges LSP diagnostics to WebSocket room clients.

The `LintService` struct:
```rust
struct LintService {
    rooms: HashMap<RoomId, Arc<LspProcess>>,
}
```

Key methods:
- `spawn(room_id, language)` — start LSP process for room, send initialize + didOpen
- `update(room_id, content)` — send didChange to LSP, parse diagnostics, broadcast
- `shutdown(room_id)` — send didClose, kill LSP process, remove from map

The update flow:
1. Receive file content change from room WS message
2. Forward to LSP via `didChange` notification on LSP stdin
3. Parse diagnostics from LSP response (received via response channel)
4. Format diagnostics as `LintMessage` (line, column, severity, message)
5. Broadcast `LintMessage` to all room clients via WS

Diagnostics format:
```json
{
  "type": "lint",
  "diagnostics": [
    {
      "line": 5,
      "column": 10,
      "severity": "error",
      "message": "unexpected token"
    }
  ]
}
```

## Acceptance Criteria

- AC1: `LintService` struct with per-room LSP process map
- AC2: `spawn()` starts LSP, sends initialize + didOpen
- AC3: `update()` forwards content to LSP and broadcasts diagnostics
- AC4: `shutdown()` cleans up LSP process for a room
- AC5: LSP `didChange` sent on document content change
- AC6: LSP diagnostics parsed and formatted as `LintMessage`
- AC7: Diagnostics broadcast to all room clients via WS
- AC8: Room cleanup on room close/disconnect
- AC9: `cargo check` passes
- AC10: Proper error handling for LSP process failures

## Technical Hints

- Use `tokio::sync::mpsc` for LSP response channel per process
- Parse LSP diagnostics: they come as `textDocument/publishDiagnostics` notifications
- Diagnostic severity mapping: Error=1, Warning=2, Info=3, Hint=4
- Broadcast pattern: `room_state.broadcast(LintMessage(diagnostics), None).await`
- Use `Arc<LspProcess>` for sharing across async tasks
- Refer to: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_publishDiagnostics
