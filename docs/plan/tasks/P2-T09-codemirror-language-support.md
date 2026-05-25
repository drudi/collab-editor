# P2-T09 — CodeMirror Language Support

**Status:** Done

## Goal

Map room language to CodeMirror language extensions for syntax highlighting.

## Description

Create `src/collab/languages.ts` — a module that maps room language settings to the appropriate CodeMirror language extension. This provides syntax highlighting and language-aware editing for different programming languages.

The language mapping:
- `javascript` → `javascript()` from `@codemirror/lang-javascript`
- `typescript` → `typescript()` from `@codemirror/lang-javascript`
- `python` → `python()` from `@codemirror/lang-python`
- `rust` → `rust()` from `@codemirror/lang-rust`
- `plaintext` → `null` (no language extension)

The module exports:
- `getLanguageExtension(language: string): Extension[]` — returns language extensions
- `getDefaultLanguage(): string` — returns 'plaintext'
- `getLanguageName(language: string): string` — human-readable name

The language extension is passed to the CodeMirror `EditorView` constructor and can be updated dynamically when the room language changes.

## Acceptance Criteria

- AC1: `getLanguageExtension(language)` function defined
- AC2: JavaScript/TypeScript mapped to `@codemirror/lang-javascript`
- AC3: Python mapped to `@codemirror/lang-python`
- AC4: Rust mapped to `@codemirror/lang-rust`
- AC5: PlainText returns empty extension array (no highlighting)
- AC6: `getDefaultLanguage()` returns 'plaintext'
- AC7: `getLanguageName()` returns human-readable name
- AC8: Extension can be updated dynamically in EditorView
- AC9: TypeScript types for all exports
- AC10: No lint errors

## Technical Hints

- CodeMirror extension pattern:
  ```typescript
  import { javascript, TypeScript } from '@codemirror/lang-javascript';
  import { python } from '@codemirror/lang-python';
  import { rust } from '@codemirror/lang-rust';
  
  export function getLanguageExtension(language: string): Extension[] {
    switch (language) {
      case 'javascript': return [javascript()];
      case 'typescript': return [TypeScript()];
      case 'python': return [python()];
      case 'rust': return [rust()];
      default: return [];
    }
  }
  ```
- Extensions are passed to `new EditorView({ extensions: [...] })`
- Dynamic update: `view.dispatch({ effects: EditorView.addonEffects.of(newExt) })`
- Refer to: https://codemirror.net/docs/ref/#language
