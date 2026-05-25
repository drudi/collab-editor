# P4-T05 — Keyboard Shortcuts

**Status:** Done

## Goal

Implement keyboard shortcuts for common editing operations.

## Description

Create `src/hooks/useKeyboardShortcuts.ts` — a React hook that registers keyboard shortcuts for common editing operations. Shortcuts are passed through to CodeMirror's built-in commands where possible.

Shortcuts to implement:
- **Ctrl+S** — trigger save (show auto-save indicator)
- **Ctrl+/** — toggle line comment (pass to CodeMirror)
- **Ctrl+Z** — undo (pass to CodeMirror)
- **Ctrl+Shift+Z** — redo (pass to CodeMirror)
- **Ctrl+Shift+F** — open find/replace panel (pass to CodeMirror search addon)

Implementation:
- Use `useEffect` with `addEventListener('keydown')` for custom shortcuts
- Use CodeMirror's built-in keymap for shortcuts that CodeMirror handles natively
- Prevent default browser behavior for all registered shortcuts
- Visual feedback: show a brief toast/notification for each shortcut action

## Acceptance Criteria

- AC1: `useKeyboardShortcuts` hook exists
- AC2: Ctrl+S triggers save and shows auto-save indicator
- AC3: Ctrl+/ toggles line comment via CodeMirror command
- AC4: Ctrl+Z triggers undo via CodeMirror command
- AC5: Ctrl+Shift+Z triggers redo via CodeMirror command
- AC6: Ctrl+Shift+F triggers find/replace via CodeMirror search addon
- AC7: `preventDefault()` called for all registered shortcuts
- AC8: Shortcuts only active when editor is focused
- AC9: Visual feedback (toast or indicator) for each shortcut action
- AC10: No lint errors

## Technical Hints

- Hook pattern:
  ```typescript
  function useKeyboardShortcuts(view: EditorView, onSave: () => void) {
    useEffect(() => {
      const handler = (e: KeyboardEvent) => {
        if (e.ctrlKey || e.metaKey) {
          switch (e.key) {
            case 's': e.preventDefault(); onSave(); break;
            case '/': e.preventDefault(); toggleComment(view); break;
          }
        }
      };
      document.addEventListener('keydown', handler);
      return () => document.removeEventListener('keydown', handler);
    }, []);
  }
  ```
- CodeMirror keymap: use `keymap.of([...])` in extensions
- Toggle comment: `toggleComment({ shift: true })(view.state, () => {})`
- Find/replace: CodeMirror's `search` add-on handles Ctrl+Shift+F natively with `searchKeymap`
- Refer to: https://codemirror.net/docs/ref/#commands.toggleComment
- Refer to: https://codemirror.net/docs/ref/#search.searchKeymap
