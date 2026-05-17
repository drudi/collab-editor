# P1-T17 — Frontend: Tailwind CSS Setup

**Status:** TODO

## Goal

Configure Tailwind CSS with PostCSS and define theme token variables.

## Description

Install and configure Tailwind CSS in the frontend project. Set up PostCSS as the build processor and create the base CSS file with Tailwind directives.

Create `frontend/tailwind.config.js` with:
- Content paths: `./src/**/*.{js,ts,jsx,tsx}`
- Dark mode: `class` strategy
- Custom theme extensions for editor colors

Create `frontend/postcss.config.js` with `tailwindcss` and `autoprefixer` plugins.

Create `frontend/src/index.css` with:
- `@tailwind base;`
- `@tailwind components;`
- `@tailwind utilities;`
- CSS custom properties for light/dark theme tokens

## Acceptance Criteria

- AC1: `tailwind.config.js` exists with correct content paths
- AC2: `postcss.config.js` exists with tailwindcss + autoprefixer
- AC3: `index.css` has all three `@tailwind` directives
- AC4: CSS custom properties defined for theme tokens (bg, text, border, accent)
- AC5: `npm run dev` compiles Tailwind without errors
- AC6: Tailwind utility classes work in a test component
- AC7: Dark mode class strategy enabled in config

## Technical Hints

- CSS custom properties pattern:
  ```css
  :root {
    --bg-primary: #ffffff;
    --bg-secondary: #f9fafb;
    --text-primary: #111827;
    --accent: #3b82f6;
  }
  .dark {
    --bg-primary: #111827;
    --bg-secondary: #1f2937;
    --text-primary: #f9fafb;
    --accent: #60a5fa;
  }
  ```
- Use `npm install -D tailwindcss postcss autoprefixer`
- Run `npx tailwindcss init -p` to generate config
- Refer to: https://tailwindcss.com/docs/installation
