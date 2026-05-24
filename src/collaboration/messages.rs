//! WebSocket message protocol — types and encoding for room collaboration.
//!
//! Defines the `WsMessage` enum and helpers for encoding/decoding messages
//! between server and clients.
//!
//! Message types:
//! - `Sync` — Yjs document update bytes (for CRDT sync)
//! - `Awareness` — awareness state update bytes (cursor/selection)
//! - `Cursor` — cursor position update
//! - `Ping` — health check ping
//! - `Pong` — health check pong

use serde::{Deserialize, Serialize};

/// WebSocket message types sent between server and clients.
///
/// All variants are serialized as JSON with a `type` discriminator field.
/// Binary data (Sync, Awareness) is stored as an array of bytes.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    /// Yjs document update (binary encoded as array of bytes).
    /// Used for both full sync on connect and incremental sync during editing.
    #[serde(rename = "sync")]
    Sync {
        #[serde(rename = "data")]
        data: Vec<u8>,
    },

    /// Awareness/state update (binary encoded as array of bytes).
    /// Contains cursor positions, selections, and other peer state.
    #[serde(rename = "awareness")]
    Awareness {
        #[serde(rename = "data")]
        data: Vec<u8>,
    },

    /// Cursor position update.
    /// Contains client ID, username, and cursor position.
    Cursor {
        #[serde(rename = "user_id")]
        user_id: String,
        username: String,
        cursor: CursorPos,
        #[serde(skip_serializing_if = "Option::is_none")]
        selection: Option<SelectionRange>,
    },

    /// Health check ping.
    Ping,

    /// Health check pong.
    Pong,
}

/// Cursor position in the document.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CursorPos {
    pub line: i32,
    pub ch: i32,
}

/// Text selection range.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectionRange {
    pub from: CursorPos,
    pub to: CursorPos,
}

impl WsMessage {
    /// Serialize the message as a JSON string.
    ///
    /// Binary data is base64-encoded for JSON compatibility.
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }

    /// Deserialize a JSON string into a WsMessage.
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize binary data as base64 string.
    pub fn encode_base64(data: &[u8]) -> String {
        base64_engine::encode(data)
    }

    /// Deserialize base64 string as binary data.
    pub fn decode_base64(encoded: &str) -> Result<Vec<u8>, base64_engine::DecodeError> {
        base64_engine::decode(encoded.as_bytes())
    }

    /// Create a Sync message from binary data.
    pub fn sync(data: Vec<u8>) -> Self {
        WsMessage::Sync { data }
    }

    /// Create an Awareness message from binary data.
    pub fn awareness(data: Vec<u8>) -> Self {
        WsMessage::Awareness { data }
    }

    /// Create a Cursor message with position.
    pub fn cursor(
        user_id: impl Into<String>,
        username: impl Into<String>,
        cursor: CursorPos,
        selection: Option<SelectionRange>,
    ) -> Self {
        WsMessage::Cursor {
            user_id: user_id.into(),
            username: username.into(),
            cursor,
            selection,
        }
    }

    /// Check if this is a ping message.
    pub fn is_ping(&self) -> bool {
        matches!(self, WsMessage::Ping)
    }

    /// Check if this is a pong message.
    pub fn is_pong(&self) -> bool {
        matches!(self, WsMessage::Pong)
    }

    /// Check if this is a sync message.
    pub fn is_sync(&self) -> bool {
        matches!(self, WsMessage::Sync { .. })
    }

    /// Check if this is an awareness message.
    pub fn is_awareness(&self) -> bool {
        matches!(self, WsMessage::Awareness { .. })
    }
}

/// Base64 engine for encoding/decoding binary data in JSON.
mod base64_engine {
    use std::fmt;

