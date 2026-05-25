# P3-T04 — CodeMirror Lint Integration

**Status:** Done

## Goal

Integrate LSP diagnostics into CodeMirror 6's lint system.

## Description

Create `src/collab/lint-integration.ts` — a CodeMirror `lint` extension that receives diagnostics from the WebSocket and displays them in the editor.

The integration:
1. Creates a `createLinter()` function that returns a CodeMirror extension
2. Subscribes to WS `LintMessage` events in the hook
3. Converts LSP diagnostics to CodeMirror `Diagnostic` format
4. Updates CodeMirror's lint state with `lintState.update(diagnostics)`

CodeMirror `Diagnostic` type:
```typescript
interface Diagnostic {
  from: number;   // character position in document
  to: number;     // end position
  severity: 'error' | 'warning' | 'info' | 'hint';
  message: string;
}
```

Position conversion from LSP `{line, column}` to CodeMirror character position:
```typescript
function posToOffset(state: EditorState, line: number, column: number): number {
  const lineObj = state.doc.getLine(line + 1);
  return lineObj.from + column - 1;
}
```

Styling:
- Errors: red squiggly underline + red gutter icon
- Warnings: yellow squiggly underline + yellow gutter icon
- Info/Hint: blue squiggly underline + blue gutter icon

## Acceptance Criteria

- AC1: `createLinter()` function returns CodeMirror `Extension`
- AC2: WS `LintMessage` events subscribed and processed
- AC3: LSP diagnostics converted to CodeMirror `Diagnostic` format
- AC4: `lintState.update()` called with new diagnostics
- AC5: `from`/`to` position conversion from LSP line/column
- AC6: Errors styled with red coloring
- AC7: Warnings styled with yellow coloring
- AC8: Info/Hint styled with blue coloring
- AC9: Diagnostics update in real-time as LSP sends them
- AC10: No lint errors

## Technical Hints

- CodeMirror lint extension:
  ```typescript
  import { linter } from '@codemirror/lint';
  
  export function createLinter(onDiagnostics: (diag: Diagnostic[]) => void) {
    return linter((view) => {
      // diagnostics come from onDiagnostics callback
      return [];
    });
  }
  ```
- LSP severity mapping: `1 → 'error'`, `2 → 'warning'`, `3 → 'info'`, `4 → 'hint'`
- CodeMirror diagnostic: `{ from: pos, to: pos, severity: 'error', message: 'text' }`
- Refer to: https://codemirror.net/docs/ref/#lint
- Refer to: https://codemirror.net/docs/ref/#state.EditorState.doc
