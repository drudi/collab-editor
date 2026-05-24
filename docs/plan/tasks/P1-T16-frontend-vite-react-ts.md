# P1-T16 — Frontend: Vite + React + TS + Dependencies

**Status:** Done

## Goal

Scaffold the frontend project and install all dependencies.

## Description

Create the frontend project using Vite with React and TypeScript template. Install all required dependencies for the collaborative code editor frontend:

**Core:**
- `react`, `react-dom` — UI framework

**Editor:**
- `@codemirror/commands` — standard commands
- `@codemirror/lang-javascript` — JS/TS syntax
- `@codemirror/lang-python` — Python syntax
- `@codemirror/lang-rust` — Rust syntax
- `@codemirror/language` — language extension API
- `@codemirror/lint` — linting integration
- `@codemirror/state` — editor state
- `@codemirror/view` — editor view

**Collaboration:**
- `yjs` — CRDT core
- `y-websocket` — WebSocket provider for Yjs

**Routing:**
- `react-router-dom` — client-side routing

## Acceptance Criteria

- AC1: `frontend/` directory created with Vite + React + TS template
- AC2: `frontend/package.json` has all listed dependencies
- AC3: `frontend/tsconfig.json` configured for React + JSX + ES modules
- AC4: `frontend/index.html` with root div
- AC5: `frontend/src/main.tsx` renders `<App />` to root
- AC6: `frontend/vite.config.ts` configured
- AC7: `npm run dev` starts the dev server without errors
- AC8: `npm run build` succeeds

## Technical Hints

- Use `npm create vite@latest frontend -- --template react-ts`
- CodeMirror 6 is a fully modular library — install only what you need
- y-websocket will be used later as the network provider for Yjs
- Refer to: https://codemirror.net/docs/
- Refer to: https://docs.yjs.dev/
