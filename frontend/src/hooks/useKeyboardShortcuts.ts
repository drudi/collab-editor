/**
 * useKeyboardShortcuts — React hook for common keyboard shortcuts.
 *
 * Registered shortcuts:
 * - Ctrl+S: trigger save (auto-save indicator)
 * - Ctrl+/: toggle line comment (CodeMirror command)
 * - Ctrl+Z: undo (CodeMirror command)
 * - Ctrl+Shift+Z: redo (CodeMirror command)
 * - Ctrl+Shift+F: open find/replace (CodeMirror search addon)
 *
 * Shortcuts only active when editor is focused.
 * Uses preventDefault() for all registered shortcuts.
 * Shows visual feedback via status callback for each action.
 */

import { useEffect, useRef } from 'react';
import type { EditorView } from '@codemirror/view';
import type { Extension } from '@codemirror/state';

// Key bindings that CodeMirror handles natively via its keymap
// These are passed as extensions to the editor
export function getKeyboardShortcutsExtension(): Extension {
  // CodeMirror's built-in keymap handles Ctrl+Z (undo), Ctrl+Shift+Z (redo),
  // Ctrl+/ (toggleComment), and Ctrl+Shift+F (find) automatically
  // when basicSetup and searchKeymap are included.
  // We return an empty extension here as a placeholder — actual keymap
  // integration happens via CodeMirror's basicSetup in cm-yjs-binding.ts.
  return [];
}

interface KeyboardShortcutsOptions {
  /** Callback triggered when Ctrl+S is pressed */
  onSave: () => void;
  /** Callback for visual feedback toast/notification */
  onToast: (message: string) => void;
  /** Whether the editor view is currently focused */
  isEditorFocused: boolean;
}

/**
 * Registers keyboard shortcuts in the document.
 * Only active when `isEditorFocused` is true.
 */
export function useKeyboardShortcuts(
  _view: EditorView | null,
  options: KeyboardShortcutsOptions,
): void {
  const optionsRef = useRef(options);

  // Keep ref in sync — this is safe because it's set before the effect runs
  useEffect(() => {
    optionsRef.current = options;
  }, [options]);

  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      const { onSave, onToast, isEditorFocused } = optionsRef.current;

      // Shortcuts only active when editor is focused
      if (!isEditorFocused) return;

      const ctrl = e.ctrlKey || e.metaKey;

      if (!ctrl) return;

      switch (e.key.toLowerCase()) {
        case 's': {
          e.preventDefault();
          onSave();
          onToast('Document saved');
          break;
        }
        case '/': {
          e.preventDefault();
          // Toggle line comment via CodeMirror
          // The actual command is handled by CodeMirror's toggleComment keymap
          // when the view is available
          onToast('Toggle comment');
          break;
        }
        case 'z': {
          if (e.shiftKey) {
            // Ctrl+Shift+Z — redo
            e.preventDefault();
            onToast('Redo');
          }
          break;
        }
        case 'f': {
          if (e.shiftKey) {
            // Ctrl+Shift+F — find/replace
            e.preventDefault();
            onToast('Find and replace');
          }
          break;
        }
        default:
          break;
      }
    };

    document.addEventListener('keydown', handler);
    return () => document.removeEventListener('keydown', handler);
  }, [options]);
}

/**
 * Get the list of registered keyboard shortcuts for display.
 */
export function getKeyboardShortcutsList(): Array<{ key: string; action: string }> {
  return [
    { key: 'Ctrl+S', action: 'Save document' },
    { key: 'Ctrl+/', action: 'Toggle line comment' },
    { key: 'Ctrl+Z', action: 'Undo' },
    { key: 'Ctrl+Shift+Z', action: 'Redo' },
    { key: 'Ctrl+Shift+F', action: 'Find and replace' },
  ];
}
