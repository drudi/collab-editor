# P4-T02 — Auto-Save Indicator

**Status:** TODO

## Goal

Create an auto-save status indicator in the editor UI.

## Description

Create `src/components/AutoSaveIndicator.tsx` — a small UI component that shows the current save status of the collaborative document. It provides visual feedback about save state to users.

States:
- "Unsaved" — document has local changes not yet persisted (red/orange dot)
- "Saving..." — save in progress (spinning icon)
- "Saved" — last save completed successfully (green checkmark)

The component is connected to the save lifecycle:
1. On local edit: show "Unsaved"
2. Before save: show "Saving..."
3. On save confirm from server: show "Saved" for 2 seconds, then fade back to "Unsaved"

Position: bottom-right of the editor area, small and unobtrusive.

## Acceptance Criteria

- AC1: `AutoSaveIndicator` component exists
- AC2: Shows three states: "Unsaved", "Saving...", "Saved"
- AC3: "Unsaved" shows red/orange dot icon
- AC4: "Saving..." shows spinning/loading icon
- AC5: "Saved" shows green checkmark
- AC6: Auto-fades from "Saved" back to "Unsaved" after 2 seconds
- AC7: Position: bottom-right of editor area
- AC8: Small, unobtrusive styling
- AC9: Tailwind-styled with dark mode support
- AC10: No lint errors

## Technical Hints

- Component structure:
  ```tsx
  function AutoSaveIndicator({ status }: { status: 'unsaved' | 'saving' | 'saved' }) {
    const [displayed, setDisplayed] = useState(status);
    
    useEffect(() => {
      if (status === 'saved') {
        const t = setTimeout(() => setDisplayed('unsaved'), 2000);
        return () => clearTimeout(t);
      }
      setDisplayed(status);
    }, [status]);
    
    // ... render based on displayed status
  }
  ```
- Icons: use Unicode or simple SVG dots/circles
- Position: `position: 'absolute'`, `bottom: '1rem'`, `right: '1rem'`
- Fade transition: `transition: opacity 300ms ease`
- Tailwind: `text-green-400`, `text-amber-400`, `animate-spin`
- Refer to: https://tailwindcss.com/docs/animation
