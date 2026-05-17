# P3-T01 — LSP Server Process

**Status:** TODO

## Goal

Create the LSP server process spawning and communication infrastructure.

## Description

Create `src/linter/server.rs` with the `LspProcess` struct that manages an external LSP server (rust-analyzer, pyright, or eslint) as a child process. The LSP process communicates via JSON-RPC over stdin/stdout pipes.

The struct manages:
- `child: Option<Child>` — the spawned LSP process
- `stdin_writer: Option<tokio::io::WriteHalf>` — pipe to LSP stdin
- `stdout_reader: tokio::io::ReadHalf` — pipe from LSP stdout
- `request_id: u64` — monotonically increasing JSON-RPC request ID
- `response_tx: mpsc::Sender<LspResponse>` — channel for LSP responses

The `spawn(language: Language)` method:
1. Selects the correct LSP binary based on language
2. Spawns the process with stdin/stdout piped
3. Starts a read task for parsing LSP responses from stdout
4. Returns the `LspProcess` instance

Language to binary mapping:
- `Rust` → `rust-analyzer` (with `--stdio` flag)
- `Python` → `pyrightlang` (with `--stdio` flag)
- `JavaScript/TypeScript` → `eslint-lsp` or `typescript-language-server`

## Acceptance Criteria

- AC1: `LspProcess` struct defined with child process management
- AC2: `spawn(language)` method that selects and spawns correct LSP binary
- AC3: Process spawned with `--stdio` flag for stdio transport
- AC4: `stdin_writer` available for sending LSP requests
- AC5: `stdout_reader` available for receiving LSP responses
- AC6: Response parsing task started on stdout pipe
- AC7: `request_id` monotonically incremented for JSON-RPC correlation
- AC8: `cargo check` passes
- AC9: Proper error handling for process spawn failures

## Technical Hints

- Process spawning:
  ```rust
  let mut child = tokio::process::Command::new(binary)
      .args(&["--stdio"])
      .stdin(tokio::process::Command::piped())
      .stdout(tokio::process::Command::piped())
      .spawn()?;
  ```
- Use `tokio::io::AsyncReadExt` and `tokio::io::AsyncWriteExt` for pipe I/O
- LSP response parsing: use `serde_json` to parse JSON-RPC messages
- JSON-RPC envelope: `{"jsonrpc": "2.0", "id": N, "result": ..., "error": ...}`
- Refer to: https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#stdioTransports
- Refer to: https://docs.rs/tokio/latest/tokio/process/struct.Command.html
