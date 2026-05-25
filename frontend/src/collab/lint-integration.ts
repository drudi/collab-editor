/**
 * CodeMirror Lint Integration
 *
 * Integrates LSP diagnostics from the WebSocket into CodeMirror 6's lint system.
 * Diagnostics arrive as WsMessage::Lint events and are converted to CodeMirror
 * Diagnostic objects for display in the editor.
 *
 * Architecture:
 * ```text
 * WS Message (type: "lint")
 *        │
 *        ▼
 * convertLspDiagnostic() → CodeMirror Diagnostic
 *        │
 *        ▼
 * lintState.update() → Display in editor
 * ```
 */

import { EditorState, Extension } from '@codemirror/state';
import { linter, Diagnostic } from '@codemirror/lint';
import { EditorView } from '@codemirror/view';
import { WsMessage } from '../hooks/useWebSocket';

/**
 * Convert an LSP diagnostic severity code to a CodeMirror severity string.
 *
 * LSP severity codes (per LSP specification):
 * - 1 = Error
 * - 2 = Warning
 * - 3 = Information
 * - 4 = Hint
 *
 * CodeMirror severity strings:
 * - 'error' — Red squiggly underline
 * - 'warning' — Yellow squiggly underline
 * - 'info' — Blue squiggly underline
 * - 'hint' — Blue squiggly underline (with dashed border)
 *
 * @param severity — LSP diagnostic severity code
 * @returns CodeMirror diagnostic severity string
 */
export function lspSeverityToCodeMirror(severity: number): 'error' | 'warning' | 'info' | 'hint' {
  switch (severity) {
    case 1:
      return 'error';
    case 2:
      return 'warning';
    case 3:
      return 'info';
    case 4:
      return 'hint';
    default:
      return 'error';
  }
}

/**
 * Convert an LSP diagnostic to a CodeMirror Diagnostic.
 *
 * Position conversion from LSP {line, column} (0-indexed) to CodeMirror
 * character position (1-indexed):
 * ```
 * from = state.doc.getLine(line + 1).from + column - 1
 * ```
 *
 * @param diagnostic — The LSP diagnostic
 * @param state — The editor state (for position calculation)
 * @returns CodeMirror Diagnostic
 */
export function lspDiagnosticToCodeMirror(
  diagnostic: LspDiagnostic,
  state: EditorState,
): Diagnostic {
  const lineObj = state.doc.getLine(diagnostic.line + 1);
  const from = lineObj.from + diagnostic.column - 1;
  const to = lineObj.from + diagnostic.column - 1;

  return {
    from,
    to,
    severity: lspSeverityToCodeMirror(diagnostic.severity),
    message: diagnostic.message,
  };
}

/**
 * LSP diagnostic received from the server.
 *
 * Mirrors the Rust `LspDiagnostic` struct for type-safe deserialization.
 */
export interface LspDiagnostic {
  line: number;
  column: number;
  end_line?: number;
  end_column?: number;
  severity: number;
  message: string;
}

/**
 * Convert a WsMessage Lint variant to CodeMirror diagnostics.
 *
 * For each LSP diagnostic in the message:
 * 1. Gets the line object for the diagnostic's line
 * 2. Converts from/to positions using the editor state
 * 3. Maps severity code to CodeMirror severity string
 *
 * @param message — The WsMessage::Lint variant
 * @param state — The current editor state
 * @returns Array of CodeMirror Diagnostic objects
 */
export function lintMessageToDiagnostics(
  message: WsMessage,
  state: EditorState,
): Diagnostic[] {
  if (message.type !== 'lint') {
    return [];
  }

  return message.diagnostics
    .map((diag: LspDiagnostic) => lspDiagnosticToCodeMirror(diag, state))
    .filter((d): d is Diagnostic => d.message.length > 0);
}

/**
 * Create a CodeMirror lint extension that receives diagnostics from a callback.
 *
 * The `onDiagnostics` callback is invoked by the WebSocket hook when
 * a `WsMessage::Lint` message is received. The callback converts the
 * diagnostic and updates the lint state.
 *
 * This pattern decouples the lint system from the WebSocket layer:
 * ```typescript
 * const linterExt = createLinter((diagnostics) => {
 *   lintState.update(diagnostics);
 * });
 * ```
 *
 * @param onDiagnostics — Callback invoked with new diagnostics
 * @returns CodeMirror Extension
 */
export function createLinter(
  onDiagnostics: (diagnostics: Diagnostic[]) => void,
): Extension {
  return linter((view) => {
    // The linter function returns diagnostics for the current view state.
    // However, our diagnostics come from the WebSocket (external source),
    // not from analyzing the current document.
    // We store the latest diagnostics in a closure and return them here.
    // This is a simplified approach — in production, you'd use a proper
    // lint state management system.
    const diagnostics = onDiagnostics([]);
    return diagnostics;
  });
}

/**
 * Linter state management for receiving WS diagnostic updates.
 *
 * Provides a clean interface for the WebSocket hook to deliver
 * lint diagnostics to the editor.
 */
export class LintManager {
  private currentDiagnostics: Diagnostic[] = [];
  private onUpdate: ((diagnostics: Diagnostic[]) => void) | null = null;
  private lintState: any = null;

  /**
   * Set the lint state provider (called by the hook).
   */
  setLintState(lintState: any): void {
    this.lintState = lintState;
  }

  /**
   * Set the update callback (for the linter extension).
   */
  setOnUpdate(onUpdate: (diagnostics: Diagnostic[]) => void): void {
    this.onUpdate = onUpdate;
  }

  /**
   * Process a lint message from the WebSocket and update the lint state.
   *
   * @param message — The WsMessage::Lint message
   * @param editorState — The current editor state for position calculation
   */
  processLintMessage(message: WsMessage, editorState: EditorState): void {
    if (message.type !== 'lint') {
      return;
    }

    // Convert LSP diagnostics to CodeMirror diagnostics
    const diagnostics: Diagnostic[] = message.diagnostics
      .map((diag: LspDiagnostic) => lspDiagnosticToCodeMirror(diag, editorState))
      .filter((d): d is Diagnostic => d.message.length > 0);

    this.currentDiagnostics = diagnostics;

    // Notify the lint state
    if (this.onUpdate) {
      this.onUpdate(diagnostics);
    }

    // Direct update if lintState is available
    if (this.lintState) {
      this.lintState.update(diagnostics);
    }
  }

  /**
   * Get the current diagnostics (for testing or inspection).
   */
  getCurrentDiagnostics(): Diagnostic[] {
    return this.currentDiagnostics;
  }

  /**
   * Clear all diagnostics.
   */
  clearDiagnostics(): void {
    this.currentDiagnostics = [];
    if (this.onUpdate) {
      this.onUpdate([]);
    }
    if (this.lintState) {
      this.lintState.update([]);
    }
  }
}
