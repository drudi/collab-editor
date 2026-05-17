# P3-T02 — LSP Protocol Messages

**Status:** TODO

## Goal

Implement LSP JSON-RPC message types and the initialize handshake.

## Description

Create `src/linter/protocol.rs` with LSP JSON-RPC message types and the protocol handshake logic. This module defines the types for all LSP messages and provides helper functions for creating valid LSP requests.

Message types:
- `LspRequest` — JSON-RPC request with id, method, params
- `LspResponse` — JSON-RPC response with id, result, error
- `LspNotification` — JSON-RPC notification (no response expected)

Key protocol messages:
- `initialize` → server capabilities
- `initialized` — notification that client is ready
- `textDocument/didOpen` — document opened in editor
- `textDocument/didChange` — document content changed
- `textDocument/didClose` — document closed

The `LspProcess::initialize()` method sends the initialize handshake:
```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "method": "initialize",
  "params": {
    "processId": <pid>,
    "rootUri": null,
    "capabilities": {}
  }
}
```

The `didOpen` message:
```json
{
  "jsonrpc": "2.0",
  "method": "textDocument/didOpen",
  "params": {
    "textDocument": {
      "uri": `file:///room/{roomId}`,
      "languageId": language,
      "version": 1,
      "text": content
    }
  }
}
```

## Acceptance Criteria

- AC1: `LspRequest`, `LspResponse`, `LspNotification` types defined
- AC2: `Serialize`/`Deserialize` derives on all message types
- AC3: `initialize()` method sends correct JSON-RPC initialize request
- AC4: `didOpen(content, language)` method sends correct didOpen notification
- AC5: `didChange(content)` method sends correct didChange notification
- AC6: `didClose()` method sends correct didClose notification
- AC7: JSON-RPC `id` correlation for matching responses to requests
- AC8: `cargo check` passes
- AC9: Proper serde field naming with `#[serde(rename = "...")]`

## Technical Hints

- JSON-RPC 2.0 spec: https://www.jsonrpc.org/specification
- Use `#[serde(rename = "jsonrpc")]` for JSON field names
- LSP capabilities: start with empty `{}` and extend later
- `textDocument/didChange` supports content changes via `contentChanges` array
- URI format: `file:///room/{roomId}` is sufficient (no actual file needed)
- Refer to: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/
