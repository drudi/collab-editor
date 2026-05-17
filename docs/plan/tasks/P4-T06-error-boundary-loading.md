# P4-T06 — Error Boundary + Loading States

**Status:** TODO

## Goal

Create error boundary and loading spinner components for resilient UI.

## Description

Create two UI components that make the application more resilient and user-friendly: error handling for React errors and loading indicators for async states.

**ErrorBoundary component** (`src/components/ErrorBoundary.tsx`):
- Wraps the app (or individual page components) in try-catch for render errors
- Shows a fallback UI with error message and "Try again" button
- Resets error state when "Try again" is clicked
- Logs error to console for debugging

```tsx
function ErrorBoundary({ children }: { children: React.ReactNode }) {
  const [error, setError] = useState<Error | null>(null);
  
  // useEffect with getDerivedStateFromError pattern
  
  if (error) {
    return (
      <div className="flex items-center justify-center h-screen">
        <div className="text-center">
          <h2 className="text-xl font-semibold">Something went wrong</h2>
          <p className="text-gray-400 mt-2">{error.message}</p>
          <button onClick={() => setError(null)} className="mt-4 ...">
            Try again
          </button>
        </div>
      </div>
    );
  }
  
  return <>{children}</>;
}
```

**LoadingSpinner component** (`src/components/LoadingSpinner.tsx`):
- Centered spinner for loading states
- Supports labeled variant: "Loading rooms...", "Connecting..."
- Uses Tailwind animation utilities

```tsx
function LoadingSpinner({ label }: { label?: string }) {
  return (
    <div className="flex flex-col items-center justify-center h-screen">
      <div className="animate-spin w-8 h-8 border-4 border-blue-400 border-t-transparent rounded-full" />
      {label && <p className="text-gray-400 mt-4">{label}</p>}
    </div>
  );
}
```

## Acceptance Criteria

- AC1: `ErrorBoundary` component exists
- AC2: Catches React render errors via error boundary mechanism
- AC3: Shows fallback UI with error message and "Try again" button
- AC4: "Try again" resets error state and re-renders children
- AC5: `LoadingSpinner` component exists
- AC6: Spinner animation with Tailwind `animate-spin`
- AC7: Optional label text displayed below spinner
- AC8: ErrorBoundary wraps app in `main.tsx`
- AC9: LoadingSpinner used in RoomPage for connection state
- AC10: No lint errors
- AC11: Tailwind-styled with dark mode support

## Technical Hints

- Error boundary in React 18+ with hooks: use `useErrorBoundary` pattern
- Use `getDerivedStateFromError` via a class component or the `react-error-boundary` library
- Simple approach: use `useEffect` + try-catch in component mount
- Spinner: `border-{color} border-t-transparent rounded-full animate-spin`
- Tailwind colors: `border-blue-400`, `text-gray-400`, `bg-gray-900`
- Refer to: https://react.dev/reference/react/components#error-boundaries
- Refer to: https://tailwindcss.com/docs/animation
