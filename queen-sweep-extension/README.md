# queen-sweep-extension

A Chromium browser extension that integrates the QueenSweep solver directly into the Queens puzzle interface, providing one-click automated solving with visual feedback.

## Overview

This extension bridges the gap between the Rust/WASM solver core and the browser environment, enabling in-page puzzle detection, extraction, solving, and automated solution application. It operates on the [QueensGame website](https://queensgame.vercel.app) with full support for single-page application navigation.

> [!NOTE]
> This extension targets the open-source QueensGame implementation. LinkedIn's Terms of Service prohibit automated interaction with their platform, so the extension does not target LinkedIn directly.

## Architecture

The extension follows the Manifest V3 architecture with three distinct execution contexts.

### Service Worker (`src/scripts/service-worker.ts`)
**Execution context**: Background, persistent across page navigations

**Responsibilities**:
- WASM module lifecycle management (lazy initialization)
- Message handling for solve requests
- Puzzle solving via the WASM-compiled Rust engine
- Result serialization and transmission

**Key implementation details**:
```ts
let wasmInitPromise: Promise<void> | null = null;
```
- Singleton initialization pattern ensures WASM loads exactly once
- Failed initializations clear the promise to allow retry
- Module persists across page navigations, avoiding reload overhead
- The wasm binary is loaded from an explicit extension URL:
  ```ts
  await init(chrome.runtime.getURL("wasm/queen_sweep_wasm_bg.wasm"));
  ```

### Content Script (`src/scripts/content-script.ts`)
**Execution context**: Injected into `queensgame.vercel.app` pages

**Responsibilities**:
- DOM monitoring for puzzle board detection
- Color region extraction from rendered HTML
- UI injection (solve button)
- Solution application via synthetic pointer events
- SPA navigation tracking

**Key implementation details**:
- Uses `MutationObserver` for URL change detection (SPA support)
- Extracts puzzle state from `data-row`, `data-col`, and inline CSS
- Applies solutions via `PointerEvent` dispatch with proper timing

### Popup (`src/popup/`)
**Execution context**: Extension popup window

**Responsibilities**:
- User documentation
- Quick reference guide
- GitHub repository link

**Design**: Static informational UI (Tailwind), no runtime logic required.

## Data Flow

```text
┌─────────────────┐
│  Puzzle Page    │
│  (DOM)          │
└────────┬────────┘
         │ 1. Detect board
         │ 2. Extract colors
         ▼
┌─────────────────┐
│ Content Script  │
└────────┬────────┘
         │ 3. Send SolveRequest
         ▼
┌─────────────────┐
│ Service Worker  │
│  ┌───────────┐  │
│  │ WASM Init │  │ 4. Lazy load
│  └─────┬─────┘  │
│        │        │
│  ┌─────▼─────┐  │
│  │  Solver   │  │ 5. Compute solution
│  └─────┬─────┘  │
│        │        │
└────────┼────────┘
         │ 6. Return SolveResponse
         ▼
┌─────────────────┐
│ Content Script  │
└────────┬────────┘
         │ 7. Inject button
         │ 8. On click: apply solution
         ▼
┌─────────────────┐
│  Puzzle Page    │
│  (Updated)      │
└─────────────────┘
```

## Technical Implementation

### Puzzle Extraction

The content script extracts puzzle state by:

1. **Querying board cells**: `document.querySelectorAll('.square')`
2. **Reading position data**: `data-row` and `data-col` attributes
3. **Extracting colors**: parsing `background-color: rgb(...)` from inline styles
4. **Building color map**: assigning sequential indices to unique RGB values
5. **Constructing matrix**: 2D array of color indices

### Solution Application

Solutions are applied through synthetic DOM event dispatch:

```ts
const down = new PointerEvent("pointerdown", {
    bubbles: true,
    cancelable: true,
    clientX,
    clientY,
    button: 0,
    buttons: 1,
    pointerId: 1,
    pointerType: "mouse",
    isPrimary: true,
});
```

**Implementation considerations**:
- Uses `PointerEvent` instead of `MouseEvent` for modern compatibility
- Staggered timing (50ms between queens) for visual feedback
- Double-click simulation (down → up → down → up) for toggle behavior
- Proper event bubbling to trigger game logic

### SPA Navigation Handling

The target website is a single-page application, requiring URL monitoring:

```ts
const checkForUrlChange = () => {
    const currentUrl = location.href;
    if (currentUrl !== lastUrl) {
        if (isOnPuzzlePage()) {
            solvePuzzle();
        } else {
            removeSolveButton();
        }
    }
    lastUrl = currentUrl;
};

new MutationObserver(checkForUrlChange).observe(document, {
    subtree: true,
    childList: true,
});
```

**Strategy**:
- `MutationObserver` triggers on any DOM change
- URL comparison detects navigation without a page reload
- Pattern matching identifies puzzle pages (`/level/`, `/community-level/`, `/bonus-level/`)
- Automatic cleanup when navigating away from puzzles

### WASM Integration

The extension uses `vite-plugin-wasm` and `vite-plugin-top-level-await` for seamless WASM module loading.

**Build pipeline**:
1. `wasm-pack` compiles Rust → WASM + JS bindings into `src/wasm/`
2. Vite bundles the extension with WASM support
3. The service worker imports the module and initializes it from `chrome.runtime.getURL(...)`

**CSP configuration**:
```json
"content_security_policy": {
    "extension_pages": "script-src 'self' 'wasm-unsafe-eval'; object-src 'self';"
}
```
Required for WASM instantiation in Manifest V3.

## Message Protocol

### SolveRequest
```ts
{
    type: "solve-request",
    colorRegions: number[][]
}
```

### SolveResponse (Success)
```ts
{
    type: "solve-response-success",
    success: true,
    queenPositions: [number, number][],
    size: number,
    numStatesVisited: number
}
```

### SolveResponse (Failure)
```ts
{
    type: "solve-response-failure",
    success: false,
    code: ErrorCode,
    message?: string
}
```

Failure codes come from the wasm `BuildErrorCode` (`EMPTY`, `NON_SQUARE`,
`NOT_PERFECT_SQUARE`, `UNSUPPORTED_SIZE`, `INVALID_INPUT`, `BOARD_INIT_FAILED`)
plus the client-side `WASM_NOT_INITIALIZED` and `BOARD_UNSOLVABLE`.

## Installation

### Development Build

1. **Build the WASM module**
```bash
cd queen-sweep-extension
npm run build:wasm
```

2. **Install and build the extension**
```bash
npm i
npm run build
```

3. **Load in Chrome**
   - Navigate to `chrome://extensions/` (or `brave://extensions/`)
   - Enable "Developer mode"
   - Click "Load unpacked"
   - Select the `queen-sweep-extension/dist` directory

## Releases

Prebuilt `.zip` archives are published on the repository's [Releases](https://github.com/lalitm1004/queen-sweep/releases) page. Download the latest `queen-sweep-extension-v*.zip`, unzip it, and load the extracted folder as an unpacked extension. The same zip is suitable for uploading to the Chrome Web Store.

## Notes

- The solver supports board sizes `1` and `4..=18`; other sizes return `UNSUPPORTED_SIZE`.
- The target site is configurable via the `host_permissions` and `content_scripts.matches` entries in `src/manifest.json`.
