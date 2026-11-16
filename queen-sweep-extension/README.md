# queen-sweep-extension

A Chromium (Manifest V3) extension that injects a one-click solver into the
[QueensGame](https://queensgame.vercel.app) puzzle site, powered by the
`queen-sweep-wasm` crate.

> Built for the open-source QueensGame site. LinkedIn's Terms of Service prohibit
> automated interaction, so this extension does not target LinkedIn.

## Architecture

- **Service worker** (`src/scripts/service-worker.ts`): lazily initializes the
  WASM module (singleton promise, reset on failure), handles `solve-request`
  messages, builds a solver with `buildFromFlat(...)`, and returns the queen
  positions (or a typed error code).
- **Content script** (`src/scripts/content-script.ts`): detects puzzle pages,
  extracts the color grid from the DOM (`.square[data-row][data-col]` +
  `background-color: rgb(...)`), injects an **Apply Solution** button, applies
  the solution with synthetic `PointerEvent`s, and tracks SPA navigation.
- **Popup** (`src/popup/`): static informational UI.

## Message protocol

```ts
// request (content -> service worker)
{ type: "solve-request", colorRegions: number[][] }

// success
{ type: "solve-response-success", success: true, queenPositions: [number, number][], size, numStatesVisited }

// failure
{ type: "solve-response-failure", success: false, code, message? }
```

Error `code`s come from the wasm `BuildErrorCode` (`EMPTY`, `NON_SQUARE`,
`NOT_PERFECT_SQUARE`, `UNSUPPORTED_SIZE`, `INVALID_INPUT`,
`BOARD_INIT_FAILED`) plus the client-side `WASM_NOT_INITIALIZED` and
`BOARD_UNSOLVABLE`.

## Build

From this directory (needs `wasm-pack`, Rust with the `wasm32-unknown-unknown`
target, and Node.js — all provided by the repo's `nix develop` shell):

```sh
npm install
npm run build      # builds the wasm package, then the extension
```

The unpacked extension ends up in `dist/`.

Other scripts:
- `npm run build:wasm` — only rebuild the wasm package into `src/wasm/`
- `npm run build:js` — only rebuild the extension JS/CSS
- `npm run typecheck` — `tsc --noEmit`

## Load in Chrome

1. Go to `chrome://extensions/` (or `brave://extensions/`).
2. Enable **Developer mode**.
3. **Load unpacked** → select this `dist/` directory.

Then open a QueensGame level; an **Apply Solution** button appears above the
board.

## Notes

- The solver supports board sizes `1` and `4..=18`; other sizes return
  `UNSUPPORTED_SIZE`.
- Manifest V3 requires `'wasm-unsafe-eval'` in `extension_pages` CSP for
  WebAssembly instantiation.
- The wasm binary is loaded from `wasm/queen_sweep_wasm_bg.wasm` via
  `chrome.runtime.getURL(...)`.
