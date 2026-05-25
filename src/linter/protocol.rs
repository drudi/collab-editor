//! LSP protocol types — JSON-RPC message envelope and method definitions.
//!
//! Implements the JSON-RPC 2.0 message types required for LSP communication:
//! - `LspRequest` — Request with id, method, params
//! - `LspResponse` — Response with id, result, error
//! - `LspNotification` — Notification (no response expected)
//!
//! Also provides helper functions for creating LSP protocol messages:
//! - `initialize()` — Server initialization handshake
//! - `did_open()` — Document opened in editor
//! - `did_change()` — Document content changed
//! - `did_close()` — Document closed

use serde::{Deserialize, Serialize};

/// JSON-RPC 2.0 request envelope.
///
/// Sent to the LSP server to initiate operations. Contains a unique
/// request `id` for correlating the response back to this request.
///
/// Example:
/// ```json
/// {
///   "jsonrpc": "2.0",
///   "id": 1,
///   "method": "initialize",
///   "params": { ... }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspRequest {
    /// JSON-RPC protocol version (always "2.0").
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// Unique request identifier for correlating request/response pairs.
    /// This must match the `id` field in the corresponding response.
    pub id: u64,

    /// Method name (e.g., "initialize", "textDocument/didOpen").
    pub method: String,

    /// Request parameters (method-specific).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

impl LspRequest {
    /// Create a new LSP request with the given ID, method, and optional params.
    pub fn new(id: u64, method: impl Into<String>, params: Option<serde_json::Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            id,
            method: method.into(),
            params,
        }
    }

    /// Create an initialize request for LSP server handshake.
    ///
    /// Returns a request with the standard `initialize` parameters:
    /// - `processId`: PID of the client process
    /// - `rootUri`: Workspace root URI
    /// - `capabilities`: Client capabilities
    pub fn initialize_request(
        request_id: u64,
        process_id: u32,
        root_uri: &Option<String>,
    ) -> Self {
        let params = serde_json::json!({
            "processId": if process_id > 0 { Some(process_id as i64) } else { None },
            "rootUri": root_uri.as_ref(),
            "capabilities": {}
        });

        Self {
            jsonrpc: "2.0".to_string(),
            id: request_id,
            method: "initialize".to_string(),
            params: Some(params),
        }
    }
}

/// JSON-RPC 2.0 response envelope.
///
/// Sent by the LSP server in response to a request. Contains the same
/// `id` as the request for correlation, plus either a `result` or `error`.
///
/// Example (success):
/// ```json
/// {
///   "jsonrpc": "2.0",
///   "id": 1,
///   "result": { "capabilities": { ... } }
/// }
/// ```
///
/// Example (error):
/// ```json
/// {
///   "jsonrpc": "2.0",
///   "id": 1,
///   "error": { "code": -32601, "message": "Method not found" }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspResponse {
    /// JSON-RPC protocol version (always "2.0").
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// Request identifier (same as the request's `id`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<u64>,

    /// Response result (present on success).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,

    /// Response error (present on failure).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<LspError>,

    /// Notification method (when response carries a notification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,

    /// Notification parameters (when response carries a notification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

impl LspResponse {
    /// Check if this response is successful (has a result, not an error).
    pub fn is_ok(&self) -> bool {
        self.result.is_some() && self.error.is_none()
    }

    /// Check if this response contains an error.
    pub fn is_error(&self) -> bool {
        self.error.is_some()
    }

    /// Get the error detail if this is an error response.
    pub fn error_detail(&self) -> Option<&LspError> {
        self.error.as_ref()
    }
}

/// LSP error detail.
///
/// Contains a numeric error code and human-readable message.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspError {
    /// Numeric error code (standard JSON-RPC or LSP-specific codes).
    pub code: i32,

    /// Human-readable error message.
    pub message: String,

    /// Optional error data (method-specific error details).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// JSON-RPC 2.0 notification envelope.
