# queen-sweep-extension

A Chromium (Manifest V3) extension that integrates the `queen-sweep-wasm` solver directly into the [QueensGame](https://queensgame.vercel.app) puzzle interface, providing one-click automated solving with visual feedback.

> [!NOTE]
> LinkedIn's Terms of Service prohibit automated interaction with its platform, so this extension only targets the open-source QueensGame site.

## Overview

The extension bridges the Rust/WebAssembly solver and the browser:

1. detects puzzle pages (including SPA navigation),
2. extracts the color-region grid from the DOM,
3. solves it in the background via WASM,
4. injects an **Apply Solution** button, and
5. applies the solution with synthetic pointer events.

## Architecture

Manifest V3 with three execution contexts:

### Service worker (`src/scripts/service-worker.ts`)
- Lazily initializes the WASM module (singleton promise, reset on failure so it can retry).
- Handles `solve-request` messages, builds a solver with `buildFromFlat(...)`, and replies with queen positions or a typed error code.
- Persists across page navigations.

### Content script (`src/scripts/content-script.ts`)
- Injected into `queensgame.vercel.app` pages.
- Detects puzzle pages and SPA navigation (`src/scripts/url-monitor.ts`).
- Extracts the color grid (`src/scripts/puzzle/extractor.ts`).
- Injects/updates the solve button (`src/scripts/button.ts`).
- Applies the solution via synthetic `PointerEvent`s (`src/scripts/puzzle/applier.ts`).

### Popup (`src/popup/`)
- Static informational UI (Tailwind), no runtime logic.

## Data flow

```text
 Puzzle page (DOM)
   │  1. detect board + extract colors
   ▼
 Content script ──2. SolveRequest──► Service worker
                                      │  3. lazy init WASM
                                      │  4. buildFromFlat(...) + solve()
   ◄──5. SolveResponse (queens)──────┘
   │  6. inject "Apply Solution" button
   │  7. on click: dispatch PointerEvents
   ▼
 Puzzle page (updated)
```

## Puzzle extraction

1. `document.querySelectorAll('.square')`
2. read `data-row` / `data-col`
3. parse `background-color: rgb(...)` from inline styles
4. map unique RGB values to sequential color indices
5. build a 2D `number[][]` region matrix

## Solution application

Queens are applied by dispatching synthetic pointer events (a double down/up per cell) at each cell's center, staggered by 50 ms for visual feedback:

```ts
new PointerEvent("pointerdown", { bubbles: true, cancelable: true, clientX, clientY, button: 0, buttons: 1, pointerId: 1, pointerType: "mouse", isPrimary: true });
```

## SPA navigation

The target site is a single-page app, so the content script watches the DOM with a `MutationObserver` and compares `location.href` to react to navigation between `/level/`, `/community-level/`, and `/bonus-level/` pages.

## WASM integration

- `wasm-pack build --target web` emits the module into `src/wasm/`.
- Vite (`vite-plugin-wasm`, `vite-plugin-top-level-await`) bundles the extension; `vite-plugin-static-copy` copies the manifest, wasm, and assets into `dist/`.
- The service worker initializes the module from an explicit URL:
  ```ts
  await init(chrome.runtime.getURL("wasm/queen_sweep_wasm_bg.wasm"));
  ```
- Manifest V3 requires `'wasm-unsafe-eval'` in the extension CSP:
  ```json
  "content_security_policy": {
    "extension_pages": "script-src 'self' 'wasm-unsafe-eval'; object-src 'self'"
  }
  ```

## Message protocol

```ts
// request (content -> service worker)
{ type: "solve-request", colorRegions: number[][] }

// success
{ type: "solve-response-success", success: true, queenPositions: [number, number][], size, numStatesVisited }

// failure
{ type: "solve-response-failure", success: false, code, message? }
```

Error `code`s come from the wasm `BuildErrorCode` (`EMPTY`, `NON_SQUARE`, `NOT_PERFECT_SQUARE`, `UNSUPPORTED_SIZE`, `INVALID_INPUT`, `BOARD_INIT_FAILED`) plus the client-side `WASM_NOT_INITIALIZED` and `BOARD_UNSOLVABLE`.

## Build

From this directory (needs `wasm-pack`, Rust with the `wasm32-unknown-unknown` target, and Node.js — all provided by the repo's `nix develop` shell):

```sh
npm install
npm run build      # builds the wasm package, then the extension
```

The unpacked extension is written to `dist/`.

Other scripts:
- `npm run build:wasm` — rebuild only the wasm package into `src/wasm/`
- `npm run build:js` — rebuild only the extension JS/CSS
- `npm run typecheck` — `tsc --noEmit`

## Load in Chrome

1. Go to `chrome://extensions/` (or `brave://extensions/`).
2. Enable **Developer mode**.
3. **Load unpacked** → select this `dist/` directory.

Then open a QueensGame level; an **Apply Solution** button appears above the board.

## Releases

Prebuilt `.zip` downloads are published on the repository's [Releases](https://github.com/lalitm1004/queen-sweep/releases) page. Download the latest `queen-sweep-extension-v*.zip`, unzip it, and load the extracted folder as an unpacked extension. The same zip is suitable for uploading to the Chrome Web Store.

## Notes

- The solver supports board sizes `1` and `4..=18`; other sizes return `UNSUPPORTED_SIZE`.
- The extension is intentionally limited to the open-source QueensGame site (see the note above).