    /// Encode bytes to base64 string.
    pub fn encode(data: &[u8]) -> String {
        const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = String::new();
        let mut i = 0;
        while i < data.len() {
            let b0 = data[i] as u32;
            let b1 = if i + 1 < data.len() { data[i + 1] as u32 } else { 0 };
            let b2 = if i + 2 < data.len() { data[i + 2] as u32 } else { 0 };
            let triple = (b0 << 16) | (b1 << 8) | b2;

            result.push(CHARSET[((triple >> 18) & 0x3F) as usize] as char);
            result.push(CHARSET[((triple >> 12) & 0x3F) as usize] as char);

            if i + 1 < data.len() {
                result.push(CHARSET[((triple >> 6) & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }

            if i + 2 < data.len() {
                result.push(CHARSET[(triple & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }

            i += 3;
        }
        result
    }

    /// Decode base64 string to bytes.
    pub fn decode(data: &[u8]) -> Result<Vec<u8>, DecodeError> {
        let mut result = Vec::new();
        let mut i = 0;

        while i < data.len() {
            // Skip whitespace
            while i < data.len() && (data[i] == b' ' || data[i] == b'\n' || data[i] == b'\r' || data[i] == b'\t') {
                i += 1;
            }
            if i >= data.len() { break; }

            let mut buf = [0u8; 4];
            let mut count = 0;
            for j in 0..4 {
                if i + j >= data.len() || data[i + j] == b'=' { break; }
                buf[j] = lookup_char(data[i + j]);
                count += 1;
            }
            if count < 2 {
                return Err(DecodeError::InvalidLength);
            }

            let triple = ((buf[0] as u32) << 18)
                | ((buf[1] as u32) << 12)
                | ((buf[2] as u32) << 6)
                | (buf[3] as u32);

            result.push(((triple >> 16) & 0xFF) as u8);
            if count > 2 {
                result.push(((triple >> 8) & 0xFF) as u8);
            }
            if count > 3 {
                result.push((triple & 0xFF) as u8);
            }

            i += count;
        }

        Ok(result)
    }

    fn lookup_char(c: u8) -> u8 {
        match c {
            b'A'..=b'Z' => c - b'A',
            b'a'..=b'z' => c - b'a' + 26,
            b'0'..=b'9' => c - b'0' + 52,
            b'+' => 62,
            b'/' => 63,
            _ => 0,
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum DecodeError {
        InvalidLength,
    }

    impl fmt::Display for DecodeError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                DecodeError::InvalidLength => write!(f, "invalid base64 length"),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_message_serialization() {
        let msg = WsMessage::Sync { data: vec![1, 2, 3] };
        let json = msg.to_json().unwrap();
        assert!(json.contains(r#""type":"sync""#));

        let decoded: WsMessage = WsMessage::from_json(&json).unwrap();
        assert!(matches!(decoded, WsMessage::Sync { data } if data == vec![1, 2, 3]));
    }

    #[test]
    fn test_ping_message_serialization() {
        let msg = WsMessage::Ping;
        let json = msg.to_json().unwrap();
        assert!(json.contains(r#""type":"Ping""#));

        let decoded: WsMessage = WsMessage::from_json(&json).unwrap();
        assert!(decoded.is_ping());
    }

    #[test]
    fn test_pong_message_serialization() {
        let msg = WsMessage::Pong;
        let json = msg.to_json().unwrap();
        assert!(json.contains(r#""type":"Pong""#));
        assert!(msg.is_pong());
    }

    #[test]
    fn test_cursor_message_serialization() {
        let msg = WsMessage::Cursor {
            user_id: "user123".to_string(),
            username: "Alice".to_string(),
            cursor: CursorPos { line: 5, ch: 10 },
            selection: Some(SelectionRange {
                from: CursorPos { line: 5, ch: 10 },
                to: CursorPos { line: 5, ch: 15 },
            }),
        };
        let json = msg.to_json().unwrap();
        assert!(json.contains(r#""type":"Cursor""#));
        assert!(json.contains(r#""line":5"#));

        let decoded: WsMessage = WsMessage::from_json(&json).unwrap();
        match decoded {
            WsMessage::Cursor { user_id, cursor, .. } => {
                assert_eq!(user_id, "user123");
                assert_eq!(cursor.line, 5);
                assert_eq!(cursor.ch, 10);
            }
            _ => panic!("Expected Cursor variant"),
        }
    }

    #[test]
    fn test_cursor_message_without_selection() {
        let msg = WsMessage::Cursor {
            user_id: "user123".to_string(),
            username: "Alice".to_string(),
            cursor: CursorPos { line: 5, ch: 10 },
            selection: None,
        };
        let json = msg.to_json().unwrap();
        // Selection should not be present when None
        assert!(!json.contains("selection"));
    }

    #[test]
    fn test_is_ping_and_is_pong() {
        assert!(WsMessage::Ping.is_ping());
        assert!(WsMessage::Pong.is_pong());
        assert!(!WsMessage::Ping.is_pong());
        assert!(!WsMessage::Pong.is_ping());
        assert!(!WsMessage::Sync { data: vec![] }.is_ping());
        assert!(!WsMessage::Cursor {
            user_id: "u".to_string(),
            username: "u".to_string(),
            cursor: CursorPos { line: 0, ch: 0 },
            selection: None,
        }.is_ping());
    }

    #[test]
    fn test_cursor_message_type_checks() {
        assert!(!WsMessage::Ping.is_sync());
        assert!(!WsMessage::Ping.is_awareness());
        assert!(!WsMessage::Sync { data: vec![] }.is_ping());
        assert!(WsMessage::Sync { data: vec![] }.is_sync());
        assert!(WsMessage::Awareness { data: vec![] }.is_awareness());
    }

    #[test]
    fn test_cursor_serialization_with_none_selection() {
        let msg = WsMessage::cursor(
            "user1",
            "Alice",
            CursorPos { line: 3, ch: 7 },
            None,
        );
        let json = msg.to_json().unwrap();
        assert!(json.contains(r#""type":"Cursor""#));
        assert!(!json.contains("selection"));

        let decoded: WsMessage = WsMessage::from_json(&json).unwrap();
        match decoded {
            WsMessage::Cursor { user_id, cursor, selection, .. } => {
                assert_eq!(user_id, "user1");
                assert_eq!(cursor, CursorPos { line: 3, ch: 7 });
                assert!(selection.is_none());
            }
            _ => panic!("Expected Cursor variant"),
        }
    }
}
