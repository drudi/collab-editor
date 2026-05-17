# UX Design Option 3 — Pair Programming Focused

## Design Philosophy

Optimized for two-person pair programming. Split-screen editor with integrated chat and presence. Inspired by Replit's pair coding interface and GitHub's copilot layout.

## Color Palette

| Token | Hex | Usage |
|-------|-----|-------|
| Background | `#0d1117` | Main background |
| Surface | `#161b22` | Panels, chat |
| Surface Light | `#1c2333` | Input fields, modals |
| Border | `#30363d` | Dividers, borders |
| Border Light | `#484f58` | Subtle borders |
| Text | `#c9d1d9` | Primary text |
| Text Secondary | `#8b949e` | Labels, timestamps |
| Accent | `#58a6ff` | Links, active elements |
| Accent Bright | `#79c0ff` | Hover states |
| Cursor Alice | `#ff7b72` | Alice's cursor (pink-red) |
| Cursor Bob | `#79c0ff` | Bob's cursor (blue) |
| Cursor Charlie | `#d2a8ff` | Charlie's cursor (purple) |No diagram type detected matching given configuration for text: 
| Error | `#ff7b72` | Lint errors |
| Warning | `#d29922` | Lint warnings |
| Success | `#3fb950` | Save, joins |
| Code Comment | `#8b949e` | Comment color |
| Code Keyword | `#ff7b72` | Keyword highlight |
| Code String | `#a5d6ff` | String highlight |

## Layout — Room Page (1920x1080)

### Default: Split Editor

```
+--------------------------------------------------------------------------+
| [Logo] CollabEdit     abc123    [🟢 Alice] [🟢 Bob]   [Settings]       |
+--------------------------+-----------------------------------------------+
|                          |                                               |
|  LEFT EDITOR (Alice)     |         RIGHT EDITOR (Bob)                    |
|  50% width               |         50% width                             |
|                          |                                               |
|  +---------------------+ |  +-----------------------------------------+   |
|  | RoomPage.tsx        | |  | RoomPage.tsx                            |   |
|  |---------------------| |  |-----------------------------------------|   |
|  | 1  import React     | |  | 1  import React                        |   |
|  | 2  from 'react'     | |  | 2  from 'react'                        |   |
|  | 3                   | |  | 3                                      |   |
|  | 4  function Room()  | |  | 4  function Room() {                    |   |
|  | 5    return (        | |  | 5      return (                        |   |
|  | 6 >      <Editor />  | |  | 6 >      <Editor />                    |   |
|  | 7    )              | |  | 7      )                                |   |
|  | 8  )                | |  | 8    )                                  |   |
|  | 9  }                | |  | 9  }                                   |   |
|  |                     | |  | 10                                     |   |
|  |                     | |  | 11                                     |   |
|  +---------------------+ |  +-----------------------------------------+   |
|  | Alice is typing...  | |  | Bob is viewing                           |   |
|  | Cursor: line 6 col 4| |  | Cursor: line 4 col 15                    |   |
|  +---------------------+ |  +-----------------------------------------+   |
|                          |                                               |
+--------------------------+-----------------------------------------------+
| Chat (bottom panel, 180px)                                               |
| +---------------------------------------------------------------------+  |
| | [🟢 Alice] hello! ready to start on the room component?             |  |
| | [🟢 Bob]     sure, let me look at it first                          |  |
| | [🟢 Alice]     ok, it's in src/components/RoomPage.tsx              |  |
| | > [Type a message...                              ]  [Send]          |  |
| +---------------------------------------------------------------------+  |
+--------------------------------------------------------------------------+
```

### Collapsed: Single Editor

```
+--------------------------------------------------------------------------+
| [Logo] CollabEdit     abc123    [🟢 Alice] [🟢 Bob]   [Split▼] [Chat▼]  |
+--------------------------------------------------------------------------+
|                          |                                               |
|  EDITOR                  |         RIGHT SIDEBAR                         |
|  ~70% width              |         30% width                             |
|                          |                                               |
|  +---------------------+ |  +-----------------------------------------+   |
|  | RoomPage.tsx        | |  | Collaborators                           |   |
|  |---------------------| |  | --------------------------------------- |   |
|  | 1  import React     | |  |                                         |   |
|  | 2  from 'react'     | |  | [🟢] Alice  @line 6                     |   |
|  | 3                   | |  | [🟢] Bob    @line 12                    |   |
|  | 4  function Room()  | |  |                                         |   |
|  | 5    return (        | |  | Room Info                               |   |
|  | 6 >      <Editor />  | |  | --------------------------------------- |   |
|  | 7    )              | |  | Code: abc123  [Copy]                    |   |
|  | 8  )                | |  | Lang: TypeScript                        |   |
|  | 9  }                | |  | Members: 2                              |   |
|  |                     | |  | Created: today                          |   |
|  |                     | |  |                                         |   |
|  +---------------------+ |  | Lint                                    |   |
|                          |  | --------------------------------------- |   |
|                          |  | RoomPage.tsx                            |   |
|                          |  | - Line 4: Unused import                 |   |
|                          |  | - Line 7: Missing return type           |   |
|                          |  +-----------------------------------------+   |
+--------------------------+-----------------------------------------------+
|  Files: 1  |  Lines: 9  |  Saved  |  2 online                            |
+--------------------------------------------------------------------------+
```

