# P2-T08 — CodeMirror + Yjs Binding

**Status:** TODO

## Goal

Bind the Yjs `YText` document to a CodeMirror editor with real-time sync.

## Description

Create `src/collab/cm-yjs-binding.ts` — the integration layer that binds Yjs shared text to CodeMirror 6. This is the core of the collaborative editing experience.

The binding:
1. Creates a CodeMirror `EditorView` with `basicSetup`
2. Uses an `updateListener` to detect local changes
3. On local change: applies update to `YText` (with origin marker to prevent echo)
4. On `YText.observe`: applies remote changes to CodeMirror state
5. Uses `transaction.origin` to distinguish local vs remote changes

Key implementation:
- On local edit: `ytext.transact(() => ytext.insert/delete(...), 'local-origin')`
- On remote update: `ytext.observe(event => { view.dispatch({ changes: ... }) }, 'remote')`
- The origin marker prevents the editor from echoing its own changes back to Yjs

The binding must handle:
- Content synchronization without flicker
- Cursor position preservation during remote updates
- Proper undo/redo scope (only undo local changes)

## Acceptance Criteria

- AC1: `createCmYjsBinding(ytext, view)` function defined
- AC2: CodeMirror `EditorView` created with `basicSetup`
- AC3: `updateListener` detects local changes
- AC4: Local changes applied to `YText` with origin marker
- AC5: `YText.observe` applies remote changes to editor state
- AC6: Origin-based filtering prevents echo of own changes
- AC7: `disconnect()` function to clean up observer and view
- AC8: TypeScript types for binding API
- AC9: No lint errors
- AC10: Editor renders and accepts input without errors

## Technical Hasts

- CodeMirror update listener:
  ```typescript
  EditorView.updateListener.of((update) => {
    if (update.docChanged) {
      const newText = update.state.doc.toString();
      ytext.transact(() => {
        if (ytext.toString() !== newText) {
          ytext.delete(0, ytext.length);
          ytext.insert(0, newText);
        }
      }, 'local');
    }
  })
  ```
- Yjs origin check: `if (event.transaction.origin === 'local') return;`
- Yrs text observe: `ytext.observe((event) => { ... })`
- For incremental sync: use `ytext.diff()` to compute changes and apply via CodeMirror `changes`
- Refer to: https://codemirror.net/docs/ref/#view.EditorView
- Refer to: https://codemirror.net/docs/ref/#state.Transaction
