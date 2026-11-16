# queen-sweep-wasm

Thin, type-safe WebAssembly bindings for [`queen-sweep-core`](../queen-sweep-core). It wraps the runtime `dynamic` solver API with `wasm-bindgen` and generates TypeScript declarations via `tsify`, so the bindings are seamless to consume from TypeScript.

The crate is environment-agnostic (no DOM / `web-sys`), so it runs equally well in a Chrome extension service worker, an offscreen document, or a content script's isolated world.

## API

### Enums

```ts
export enum SolverKind { Dfs = 0 }
export enum HeuristicKind { NoOp = 0, SmallestRegionFirst = 1 }
export enum CellKind { Empty = 0, Blocked = 1, Queen = 2 }
```

### Input

```ts
export interface GridInput { regions: number[][] }
```

`buildFromGrid` takes a `GridInput` (nested arrays travel through serde). `buildFromFlat` takes a flat, row-major `number[]`/`Uint32Array` (no `GridInput` wrapper needed).

### Output

```ts
export interface SolveResult {
  size: number;
  numStatesVisited: number;
  solved: boolean;
  board: number[][] | undefined;   // values are CellKind
  queens: [number, number][];      // [row, col] pairs
}
```

### Errors

Both builders **throw** a structured `BuildError` on invalid input:

```ts
export type BuildErrorCode =
  | "EMPTY"
  | "NON_SQUARE"
  | "NOT_PERFECT_SQUARE"
  | "UNSUPPORTED_SIZE"
  | "INVALID_INPUT"
  | "BOARD_INIT_FAILED";

export interface BuildError {
  code: BuildErrorCode;
  message: string;
}
```

### Functions & class

```ts
export function buildFromGrid(grid: GridInput, solver: SolverKind, heuristic: HeuristicKind): WasmSolver;
export function buildFromFlat(flat: Uint32Array | number[], solver: SolverKind, heuristic: HeuristicKind): WasmSolver;

export class WasmSolver {
  solve(): SolveResult;
  free(): void;
}
```

`WasmSolver` is reusable — call `solve()` as many times as needed. Call `free()` (or use `using`/`Symbol.dispose`) when done.

## Build

```sh
wasm-pack build --target web
# or: --target bundler
```

This produces `pkg/` with `queen_sweep_wasm.js`, `queen_sweep_wasm_bg.wasm`, and `queen_sweep_wasm.d.ts`.

## Usage (TypeScript)

```ts
import init, {
  buildFromFlat,
  SolverKind,
  HeuristicKind,
  CellKind,
  type BuildError,
} from "./pkg/queen_sweep_wasm.js";

await init(); // or init(wasmUrl) for an explicit location

try {
  const solver = buildFromFlat(
    [0, 0, 1, 1, 0, 0, 1, 1, 2, 2, 3, 3, 2, 2, 3, 3],
    SolverKind.Dfs,
    HeuristicKind.SmallestRegionFirst,
  );

  const { solved, queens, board, size, numStatesVisited } = solver.solve();

  if (solved) {
    // queens: [row, col][]
    // board[r][c]: CellKind.Empty (0) | CellKind.Blocked (1) | CellKind.Queen (2)
    console.log(`Solved ${size}x${size} in ${numStatesVisited} states`, queens);
  }
} catch (e) {
  const err = e as BuildError;
  console.error(`build failed: ${err.code} — ${err.message}`);
}
```

## Chrome extension notes

- Manifest V3 requires `'wasm-unsafe-eval'` to compile WebAssembly:
  ```json
  "content_security_policy": {
    "extension_pages": "script-src 'self' 'wasm-unsafe-eval'; object-src 'self'"
  }
  ```
- Prefer running the solver in the **background service worker** or an **offscreen document**, then message results to the content script (page CSP / isolated-world restrictions can block Wasm).
- With `--target web`, initialize from the extension's own URL:
  ```ts
  await init(chrome.runtime.getURL("wasm/queen_sweep_wasm_bg.wasm"));
  ```

## Board size support

The underlying solver supports sizes `1` and `4..=18`. Other sizes throw `BuildError` with code `UNSUPPORTED_SIZE`.

## Testing

```sh
wasm-pack test --node
```