## Key UX Decisions

### Split Editor

- Default split 50/50 for two collaborators
- Draggable divider between editors
- Each editor independently scrollable
- Each editor shows the same file with independent cursor/selection
- Each editor has its own language mode (can differ)
- "Mirror scroll" toggle: when enabled, scrolling one editor scrolls both
- Split can be horizontal or vertical (toggle in toolbar)
- On 3+ users: shared single editor + right panel

### Presence Strip

- Below each editor: status bar showing who's active
- "Alice is typing..." / "Bob is viewing" with cursor position
- Color-coded to match user's cursor color
- Click status → jump to their cursor

### Integrated Chat — Bottom Panel

- Fixed bottom panel (collapsible to icon)
- Chat bubbles with colored left border per user
- Timestamp on hover
- Quick actions: @mention, link to code line (`src/file.ts:42`)
- Auto-scroll on new messages
- "Code snippet" button: paste inline code block

### Right Panel — Three Tabs

**Tab 1: Collaborators**
- Online users with colored circle + username
- "Line N" next to cursor position
- Click to jump to their cursor
- "Request edit" button for viewers

**Tab 2: Room Info**
- Code, language, member count, created date
- "Copy invite link" button
- "Leave room" at bottom

**Tab 3: Lint**
- File-level issue summary
- Click to jump to issue in editor
- Filter by severity

### Code Navigation

- Each editor has its own tab bar, file tree, minimap
- Independent zoom level per editor
- Independent theme per editor (both dark or light)

### Chat Integration

- Chat messages can link to code lines: `[src/file.ts:42](room:abc123:line:42)`
- Click link → editor on the right scrolls to that line
- Inline code blocks with syntax highlighting
- "Pin message" for important instructions
- "Start discussion" button: creates a threaded conversation

### Room Code

- Large, prominent display in top toolbar
- Click to copy
- Invite modal with room code, copyable link, and QR code (for mobile sharing)

## Authentication Flow UX

```
+--------------------------------------------------+
|        Join a Room                                |
|                                                   |
|  You can jump right in. Choose how to continue:   |
|                                                   |
|  ┌─────────────────────────────────────────────┐  |
|  |  👤 Sign in / Create account                 |  |
|  |                                              |  |
|  |  Email: [____________]                       |  |
|  |  Password: [____________]                    |  |
|  |                                              |  |
|  |  [  Sign In  ]                               |  |
|  └─────────────────────────────────────────────┘  |
|                                                   |
|  ─── or ───                                       |
|                                                   |
|  ┌─────────────────────────────────────────────┐  |
|  |  🚀 Continue as Guest                        |  |
|  |                                              |  |
|  |  Name: [____________]                        |  |
|  |                                              |  |
|  |  [  Continue  ]                              |  |
|  └─────────────────────────────────────────────┘  |
|                                                   |
|  Guest accounts don't save history.               |
|  [Sign in to save your work →]                    |
+--------------------------------------------------+
```

- Two-panel approach: sign in on left, guest on right
- Guest name input required (not optional)
- After guest sign-in: toast "Welcome, Guest! Sign in to save your rooms"
- Guest can upgrade to account anytime from settings

### Room Creation

```
+--------------------------------------------------+
|        Create a Room                              |
|                                                   |
|  Room name:  [___________________]                |
|  Language:   [TypeScript ▼]                       |
|  Split mode: [50/50 ▼]  [Vertical ▼]             |
|                                                   |
|  Description (optional):                          |
|  [_________________________________]               |
|  [_________________________________]               |
|                                                   |
|  [Create Room]                                    |
+--------------------------------------------------+
```

## Responsive Breakpoints

| Screen | Behavior |
|--------|----------|
| > 1400px | Full split editor + chat panel |
| 1000-1400px | Split editor; chat collapses to icon, expands on click |
| < 1000px | Single editor; split toggle; chat as bottom sheet |

## Accessibility

- Split divider is keyboard resizable (arrow keys)
- Chat panel: full keyboard nav, tab through messages and input
- Color-only indicators supplemented with text labels
- All cursor colors meet WCAG AA on dark background
- "Reduce motion" toggle in settings
