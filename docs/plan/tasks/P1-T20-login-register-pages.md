# P1-T20 — Frontend: Login + Register Pages

**Status:** TODO

## Goal

Implement styled login and register pages with form validation.

## Description

Create `src/pages/LoginPage.tsx` and `src/pages/RegisterPage.tsx` with Tailwind-styled forms. Extract a shared `AuthForm` component to avoid duplication.

`AuthForm` props:
- `title: string` — form heading
- `mode: 'login' | 'register'`
- `onSubmit(username: string, password: string): Promise<void>`
- `error: string | null` — validation or server error

Each form has:
- Username input (3-32 chars, alphanumeric + underscore)
- Password input (minimum 6 characters)
- Submit button
- Inline error display
- Link to switch between login/register

`LoginPage` uses the auth context's `login` function.
`RegisterPage` uses the auth context's `register` function.
Both navigate to `/` on success.

## Acceptance Criteria

- AC1: `AuthForm` component exists as shared base
- AC2: `LoginPage` renders with title "Login"
- AC3: `RegisterPage` renders with title "Register"
- AC4: Username validation: 3-32 chars, alphanumeric + underscore
- AC5: Password validation: minimum 6 characters
- AC6: Inline error display for validation and server errors
- AC7: Login/register calls backend API via auth context
- AC8: Navigates to `/` on successful authentication
- AC9: Links to switch between login/register pages
- AC10: Tailwind-styled with dark mode support

## Technical Hints

- Use `input type="password"` for password field
- Real-time validation with `onChange` handlers
- Pattern for validation:
  ```typescript
  const validate = (username, password) => {
    if (username.length < 3) return 'Username must be at least 3 characters';
    if (!/^[a-zA-Z0-9_]+$/.test(username)) return 'Username must be alphanumeric';
    if (password.length < 6) return 'Password must be at least 6 characters';
    return null;
  };
  ```
- Use `useAuth().navigate('/');` for post-login redirect
- Tailwind: `bg-gray-900`, `text-white`, `rounded-lg`, `px-4 py-2`, `hover:bg-gray-800`
- Refer to: https://tailwindcss.com/docs/utility-first
