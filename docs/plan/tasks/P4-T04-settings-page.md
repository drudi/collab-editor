# P4-T04 — Settings Page

**Status:** TODO

## Goal

Create the settings page with account, editor, and notification preferences.

## Description

Create `src/pages/SettingsPage.tsx` — a tabbed settings page where users can manage their account, editor preferences, and notification settings. Preferences are persisted to localStorage.

Three tabs:

**Tab 1 — Account:**
- Current username (display-only)
- Change password form (current password + new password)
- Delete account button (warning confirmation)

**Tab 2 — Editor:**
- Font size slider: 12–20px (default: 14)
- Tab width selector: 2 or 4 (default: 2)
- Theme toggle: light/dark (default: system preference)
- Changes apply immediately to the current editor

**Tab 3 — Notifications:**
- (Placeholder for future notification preferences)
- Currently shows "Coming soon" message

All settings saved to `localStorage` with key `collab-editor-settings`.

## Acceptance Criteria

- AC1: `SettingsPage` component exists in `src/pages/SettingsPage.tsx`
- AC2: Three tabs: Account, Editor, Notifications
- AC3: Account tab shows username (read-only)
- AC4: Account tab has change password form
- AC5: Editor tab has font size slider (12–20px)
- AC6: Editor tab has tab width selector (2 or 4)
- AC7: Editor tab has theme toggle (light/dark)
- AC8: All settings persisted to `localStorage`
- AC9: Settings applied immediately when changed
- AC10: Notifications tab shows "Coming soon" message
- AC11: Tailwind-styled with dark mode support
- AC12: No lint errors

## Technical Hints

- Tab implementation: `useState<'account' | 'editor' | 'notifications'>`
- localStorage pattern:
  ```typescript
  const [settings, setSettings] = useState(() => {
    const saved = localStorage.getItem('collab-editor-settings');
    return saved ? JSON.parse(saved) : defaultSettings;
  });
  
  useEffect(() => {
    localStorage.setItem('collab-editor-settings', JSON.stringify(settings));
  }, [settings]);
  ```
- Theme toggle: `document.documentElement.classList.toggle('dark')`
- Font size: `document.body.style.fontSize = `${size}px``
- Refer to: https://developer.mozilla.org/en-US/docs/Web/API/Window/localStorage
