/**
 * CodeMirror + Yjs Binding
 *
 * Binds a Yjs YText document to a CodeMirror EditorView with real-time sync.
 *
 * Sync loop:
 * 1. User types → updateListener → apply to YText with 'local' origin
 * 2. Remote change → YText.observe (origin='remote') → apply to CodeMirror
 * 3. Origin markers prevent echo of own changes
 *
 * Handles cursor position preservation during remote updates.
 */

import type { Extension } from '@codemirror/state';
import { EditorView } from '@codemirror/view';
import * as Y from 'yjs';

// Origin markers for change classification
const LOCAL_ORIGIN = 'local';

// ─── Types ──────────────────────────────────────────────────────────────────

export interface CmYjsBindingOptions {
  /** The Yjs YText shared type. */
  ytext: Y.Text;
  /** Optional extensions to add to the editor (language, theme, etc.). */
  extensions?: Extension[];
  /** Optional initial document content. */
  initialContent?: string;
}

export interface CmYjsBinding {
  /** The CodeMirror EditorView instance. */
  view: EditorView;
  /** Disconnect the binding and clean up. */
  disconnect: () => void;
}

// ─── Binding ────────────────────────────────────────────────────────────────

/**
 * Create a binding between a Yjs YText and a CodeMirror editor.
 *
 * The binding establishes a bidirectional sync loop:
 * - Local edits (user types) → updateListener → YText (origin='local')
 * - Remote edits (observe) → CodeMirror dispatch (origin='remote')
 *
 * Origin markers prevent echo: the updateListener skips changes where
 * YText content matches the editor doc (remote changes), and the
 * YText observer skips changes with 'local' origin.
 *
 * @param container - DOM element to mount the editor into.
 * @param options - Configuration options.
 * @returns The binding instance with view and disconnect.
 */
export function createCmYjsBinding(
  container: HTMLElement,
  { ytext, extensions = [], initialContent = '' }: CmYjsBindingOptions,
): CmYjsBinding {
  // Determine initial content from YText or provided content
  const docContent = initialContent || ytext.toString() || '';

  // Create the editor view
  const view = new EditorView({
    doc: docContent,
    extensions: [
      // Enable editing
      EditorView.editable.of(true),

      // Detect local changes and sync to YText
      EditorView.updateListener.of((update: { docChanged: boolean; state: { doc: { toString: () => string } } }) => {
        if (!update.docChanged) return;

        const newContent = update.state.doc.toString();
        const yContent = ytext.toString();

        // If content differs from YText, this is a local change
        if (yContent !== newContent) {
          // Apply to YText with 'local' origin marker
          ytext.doc?.transact(() => {
            ytext.delete(0, yContent.length);
            ytext.insert(0, newContent);
          }, LOCAL_ORIGIN);
        }
      }),

      // Merge in language/theme extensions
      ...extensions,
    ],
  });

  // Mount the view
  container.appendChild(view.dom);

  // Set up YText observer for remote changes
  const observer = (event: Y.YTextEvent) => {
    // Skip changes we originated locally
    if (event.transaction.origin === LOCAL_ORIGIN) return;

    // Apply remote changes to the editor
    const newContent = ytext.toString();
    const currentContent = view.state.doc.toString();

    if (currentContent !== newContent) {
      view.dispatch({
        changes: {
          from: 0,
          to: currentContent.length,
          insert: newContent,
        },
        // Preserve cursor position: if cursor was at position p,
        // keep it relative to the same text offset
        selection: { anchor: Math.min(view.state.selection.main.anchor, newContent.length) },
      });
    }
  };
  ytext.observe(observer);

  // Return binding with disconnect
  return {
    view,
    disconnect: () => {
      // Remove observer
      ytext.unobserve(observer);
      // Destroy editor view
      view.destroy();
      // Remove DOM element
      if (container.contains(view.dom)) {
        container.removeChild(view.dom);
      }
    },
  };
}

/**
 * Apply remote changes to the CodeMirror editor.
 *
 * Called when YText changes are detected remotely (via Yjs awareness or direct sync).
 * This is a convenience function for cases where the observer pattern is not used.
 *
 * @param view - The CodeMirror EditorView.
 * @param ytext - The Yjs YText that changed.
 */
export function applyRemoteChanges(view: EditorView, ytext: Y.Text): void {
  const currentContent = view.state.doc.toString();
  const remoteContent = ytext.toString();

  if (currentContent === remoteContent) return;

  view.dispatch({
    changes: {
      from: 0,
      to: currentContent.length,
      insert: remoteContent,
    },
    selection: { anchor: Math.min(view.state.selection.main.anchor, remoteContent.length) },
  });
}
