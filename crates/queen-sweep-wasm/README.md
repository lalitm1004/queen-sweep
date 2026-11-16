# queen-sweep-wasm

Thin, type-safe WebAssembly bindings for [`queen-sweep-core`](../queen-sweep-core).
Exposes the runtime dynamic solver to JavaScript/TypeScript with generated
`.d.ts` declarations.

## Build

```sh
wasm-pack build --target web --out-dir pkg
# or: --target bundler
```

This produces `pkg/` with `queen_sweep_wasm.js`, `queen_sweep_wasm_bg.wasm`, and
`queen_sweep_wasm.d.ts`.

## Usage (TypeScript)

```ts
import init, {
  buildFromGrid,
  buildFromFlat,
  SolverKind,
  HeuristicKind,
  CellKind,
} from "./pkg/queen_sweep_wasm.js";

await init(); // or init(wasmUrl) in a Chrome extension

const solver = buildFromFlat(
  [0, 0, 1, 1, 0, 0, 1, 1, 2, 2, 3, 3, 2, 2, 3, 3],
  SolverKind.Dfs,
  HeuristicKind.SmallestRegionFirst,
);

const { solved, board, size, numStatesVisited } = solver.solve();
// board[r][c] is CellKind.Empty (0) | CellKind.Blocked (1) | CellKind.Queen (2)
```

`buildFromGrid` takes `{ regions: number[][] }` (nested arrays travel through
serde, so they are wrapped in an object rather than passed as a bare array).

Both builders **throw** a JavaScript `Error` on malformed input or an
unsupported board size.

## Chrome extension notes

WebAssembly is a great fit for an extension that solves a game in the page:

- Manifest V3 requires `'wasm-unsafe-eval'` to compile WebAssembly:

  ```json
  "content_security_policy": {
    "extension_pages": "script-src 'self' 'wasm-unsafe-eval'; object-src 'self'"
  }
  ```

- Prefer running the solver in the **background service worker** or an
  **offscreen document** and messaging results to the content script. Content
  scripts run under page CSP / isolated-world restrictions that can block Wasm.
- With `--target web`, initialize from the extension's own URL:

  ```ts
  await init(chrome.runtime.getURL("queen_sweep_wasm_bg.wasm"));
  ```

- The crate uses no DOM / `web-sys` APIs, so it runs in a service worker,
  offscreen document, content-script isolated world, or a normal page.

## Tests

```sh
wasm-pack test --node
```
