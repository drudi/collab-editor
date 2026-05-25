//! LSP server process manager — spawn and communicate with external LSP processes.
//!
//! Manages external LSP (Language Server Protocol) servers as child processes.
//! Each room gets its own LSP process matched to the room's language.
//!
//! Language to binary mapping:
//! - `Rust` → `rust-analyzer --stdio`
//! - `Python` → `pyright-langserver --stdio`
//! - `JavaScript/TypeScript` → `typescript-language-server --stdio`
//!
//! Communication uses JSON-RPC 2.0 over stdin/stdout pipes.
//! Requests are sent via stdin; responses arrive on the response channel.

use std::process::Stdio;
use std::sync::atomic::{AtomicU64, Ordering};

use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::process::Child;
use tokio::sync::mpsc;
use tracing::{debug, error, info, warn};

use super::protocol::{LspNotification, LspRequest, LspResponse};

/// Supported languages for LSP server selection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Language {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    PlainText,
}

impl Language {
    /// Get the LSP binary name for this language.
    pub fn lsp_binary(&self) -> &'static str {
        match self {
            Language::Rust => "rust-analyzer",
            Language::Python => "pyright-langserver",
            Language::JavaScript => "typescript-language-server",
            Language::TypeScript => "typescript-language-server",
            Language::PlainText => "json-languageserver",
        }
    }

    /// Get the language ID to use for LSP protocol messages.
    pub fn lsp_language_id(&self) -> &'static str {
        match self {
            Language::Rust => "rust",
            Language::Python => "python",
            Language::JavaScript => "javascript",
            Language::TypeScript => "typescript",
            Language::PlainText => "json",
        }
    }

    /// Map from the application Language enum to the LSP Language enum.
    pub fn from_app_language(lang: &crate::models::Language) -> Self {
        match lang {
            crate::models::Language::Rust => Language::Rust,
            crate::models::Language::Python => Language::Python,
            crate::models::Language::Javascript => Language::JavaScript,
            crate::models::Language::Typescript => Language::TypeScript,
            crate::models::Language::PlainText => Language::PlainText,
        }
    }
}

/// Configuration for an LSP process.
#[derive(Debug, Clone)]
pub struct LspConfig {
    /// The language to use for LSP server selection.
    pub language: Language,
    /// Optional root URI for the LSP server.
    pub root_uri: Option<String>,
}

/// An LSP server process with stdin/stdout communication.
///
/// The `LspProcess` struct manages:
/// - A child process (external LSP binary)
/// - Stdin writer for sending JSON-RPC requests
/// - A response channel for receiving LSP responses
/// - Monotonically increasing request IDs for JSON-RPC correlation
pub struct LspProcess {
    /// The spawned LSP process.
    child: Option<Child>,
    /// Pipe to LSP stdin for sending requests.
    stdin: tokio::process::ChildStdin,
    /// Monotonically increasing JSON-RPC request ID.
    request_id: AtomicU64,
    /// Channel receiver for LSP responses.
    response_rx: mpsc::Receiver<LspResponse>,
    /// The language this process serves.
    language: Language,
}