///
/// A notification is a request that doesn't expect a response.
/// It contains a `method` and `params` but no `id`.
///
/// Example:
/// ```json
/// {
///   "jsonrpc": "2.0",
///   "method": "textDocument/didOpen",
///   "params": { "textDocument": { ... } }
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspNotification {
    /// JSON-RPC protocol version (always "2.0").
    #[serde(rename = "jsonrpc")]
    pub jsonrpc: String,

    /// Notification method name (e.g., "textDocument/didOpen").
    pub method: String,

    /// Notification parameters (method-specific).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

impl LspNotification {
    /// Create a new LSP notification with the given method and optional params.
    pub fn new(method: impl Into<String>, params: Option<serde_json::Value>) -> Self {
        Self {
            jsonrpc: "2.0".to_string(),
            method: method.into(),
            params,
        }
    }

    /// Create a `textDocument/didOpen` notification.
    ///
    /// # Arguments
    /// * `uri` — Document URI (e.g., `file:///room/{room_id}`)
    /// * `content` — Document content
    /// * `language_id` — Language identifier (e.g., "rust", "python")
    pub fn did_open(
        uri: &str,
        content: &str,
        language_id: &str,
    ) -> Self {
        let text_document = serde_json::json!({
            "uri": uri,
            "languageId": language_id,
            "version": 1,
            "text": content
        });

        Self {
            jsonrpc: "2.0".to_string(),
            method: "textDocument/didOpen".to_string(),
            params: Some(serde_json::json!({
                "textDocument": text_document
            })),
        }
    }

    /// Create a `textDocument/didChange` notification.
    ///
    /// # Arguments
    /// * `uri` — Document URI
    /// * `version` — Document version number
    /// * `content` — New document content
    /// * `language_id` — Language identifier
    pub fn did_change(
        uri: &str,
        version: u32,
        content: &str,
        language_id: &str,
    ) -> Self {
        let text_document = serde_json::json!({
            "uri": uri,
            "languageId": language_id,
            "version": version
        });

        let content_changes = serde_json::json!([
            { "text": content }
        ]);

        Self {
            jsonrpc: "2.0".to_string(),
            method: "textDocument/didChange".to_string(),
            params: Some(serde_json::json!({
                "textDocument": text_document,
                "contentChanges": content_changes
            })),
        }
    }

    /// Create a `textDocument/didChange` notification with an incremental text edit.
    ///
    /// # Arguments
    /// * `uri` — Document URI
    /// * `version` — Document version number
    /// * `range` — Text range to replace
    /// * `new_text` — New text to insert
    pub fn did_change_text_edits(
        uri: &str,
        version: u32,
        range: serde_json::Value,
        new_text: &str,
    ) -> Self {
        let text_document = serde_json::json!({
            "uri": uri,
            "version": version
        });

        let content_changes = serde_json::json!([
            {
                "range": range,
                "text": new_text
            }
        ]);

        Self {
            jsonrpc: "2.0".to_string(),
            method: "textDocument/didChange".to_string(),
            params: Some(serde_json::json!({
                "textDocument": text_document,
                "contentChanges": content_changes
            })),
        }
    }

    /// Create a `textDocument/didClose` notification.
    ///
    /// # Arguments
    /// * `uri` — Document URI
    pub fn did_close(uri: &str) -> Self {
        let text_document = serde_json::json!({
            "uri": uri,
            "languageId": "",
            "version": 0,
            "text": ""
        });

        Self {
            jsonrpc: "2.0".to_string(),
            method: "textDocument/didClose".to_string(),
            params: Some(serde_json::json!({
                "textDocument": text_document
            })),
        }
    }
}

/// LSP diagnostic severity codes matching the LSP specification.
pub mod diagnostic_severity {
    /// Error severity code (LSP value: 1).
    pub const ERROR: i32 = 1;
    /// Warning severity code (LSP value: 2).
    pub const WARNING: i32 = 2;
    /// Information severity code (LSP value: 3).
    pub const INFO: i32 = 3;
    /// Hint severity code (LSP value: 4).
    pub const HINT: i32 = 4;
}

/// Parse LSP diagnostics from a `textDocument/publishDiagnostics` notification.
///
/// Extracts diagnostics from the JSON parameters of the notification.
/// Each diagnostic contains:
/// - `range`: { start: { line, character }, end: { line, character } }
/// - `severity`: 1=Error, 2=Warning, 3=Info, 4=Hint
/// - `message`: Human-readable diagnostic message
///
/// Returns a list of diagnostic objects with parsed fields.
pub fn parse_lsp_diagnostics(params: &serde_json::Value) -> Vec<LspDiagnostic> {
    let mut diagnostics = Vec::new();

    // Extract diagnostics array from params
    if let Some(diagnostics_array) = params.get("diagnostics").and_then(|d| d.as_array()) {
        for diag_value in diagnostics_array {
            if let Some(diag) = parse_single_diagnostic(diag_value) {
                diagnostics.push(diag);
            }
        }
    }

    diagnostics
}

/// Parse a single LSP diagnostic value.
fn parse_single_diagnostic(value: &serde_json::Value) -> Option<LspDiagnostic> {
    let range = value.get("range")?;

    let start_line = range.get("start")?.get("line")?.as_i64()? as i32;
    let start_char = range.get("start")?.get("character")?.as_i64()? as i32;
    let end_line = range.get("end")?.get("line")?.as_i64()? as i32;
    let end_char = range.get("end")?.get("character")?.as_i64()? as i32;

    let severity = value
        .get("severity")
        .and_then(|s| s.as_i64())
        .unwrap_or(1) as i32;

    let message = value
        .get("message")
        .and_then(|m| m.as_str())
        .unwrap_or("")
        .to_string();

    Some(LspDiagnostic {
        line: start_line,
        column: start_char,
        end_line,
        end_column: end_char,
        severity: severity,
        message,
    })
}

/// LSP diagnostic as parsed from server response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LspDiagnostic {
    /// Start line number (0-indexed).
    pub line: i32,
    /// Start column number (0-indexed).
    pub column: i32,
    /// End line number (0-indexed).
    pub end_line: i32,
    /// End column number (0-indexed).
    pub end_column: i32,
    /// Diagnostic severity (1=Error, 2=Warning, 3=Info, 4=Hint).
    pub severity: i32,
    /// Diagnostic message text.
    pub message: String,
}

