# UX Design Option 2 — Clean Minimal

## Design Philosophy

Light theme, spacious, modern aesthetic. Reduces visual noise to focus on code. Inspired by modern SaaS design patterns — think Linear, Notion, Vercel.

## Color Palette

| Token | Hex | Usage |
|-------|-----|-------|
| Background | `#ffffff` | Main editor |
| Surface | `#f9fafb` | Sidebar, panels |
| Border | `#e5e7eb` | Dividers, borders |
| Text | `#111827` | Primary text |
| Text Secondary | `#6b7280` | Labels, placeholders |
| Accent | `#6366f1` | Buttons, active states |
| Accent Hover | `#4f46e5` | Button hover |
| Cursor Alice | `#ef4444` | Alice's cursor |
| Cursor Bob | `#22c55e` | Bob's cursor |
| Cursor Charlie | `#f59e0b` | Charlie's cursor |
| Error | `#dc2626` | Lint errors |
| Warning | `#eab308` | Lint warnings |
| Success | `#16a34a` | Save confirmation, joins |
| Selection | `#e0e7ff` | Cursor selection highlight |

## Layout — Room Page (1920x1080)

```
+--------------------------------------------------------------------------+
| [Logo] CollabEdit                   Search files...  [ABC123] [Users▼]  |
+--------------------------------------------------------------------------+
| Sidebar (220px)   | Editor (flex)              | Panel (260px)          |
|                   |                            |                        |
| +----------------+ | +----------------------+   | +------------------+   |
| | Files          | | | RoomPage.tsx           |   | | Collaborators    |   |
| |                | | |----------------------|   | |                  |   |
| | > src          | | |  import React        |   | | [🟢] Alice       |   |
| |   > comp       | | |  import { Editor}    |   | | [🟢] Bob         |   |
| |     RoomPage   | | |                      |   | | [🟡] Charlie     |   |
| |     Editor     | | |  function Room() {    |   | |                  |   |
| |   > hooks      | | |    return (           |   | | + Add member     |   |
| |     useAuth    | | |      <Editor />       |   | +------------------+   |
| |     useWS      | | |      </>              |   |                        |
| |   > utils      | | |                      |   | +------------------+   |
| |   config.ts    | | |                      |   | | Room Info          |   |
| |                | | |                      |   | | ------------------ |   |
| | +-------------- | | |                      |   | | Code: ABC123       |   |
| | | README       | | |                      |   | | Language: TS       |   |
| | +-------------- | | |                      |   | | Members: 3         |   |
| |                | | |                      |   | | Created: today     |   |
| | > + New File   | | |                      |   | +------------------+   |
| +----------------+ | |                      |   |                        |
|                   | |                      |   | +------------------+   |
|                   | |                      |   | | Activity           |   |
|                   | |                      |   | | ------------------ |   |
|                   | |                      |   | | Alice joined       |   |
|                   | |                      |   | | Bob saved file     |   |
|                   | |                      |   | | Charlie edited     |   |
|                   | |                      |   | +------------------+   |
+-------------------+----------------------+   +------------------------+
|  Files: 3  |  Lines: 142  |  Saved  |  3 online                       |
+--------------------------------------------------------------------------+
```

## Key UX Decisions

### Top Bar — Minimal

- Logo on left (clickable → home page)
- Center: file search bar (Ctrl+P to activate)
- Right: room code badge (click to copy), users dropdown, settings gear

### Sidebar — Tree View

- Clean file tree with chevron expand/collapse
- Active file highlighted with accent color left border
- Hover effect on files: subtle background shift
- "+ New File" at bottom of tree
- Icons per file type (minimal, outline style)

### Panel — Two Tabs

**Tab 1: Collaborators**
- Inline list of users with colored circle initials
- Green dot = online, yellow = idle (3 min no activity)
- Username next to circle
- Click user → jump to their cursor location
- "+ Add member" button for inviting

**Tab 2: Room Info**
- Room code with copy button
- Language mode indicator
- Member count
- Creation timestamp
- "Leave Room" button at bottom (destructive, red)

### Collab Features

- Cursor: solid line in user's color, username label follows cursor
- Selection: semi-transparent fill in user's color
- Remote typing: characters flash briefly in user's color at insertion point
- Inactive cursor: fades to 40% opacity after 3 seconds
- Click remote cursor → shows "View Alice's position" link

### Editor

- Full-width editor, no borders
- Line numbers in muted gray (`#9ca3af`)
- Active line highlighted with subtle background (`#f3f4f6`)
- Smooth scroll, soft wrap toggle
- Minimap on right (toggleable, minimal)
- Font: JetBrains Mono, 14px (user adjustable)

### Save States

- Auto-save indicator: small spinner in bottom-left that fades to checkmark
- "Saved" with checkmark (green) fades out after 2 seconds
- Manual save: Ctrl+S (also shows confirmation)

### Room Code Sharing

- Room code shown as large badge in editor header
- Click badge → copy to clipboard + toast notification "Code copied!"
- "Share" button opens modal with copyable link

## Authentication Flow UX

```
+--------------------------------------------------+
|        Welcome to CollabEdit                      |
|                                                   |
|  Create an account to save your work and          |
|  access rooms across devices.                     |
|                                                   |
|     ┌──────────────────┐  ┌──────────────────┐   |
|     │   Sign Up         │                    │   |
|     │                  │  │     or            │   |
|     │ Name:  _______   │  │                  │   |
|     │ Email: _______   │  │   [Sign in]      │   |
|     │ Pwd:   _______   │  │                  │   |
|     │                  │  │                  │   |
|     │ [Create account] │  │                  │   |
|     └──────────────────┘  └──────────────────┘   |
|                                                   |
|     Continue as guest →                            |
+--------------------------------------------------+
```

- Simple two-column form (sign up left, sign in right)
- Guest link below forms in muted text
- Form validation: red border on invalid field, inline error message
- Password strength meter (3 bars: weak, medium, strong)

## Responsive Breakpoints

| Screen | Behavior |
|--------|----------|
| > 1400px | Full layout as shown above |
| 1000-1400px | Right panel collapses to icons; expand on hover |
| 700-1000px | Left sidebar collapses to icons; expand on hover |
| < 700px | Single column: editor full width, panels become bottom sheets |

## Accessibility

- Light theme has 16:1+ contrast on all text
- Focus rings: 2px solid accent color
- All interactive elements keyboard-navigable
- Screen reader: "Collaborator Alice, cursor at line 23"
- Reduced motion: disable cursor animations for users preferring it