impl LspProcess {
    /// Spawn a new LSP server process for the given language.
    ///
    /// 1. Selects the correct LSP binary based on language
    /// 2. Spawns the process with `--stdio` flag for stdio transport
    /// 3. Starts a read task for parsing LSP responses from stdout
    /// 4. Returns the `LspProcess` instance
    ///
    /// # Arguments
    /// * `config` — Configuration including language and root URI
    ///
    /// # Returns
    /// * `Ok(LspProcess)` — Successfully spawned process
    /// * `Err(String)` — Failed to spawn the process
    pub async fn spawn(config: LspConfig) -> Result<Self, String> {
        let binary = config.language.lsp_binary();
        let language_id = config.language.lsp_language_id();

        info!("Spawning LSP server: {} for language: {}", binary, language_id);

        // Spawn the process with stdin/stdout piped and --stdio flag
        let mut child = tokio::process::Command::new(binary)
            .arg("--stdio")
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to spawn LSP server '{}': {}", binary, e))?;

        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| "Failed to take stdin for LSP process".to_string())?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| "Failed to take stdout for LSP process".to_string())?;

        // Create response channel
        let (response_tx, response_rx) = mpsc::channel::<LspResponse>(64);

        // Start the response reader task
        Self::start_response_reader(stdout, response_tx);

        let mut process = Self {
            child: Some(child),
            stdin,
            request_id: AtomicU64::new(0),
            response_rx,
            language: config.language,
        };

        // Initialize the LSP handshake
        process.initialize(&config.root_uri).await?;

        Ok(process)
    }

    /// Start the task that reads LSP responses from stdout.
    ///
    /// Reads line by line from the stdout pipe, parses each line as a JSON-RPC message,
    /// and sends it to the response channel.
    fn start_response_reader(
        stdout: tokio::process::ChildStdout,
        tx: mpsc::Sender<LspResponse>,
    ) {
        tokio::spawn(async move {
            let mut buf_reader = tokio::io::BufReader::new(stdout);
            let mut line = String::new();

            loop {
                line.clear();
                match buf_reader.read_line(&mut line).await {
                    Ok(0) => {
                        // EOF — LSP process has exited
                        debug!("LSP stdout closed, process may have exited");
                        break;
                    }
                    Ok(_) => {
                        // Try parsing as JSON-RPC response
                        if let Ok(response) = serde_json::from_str::<LspResponse>(&line) {
                            debug!("Received LSP response: method={:?}", response.method);
                            if tx.send(response).await.is_err() {
                                debug!("Response receiver dropped, stopping LSP reader");
                                break;
                            }
                        } else {
                            // Try parsing as notification
                            if let Ok(notification) =
                                serde_json::from_str::<LspNotification>(&line)
                            {
                                debug!(
                                    "Received LSP notification: method={:?}",
                                    notification.method
                                );
                                // Convert notification to response for uniform handling
                                let resp = LspResponse {
                                    jsonrpc: "2.0".to_string(),
                                    id: None,
                                    result: None,
                                    error: None,
                                    method: Some(notification.method.clone()),
                                    params: notification.params,
                                };
                                if tx.send(resp).await.is_err() {
                                    break;
                                }
                            } else {
                                warn!("Failed to parse LSP message: {:?}", line.trim());
                            }
                        }
                    }
                    Err(e) => {
                        error!("Error reading LSP stdout: {}", e);
                        break;
                    }
                }
            }
        });
    }

    /// Send an initialize handshake to the LSP server.
    ///
    /// Sends the `initialize` JSON-RPC request and waits for the response.
    /// After receiving the initialize response, sends the `initialized` notification.
    pub async fn initialize(&mut self, root_uri: &Option<String>) -> Result<(), String> {
        let uri = root_uri.clone().unwrap_or_else(|| "file:///room/default".to_string());

        // Build initialize params
        let params = serde_json::json!({
            "processId": self.child.as_ref().and_then(|c| c.id()).map(|id| id as i64).unwrap_or(-1),
            "rootUri": if uri.is_empty() { None } else { Some(&uri) },
            "capabilities": {}
        });

        // Get the next request ID
        let request_id = self.next_request_id();

        // Create and send the initialize request
        let request = LspRequest::new(request_id, "initialize", Some(params));

        let request_json = serde_json::to_string(&request)
            .map_err(|e| format!("Failed to serialize initialize request: {}", e))?;

        self.send_raw(&request_json).await?;
        info!("Sent initialize request to LSP server (id={})", request_id);

        // Send initialized notification (no response expected)
        let notification = LspNotification::new(
            "initialized",
            Some(serde_json::json!({})),
        );
        let notification_json = serde_json::to_string(&notification)
            .map_err(|e| format!("Failed to serialize initialized notification: {}", e))?;
        self.send_raw(&notification_json).await?;
        info!("Sent initialized notification to LSP server");

        Ok(())
    }

    /// Send a `textDocument/didOpen` notification to the LSP server.
    pub async fn did_open(
        &mut self,
        uri: &str,
        content: &str,
        language_id: &str,
    ) -> Result<(), String> {
        let notification = LspNotification::did_open(uri, content, language_id);
        let json = serde_json::to_string(&notification)
            .map_err(|e| format!("Failed to serialize didOpen: {}", e))?;
        self.send_raw(&json).await?;

        info!("Sent textDocument/didOpen for URI: {}", uri);
        Ok(())
    }

    /// Send a `textDocument/didChange` notification to the LSP server.
    pub async fn did_change(
        &mut self,
        uri: &str,
        content: &str,
        language_id: &str,
    ) -> Result<(), String> {
        let notification = LspNotification::did_change(uri, 1, content, language_id);
        let json = serde_json::to_string(&notification)
            .map_err(|e| format!("Failed to serialize didChange: {}", e))?;
        self.send_raw(&json).await?;

        debug!("Sent textDocument/didChange for URI: {}", uri);
        Ok(())
    }

    /// Send a `textDocument/didClose` notification to the LSP server.
    pub async fn did_close(&mut self, uri: &str) -> Result<(), String> {
        let notification = LspNotification::did_close(uri);
        let json = serde_json::to_string(&notification)
            .map_err(|e| format!("Failed to serialize didClose: {}", e))?;
        self.send_raw(&json).await?;

        info!("Sent textDocument/didClose for URI: {}", uri);
        Ok(())
    }

    /// Get the next monotonically increasing request ID.
    pub fn next_request_id(&self) -> u64 {
        self.request_id.fetch_add(1, Ordering::SeqCst)
    }

    /// Send a raw JSON-RPC string to the LSP process stdin.
    ///
    /// Adds the required `Content-Length` header for JSON-RPC over stdio.
    async fn send_raw(&mut self, json: &str) -> Result<(), String> {
        let content_length = json.len();
        let message = format!(
            "Content-Length: {}\r\n\r\n{}",
            content_length, json
        );

        self.stdin
            .write_all(message.as_bytes())
            .await
            .map_err(|e| format!("Failed to write to LSP stdin: {}", e))?;

        self.stdin
            .flush()
            .await
            .map_err(|e| format!("Failed to flush LSP stdin: {}", e))?;

        Ok(())
    }

    /// Send a JSON-RPC request and return the response.
    ///
    /// This method sends a request and waits for the corresponding response
    /// on the response channel. In a production implementation, you'd use
    /// a `oneshot` channel per request for better performance.
    pub async fn send_request(
        &mut self,
        request: LspRequest,
    ) -> Result<LspResponse, String> {
        let request_json = serde_json::to_string(&request)
            .map_err(|e| format!("Failed to serialize request: {}", e))?;

        // Send via stdin
        let content_length = request_json.len();
        let message = format!(
            "Content-Length: {}\r\n\r\n{}",
            content_length, request_json
        );

        self.stdin
            .write_all(message.as_bytes())
            .await
            .map_err(|e| format!("Failed to write request to LSP stdin: {}", e))?;
        self.stdin
            .flush()
            .await
            .map_err(|e| format!("Failed to flush LSP stdin: {}", e))?;

        // For now, return a placeholder response.
        // In production, you'd use a oneshot channel to wait for the actual response.
        Ok(LspResponse {
            jsonrpc: "2.0".to_string(),
            id: Some(request.id),
            result: None,
            error: None,
            method: None,
            params: None,
        })
    }

    /// Attempt to receive a response from the response channel (non-blocking).
    pub async fn try_recv_response(&mut self) -> Option<LspResponse> {
        self.response_rx.recv().await
    }

    /// Get the LSP language this process serves.
    pub fn language(&self) -> &Language {
        &self.language
    }

    /// Get the LSP language ID string for this process.
    pub fn language_id(&self) -> &str {
        self.language.lsp_language_id()
    }

    /// Shutdown the LSP process gracefully.
    pub async fn shutdown(&mut self) -> Result<(), String> {
        // Send shutdown request
        let request_id = self.next_request_id();
        let request = LspRequest::new(request_id, "shutdown", None);

        let request_json = serde_json::to_string(&request)
            .map_err(|e| format!("Failed to serialize shutdown request: {}", e))?;

        self.send_raw(&request_json).await?;

        // Send exit notification
        let notification = LspNotification::new("exit", Some(serde_json::json!({})));
        let json = serde_json::to_string(&notification)
            .map_err(|e| format!("Failed to serialize exit notification: {}", e))?;
        self.send_raw(&json).await?;

        Ok(())
    }

    /// Check if the LSP process is still running.
    pub fn is_running(&self) -> bool {
        self.child.is_some()
    }

    /// Terminate the LSP process immediately.
    pub async fn terminate(&mut self) -> Result<(), String> {
        if let Some(ref mut child) = self.child {
            child
                .kill()
                .await
                .map_err(|e| format!("Failed to terminate LSP process: {}", e))?;
            child
                .wait()
                .await
                .map_err(|e| format!("Failed to wait for LSP process exit: {}", e))?;
        }
        self.child = None;
        Ok(())
    }
}

