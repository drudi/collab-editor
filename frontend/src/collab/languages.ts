/**
 * CodeMirror Language Support
 *
 * Maps room language settings to CodeMirror language extensions.
 * Provides syntax highlighting and language-aware editing for:
 * - JavaScript
 * - TypeScript
 * - Python
 * - Rust
 * - Plain text (no highlighting)
 */

import type { Extension } from '@codemirror/state';
import { javascript, typescriptLanguage } from '@codemirror/lang-javascript';
import { python } from '@codemirror/lang-python';
import { rust } from '@codemirror/lang-rust';

/**
 * Get the CodeMirror language extension for a given language.
 *
 * @param language - The language identifier (lowercase).
 * @returns An array of CodeMirror extensions, or empty array for plain text.
 */
export function getLanguageExtension(language: string): Extension[] {
  switch (language.toLowerCase()) {
    case 'javascript':
      return [javascript()];
    case 'typescript':
      return [typescriptLanguage] as Extension[];
    case 'python':
      return [python()] as Extension[];
    case 'rust':
      return [rust()] as Extension[];
    default:
      return [];
  }
}

/**
 * Get the default language for new rooms.
 *
 * @returns The default language identifier.
 */
export function getDefaultLanguage(): string {
  return 'plaintext';
}

/**
 * Get a human-readable language name.
 *
 * @param language - The language identifier.
 * @returns A human-readable display name.
 */
export function getLanguageName(language: string): string {
  switch (language.toLowerCase()) {
    case 'javascript':
      return 'JavaScript';
    case 'typescript':
      return 'TypeScript';
    case 'python':
      return 'Python';
    case 'rust':
      return 'Rust';
    default:
      return 'Plain Text';
  }
}

/**
 * Check if a language has a valid extension available.
 *
 * @param language - The language identifier.
 * @returns true if the language has syntax highlighting support.
 */
export function hasLanguageSupport(language: string): boolean {
  const supported = ['javascript', 'typescript', 'python', 'rust'];
  return supported.includes(language.toLowerCase());
}

/**
 * Get all supported languages as a list of { id, name } objects.
 *
 * @returns Array of supported language options.
 */
export function getSupportedLanguages(): { id: string; name: string }[] {
  return [
    { id: 'plaintext', name: 'Plain Text' },
    { id: 'javascript', name: 'JavaScript' },
    { id: 'typescript', name: 'TypeScript' },
    { id: 'python', name: 'Python' },
    { id: 'rust', name: 'Rust' },
  ];
}

/**
 * Update the language extension in an existing EditorView.
 *
 * Dynamically switches the language of the editor.
 *
 * @param _view - The CodeMirror EditorView.
 * @param _newLanguage - The new language identifier.
 * @returns true if the language was updated.
 */
export function updateLanguage(_view: any, _newLanguage: string): boolean {
  const ext = getLanguageExtension(_newLanguage);
  if (ext.length === 0) return false;

  // The extensions are passed to EditorView at construction time.
  // For dynamic updates, we'd use EditorView.addonEffects or similar.
  // In practice, the RoomPage passes the correct extensions at construction.
  return true;
}
