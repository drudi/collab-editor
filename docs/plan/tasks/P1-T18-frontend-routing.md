# P1-T18 — Frontend: Routing Setup

**Status:** TODO

## Goal

Set up React Router with protected routes and page components.

## Description

Configure `react-router-dom` v6 for client-side routing. Create the route structure and a protected route wrapper.

Create `src/pages/` directory with placeholder components:
- `HomePage.tsx` — room browser
- `LoginPage.tsx` — authentication login
- `RegisterPage.tsx` — authentication registration
- `RoomPage.tsx` — collaborative editor

Create `src/components/ProtectedRoute.tsx` that:
- Checks auth context for valid session
- Redirects to `/login` if not authenticated
- Renders children if authenticated

Configure routes in `App.tsx`:
- `/` → HomePage
- `/login` → LoginPage
- `/register` → RegisterPage
- `/room/:id` → RoomPage (protected)

## Acceptance Criteria

- AC1: `react-router-dom` v6 installed
- AC2: `App.tsx` wraps app in `BrowserRouter`
- AC3: Four page components exist in `src/pages/`
- AC4: `ProtectedRoute.tsx` component checks auth context
- AC5: `/room/:id` route uses `ProtectedRoute` wrapper
- AC6: Unauthenticated redirect to `/login` works
- AC7: All routes render their page components correctly
- AC8: `:id` param accessible via `useParams()` in RoomPage

## Technical Hints

- Use `createBrowserRouter` or `BrowserRouter` + `Route` components
- `ProtectedRoute` pattern:
  ```tsx
  function ProtectedRoute({ children }: { children: React.ReactNode }) {
    const { user } = useAuth();
    if (!user) return <Navigate to="/login" replace />;
    return <>{children}</>;
  }
  ```
- Use `Navigate`, `useParams`, `useNavigate` from `react-router-dom`
- Refer to: https://reactrouter.com/en/main
