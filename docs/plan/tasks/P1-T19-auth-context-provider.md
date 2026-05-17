# P1-T19 — Frontend: Auth Context + Provider

**Status:** TODO

## Goal

Create the authentication context and provider for managing user session state.

## Description

Create `src/contexts/AuthContext.tsx` with a React context that manages authentication state across the application. The context provides login, register, and logout functions that call the backend API endpoints.

The context value includes:
- `user: User | null` — current authenticated user
- `isLoading: boolean` — loading state for auth operations
- `login(username: string, password: string): Promise<void>` — calls POST /api/auth/login
- `register(username: string, password: string): Promise<void>` — calls POST /api/auth/register
- `logout(): Promise<void>` — calls POST /api/auth/logout
- `isAuthenticated: boolean` — derived from user state

The provider wraps the app and manages state with `useState` and `useEffect`. Login/register calls the backend API — the session cookie is handled automatically by the browser. After successful login, refetch user data.

## Acceptance Criteria

- AC1: `AuthContext.tsx` created with `createContext<AuthContextType | null>(null)`
- AC2: `AuthProvider` component wraps children with `useContext`
- AC3: `login` function calls `POST /api/auth/login` with fetch
- AC4: `register` function calls `POST /api/auth/register` with fetch
- AC5: `logout` function calls `POST /api/auth/logout` with fetch
- AC6: `isLoading` state managed during API calls
- AC7: `isAuthenticated` derived from `user !== null`
- AC8: `useAuth()` hook for consuming the context with null check
- AC9: AuthProvider wrapped around app in `main.tsx`
- AC10: Type-safe context with TypeScript interfaces

## Technical Hints

- Use `fetch` with `{ credentials: 'include' }` to send cookies
- Error handling: catch fetch errors, set appropriate error state
- Pattern:
  ```typescript
  const authContext = useContext(AuthContext);
  if (!authContext) throw new Error('useAuth must be used within AuthProvider');
  return authContext;
  ```
- Use `useCallback` for login/register/logout to prevent unnecessary re-renders
- Refer to: https://react.dev/reference/react/createContext
