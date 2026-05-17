# UX Design Option 1 — VS Code Live Share Style

## Design Philosophy

Professional, developer-first dark theme. Feels like VS Code — developers already know this interface. Minimizes cognitive load by mirroring a familiar environment.

## Color Palette

| Token | Hex | Usage |
|-------|-----|-------|
| Background | `#1e1e1e` | Main editor, panels |
| Sidebar | `#252526` | Left sidebar, right panel |
| Activity Bar | `#333333` | Icon rail |
| Text | `#d4d4d4` | Editor text |
| Accent | `#007acc` | Active elements, links |
| Cursor Alice | `#ff6b6b` | Alice's cursor/selection highlight |
| Cursor Bob | `#4ec9b0` | Bob's cursor/selection highlight |
| Cursor Charlie | `#c792ea` | Charlie's cursor/selection highlight |
| Border | `#3c3c3c` | Panel dividers |
| Status Bar | `#007acc` | Bottom status bar |
| Error | `#f44747` | Lint errors, validation |
| Warning | `#cca700` | Lint warnings |
| Info | `#3b8eea` | Activity feed info |

## Layout — Room Page (1920x1080)

```
+--------------------------------------------------------------------------+
| Activity Bar  | Left Sidebar (200px) | Editor (flex)    | Right Panel  |
|               |                      |                  | (280px)      |
| [Explorer]    | > src/               | +--------------------------------+|
| [Search]      |   components/        | | Room: Main Room                ||
| [Rooms]       |     RoomPage.jsx     | | Code: ABC123                   ||
| [Users]       |     hooks/           | |                                ||
| [Lint]        |       useAuth.ts     | |  import React from 'react';   ||
|               |       useWS.ts       | |  import { Editor } from './..';||
|               |   App.tsx            | |                                ||
|               |   main.ts            | |  function RoomPage() {        ||
|               |                      | |    const room = useRoom();    ||
|               | > .gitignore         | |    return <Editor />;         ||
|               |   README.md          | |  }                             ||
|               |                      | |                                ||
|               |                      | |                                ||
|               |                      | +--------------------------------+|
|               |                      | | Users (3)                      ||
|               |                      | | [🟢 Alice]                     ||
|               |                      | | [🟢 Bob]                       ||
|               |                      | | [🟢 Charlie]                   ||
|               |                      | +--------------------------------+|
|               |                      | | Activity                       ||
|               |                      | | Alice joined                  ||
|               |                      | | Document saved                ||
|               |                      | +--------------------------------+|
+---------------+----------------------+-----------------------------------+
| Status: Connected  | Room: ABC123  | Language: TypeScript  | Users: 3    |
+--------------------------------------------------------------------------+
```

## Key UX Decisions

### Left Sidebar — Collapsible Sections

- **Explorer**: File tree with icons per file type, expand/collapse folders
- **Search**: Simple search box, search across all files
- **Rooms**: List of user's rooms, create new room button
- **Users**: Shows all users in current room (also shown in right panel)
- **Lint**: Shows lint issues grouped by file, click to jump

### Right Panel — Three Tabs

**Tab 1: Users**
- User avatar (initials in colored circle)
- Username with presence indicator (green = online)
- Current role (editor/viewer)
- Cursor position summary ("at line 23, col 5")

**Tab 2: Activity Feed**
- Timestamped list of events
- Color-coded by type (green for joins, blue for saves, yellow for lints)
- Expandable to show file diffs
- "Show last N events" toggle

**Tab 3: Lint Results**
- File-level summary (X errors, Y warnings)
- Click any issue to jump to location in editor
- Filter by severity, file, rule
- Quick-fix buttons where applicable

### Cursor Indicators

- Each user gets a unique color from the palette above
- Remote cursor shown as:
  - Thin vertical line at caret position (with color)
  - Highlighted selection in semi-transparent color
  - Username tooltip on hover
- Cursor blurs when user is inactive for 3 seconds
- Cursor disappears when user disconnects

### Collab Overlays

- "X collaborators editing" badge in top-right corner of editor
- Flash animation on remote changes (subtle, 200ms)
- "Alice is typing..." tooltip near her cursor
- Selection highlight respects z-index so multiple selections don't overlap confusingly

### File Navigation

- Breadcrumbs below tab bar: `src > components > RoomPage.jsx`
- Tab bar shows open files as tabs
- Ctrl+P for quick file open (fuzzy search)
- Gutter shows line numbers, breakpoints, lint indicators

## Authentication Flow UX

```
+--------------------------------------------------+
|                  Landing Page                     |
|                                                   |
|       [Logo] Collaborative Code Editor            |
|                                                   |
|     ┌──────────────────┐  ┌──────────────────┐   |
|     │   Login           │  │   Register        │   |
|     │                  │  │                  │   |
|     │ Email: _______   │  │ Username: ______ │   |
|     │ Password: ______ │  │ Password: ______ │   |
|     │                  │  │ Confirm: ______  │   |
|     │ [  Login  ]      │  │ [Register]       │   |
|     └──────────────────┘  └──────────────────┘   |
|                                                   |
|     ───── or ─────                                |
|                                                   |
|     [Continue as Guest →]                          |
+--------------------------------------------------+
```

- Guest mode: auto-generates username, skips auth entirely
- Login form validates email format, password length client-side
- Error messages inline (red text below field)
- Remember-me checkbox for persistent sessions

### Room Creation/Joining

```
+--------------------------------------------------+
|        Create New Room                            |
|                                                   |
|  Room name:  [____________]                       |
|  Language:   [TypeScript ▼]                       |
|                                                   |
|  [Create Room]   [Cancel]                          |
+--------------------------------------------------+

+--------------------------------------------------+
|        Join Room                                  |
|                                                   |
|  Room code:  [____]                               |
|                                                   |
|  [Join]   [Cancel]                                |
+--------------------------------------------------+
```

- Room code: 6-character alphanumeric, displayed prominently on room page
- Language selector maps to editor syntax highlighting
- After creation, room code shown with copy button: `ABC123` [Copy]

## Lint Display UX

- Gutter indicators: red dot for error, yellow triangle for warning
- Underline in editor: wavy red for error, yellow for warning
- Hover on indicator: tooltip with issue description
- Click indicator: editor jumps to location, highlights problematic code
- Right panel "Lint" tab aggregates all issues for the room

## Accessibility

- All cursor colors meet WCAG AA contrast on dark background
- Keyboard shortcuts: Ctrl+K Ctrl+W (toggle panels), Ctrl+P (file search), Escape (close panels)
- Focus outlines visible on all interactive elements
- Screen reader labels on all icons in activity bar