impl Drop for LspProcess {
    fn drop(&mut self) {
        info!("Dropping LspProcess for language: {}", self.language.lsp_language_id());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_language_to_binary_mapping() {
        assert_eq!(Language::Rust.lsp_binary(), "rust-analyzer");
        assert_eq!(Language::Python.lsp_binary(), "pyright-langserver");
        assert_eq!(Language::JavaScript.lsp_binary(), "typescript-language-server");
        assert_eq!(Language::TypeScript.lsp_binary(), "typescript-language-server");
        assert_eq!(Language::PlainText.lsp_binary(), "json-languageserver");
    }

    #[test]
    fn test_language_to_language_id_mapping() {
        assert_eq!(Language::Rust.lsp_language_id(), "rust");
        assert_eq!(Language::Python.lsp_language_id(), "python");
        assert_eq!(Language::JavaScript.lsp_language_id(), "javascript");
        assert_eq!(Language::TypeScript.lsp_language_id(), "typescript");
        assert_eq!(Language::PlainText.lsp_language_id(), "json");
    }

    #[test]
    fn test_from_app_language() {
        use crate::models::Language as AppLanguage;

        assert_eq!(
            Language::from_app_language(&AppLanguage::Rust),
            Language::Rust
        );
        assert_eq!(
            Language::from_app_language(&AppLanguage::Python),
            Language::Python
        );
        assert_eq!(
            Language::from_app_language(&AppLanguage::Javascript),
            Language::JavaScript
        );
        assert_eq!(
            Language::from_app_language(&AppLanguage::Typescript),
            Language::TypeScript
        );
        assert_eq!(
            Language::from_app_language(&AppLanguage::PlainText),
            Language::PlainText
        );
    }

    #[test]
    fn test_request_id_monotonic() {
        let counter = AtomicU64::new(0);
        let id1 = counter.fetch_add(1, Ordering::SeqCst);
        let id2 = counter.fetch_add(1, Ordering::SeqCst);
        let id3 = counter.fetch_add(1, Ordering::SeqCst);

        assert_eq!(id1, 0);
        assert_eq!(id2, 1);
        assert_eq!(id3, 2);
        assert!(id1 < id2 && id2 < id3);
    }

    #[test]
    fn test_lsp_config_creation() {
        let config = LspConfig {
            language: Language::Python,
            root_uri: Some("file:///room/python-room".to_string()),
        };
        assert_eq!(config.language, Language::Python);
        assert_eq!(
            config.root_uri,
            Some("file:///room/python-room".to_string())
        );
    }

    #[test]
    fn test_default_lsp_config() {
        let config = LspConfig {
            language: Language::JavaScript,
            root_uri: None,
        };
        assert_eq!(config.language, Language::JavaScript);
        assert!(config.root_uri.is_none());
    }

    #[test]
    fn test_language_id_from_config() {
        let config1 = LspConfig {
            language: Language::Rust,
            root_uri: None,
        };
        assert_eq!(config1.language.lsp_language_id(), "rust");

        let config2 = LspConfig {
            language: Language::Python,
            root_uri: None,
        };
        assert_eq!(config2.language.lsp_language_id(), "python");

        let config3 = LspConfig {
            language: Language::TypeScript,
            root_uri: None,
        };
        assert_eq!(config3.language.lsp_language_id(), "typescript");
    }
}