impl LspDiagnostic {
    /// Convert LSP severity code to the application's severity enum.
    pub fn severity(&self) -> crate::collaboration::messages::LspDiagnosticSeverity {
        match self.severity {
            diagnostic_severity::ERROR => crate::collaboration::messages::LspDiagnosticSeverity::Error,
            diagnostic_severity::WARNING => crate::collaboration::messages::LspDiagnosticSeverity::Warning,
            diagnostic_severity::INFO => crate::collaboration::messages::LspDiagnosticSeverity::Info,
            diagnostic_severity::HINT => crate::collaboration::messages::LspDiagnosticSeverity::Hint,
            _ => crate::collaboration::messages::LspDiagnosticSeverity::Error,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsp_request_creation() {
        let request = LspRequest::new(
            1,
            "initialize",
            Some(serde_json::json!({"capabilities": {}})),
        );

        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.id, 1);
        assert_eq!(request.method, "initialize");
        assert!(request.params.is_some());
    }

    #[test]
    fn test_lsp_request_initialization() {
        let request = LspRequest::initialize_request(1, 12345, &Some("file:///room/test".to_string()));

        assert_eq!(request.method, "initialize");
        assert_eq!(request.id, 1);

        let params = request.params.unwrap();
        assert_eq!(params["processId"], 12345);
        assert_eq!(params["rootUri"], "file:///room/test");
        assert!(params["capabilities"].is_object());
    }

    #[test]
    fn test_lsp_response_is_ok() {
        let ok_response = LspResponse {
            jsonrpc: "2.0".to_string(),
            id: Some(1),
            result: Some(serde_json::json!({"capabilities": {}})),
            error: None,
            method: None,
            params: None,
        };
        assert!(ok_response.is_ok());
        assert!(!ok_response.is_error());

        let error_response = LspResponse {
            jsonrpc: "2.0".to_string(),
            id: Some(1),
            result: None,
            error: Some(LspError {
                code: -32601,
                message: "Method not found".to_string(),
                data: None,
            }),
            method: None,
            params: None,
        };
        assert!(!error_response.is_ok());
        assert!(error_response.is_error());
    }

    #[test]
    fn test_lsp_notification_did_open() {
        let notification = LspNotification::did_open(
            "file:///room/test-room",
            "let x = 1;",
            "javascript",
        );

        assert_eq!(notification.method, "textDocument/didOpen");
        assert!(notification.params.is_some());

        let params = notification.params.unwrap();
        let text_doc = params["textDocument"].clone();
        assert_eq!(text_doc["uri"], "file:///room/test-room");
        assert_eq!(text_doc["languageId"], "javascript");
        assert_eq!(text_doc["version"], 1);
        assert_eq!(text_doc["text"], "let x = 1;");
    }

    #[test]
    fn test_lsp_notification_did_change() {
        let notification = LspNotification::did_change(
            "file:///room/test-room",
            2,
            "let x = 2;\nlet y = 3;",
            "javascript",
        );

        assert_eq!(notification.method, "textDocument/didChange");
        let params = notification.params.unwrap();
        let text_doc = params["textDocument"].clone();
        assert_eq!(text_doc["version"], 2);

        let content_changes = params["contentChanges"].clone();
        assert_eq!(content_changes[0]["text"], "let x = 2;\nlet y = 3;");
    }

    #[test]
    fn test_lsp_notification_did_close() {
        let notification = LspNotification::did_close("file:///room/test-room");

        assert_eq!(notification.method, "textDocument/didClose");
        let params = notification.params.unwrap();
        assert_eq!(params["textDocument"]["uri"], "file:///room/test-room");
    }

    #[test]
    fn test_parse_lsp_diagnostics() {
        let params = serde_json::json!({
            "uri": "file:///room/test-room",
            "diagnostics": [
                {
                    "range": {
                        "start": { "line": 5, "character": 10 },
                        "end": { "line": 5, "character": 15 }
                    },
                    "severity": 1,
                    "message": "unexpected token"
                },
                {
                    "range": {
                        "start": { "line": 10, "character": 0 },
                        "end": { "line": 10, "character": 5 }
                    },
                    "severity": 2,
                    "message": "unused variable"
                }
            ]
        });

        let diagnostics = parse_lsp_diagnostics(&params);
        assert_eq!(diagnostics.len(), 2);

        assert_eq!(diagnostics[0].line, 5);
        assert_eq!(diagnostics[0].column, 10);
        assert_eq!(diagnostics[0].severity, diagnostic_severity::ERROR);
        assert_eq!(diagnostics[0].message, "unexpected token");

        assert_eq!(diagnostics[1].line, 10);
        assert_eq!(diagnostics[1].column, 0);
        assert_eq!(diagnostics[1].severity, diagnostic_severity::WARNING);
        assert_eq!(diagnostics[1].message, "unused variable");
    }

    #[test]
    fn test_parse_lsp_diagnostics_with_missing_severity() {
        let params = serde_json::json!({
            "diagnostics": [
                {
                    "range": {
                        "start": { "line": 0, "character": 0 },
                        "end": { "line": 0, "character": 5 }
                    },
                    "message": "missing severity defaults to error"
                }
            ]
        });

        let diagnostics = parse_lsp_diagnostics(&params);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].severity, 1); // defaults to ERROR
    }

    #[test]
    fn test_diagnostic_severity_conversion() {
        assert_eq!(
            LspDiagnostic {
                line: 0, column: 0, end_line: 0, end_column: 5,
                severity: diagnostic_severity::ERROR,
                message: "error".to_string(),
            }
            .severity(),
            crate::collaboration::messages::LspDiagnosticSeverity::Error
        );

        assert_eq!(
            LspDiagnostic {
                line: 0, column: 0, end_line: 0, end_column: 5,
                severity: diagnostic_severity::WARNING,
                message: "warning".to_string(),
            }
            .severity(),
            crate::collaboration::messages::LspDiagnosticSeverity::Warning
        );

        assert_eq!(
            LspDiagnostic {
                line: 0, column: 0, end_line: 0, end_column: 5,
                severity: diagnostic_severity::INFO,
                message: "info".to_string(),
            }
            .severity(),
            crate::collaboration::messages::LspDiagnosticSeverity::Info
        );

        assert_eq!(
            LspDiagnostic {
                line: 0, column: 0, end_line: 0, end_column: 5,
                severity: diagnostic_severity::HINT,
                message: "hint".to_string(),
            }
            .severity(),
            crate::collaboration::messages::LspDiagnosticSeverity::Hint
        );
    }

    #[test]
    fn test_lsp_request_serialization() {
        let request = LspRequest::initialize_request(1, 0, &None);
        let json = serde_json::to_string(&request).unwrap();

        assert!(json.contains("\"jsonrpc\":\"2.0\""));
        assert!(json.contains("\"id\":1"));
        assert!(json.contains("\"method\":\"initialize\""));
        assert!(json.contains("\"capabilities\""));
    }

    #[test]
    fn test_lsp_notification_serialization() {
        let notification = LspNotification::did_open(
            "file:///room/test",
            "content",
            "javascript",
        );
        let json = serde_json::to_string(&notification).unwrap();

        assert!(json.contains("\"jsonrpc\":\"2.0\""));
        assert!(json.contains("\"method\":\"textDocument/didOpen\""));
        assert!(!json.contains("\"id\"")); // Notifications have no id
    }
}
