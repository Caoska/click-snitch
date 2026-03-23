# ClickSnitch

ClickSnitch is a **minimal product analytics tool** built with **Rust** and **TypeScript**.  
It automatically tracks user interactions (button/link clicks and page views) and sends them to a Rust backend.  

Designed as a **portfolio-ready MVP**, it demonstrates a full Rust backend + JS/TS integration with a live demo.

## Features

- Automatically tracks `<button>` and `<a>` clicks  
- Tracks page views  
- Easy drop-in NPM package  
- Rust backend with JSONL storage  
- Optional live demo page

## Repo Structure

clicksnitch/
├─ backend/ ← Rust server
│ ├─ .gitignore
│ ├─ Cargo.toml
│ ├─ Cargo.lock
│ ├─ src/.gitignore
│ ├─ src/main.rs
│ └─ data.jsonl ← created on first run
├─ package/ ← NPM package (TS/JS client)
│ ├─ .gitignore
│ ├─ package.json
│ ├─ package-lock.json
│ ├─ tsconfig.json
│ └─ src/index.ts
├─ demo/ ← Vite demo page
│ ├─ .gitignore
│ ├─ index.html
│ ├─ main.ts
│ ├─ package-lock.json
│ ├─ package.json
│ └─ tsconfig.json
└─ README.md


## 1. Backend (Rust)

### Setup

```bash
cd backend
cargo run
```

- Runs the Rust server on http://localhost:3000
- /collect accepts POST events from front-end
- /events returns all stored events

### Storage
- Events are stored in data.jsonl (JSON Lines format) for MVP simplicity

### Notes
- CORS enabled for local development (localhost:5173 or any origin)
- For production, restrict origins as needed

## 2. NPM Package (TypeScript/JS client)

### Build

```bash
cd package
npm install
npx tsc
```

- Generates dist/index.js and dist/index.d.ts
- Link locally for demo/testing

```bash
npm link           # in package folder
cd ../demo
npm link clicksnitch
```

This allows the demo (or any local project) to import the package without publishing to NPM yet.

### Usage
```ts
import { initClickSnitch } from "clicksnitch";

initClickSnitch({
  serverUrl: "http://localhost:3000/collect" // optional, defaults to localhost
});
```

- Automatically tracks clicks and page views
- Sends events to your Rust backend

## 3. Demo (Vite-based)

### Setup

```bash
cd demo
npm install
npm run dev
```

- Opens Vite dev server (usually http://localhost:5173)
- Loads main.ts which imports your local clicksnitch package
- Shows live events in the browser

### Demo page
- Contains buttons and a link:
```html
<button id="signup">Sign Up</button>
<button id="login">Login</button>
<a href="#" id="learn">Learn More</a>
```

- <ul id="events"></ul> shows live events fetched from the backend
- Events update every second

## 4. Testing Flow

1. Start Rust backend:
```bash
cd backend
cargo run
```

2. Start demo Vite server:
```bash
cd demo
npm run dev
```

3. Open Vite dev URL in browser (http://localhost:5173)
4. Click buttons/links → events appear in live list
5. Open /events endpoint in backend to see raw JSONL storage

## 5. Notes / Tips
- Each developer runs their own backend → keeps data private
- JSONL storage is simple; can migrate to Postgres/SQLite later
- Default server URL is http://localhost:3000/collect but can be overridden
- Shows Rust async backend, TS integration, and front-end analytics