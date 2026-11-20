import { vi } from "vitest";

// Controllable stand-in for the generated `queen_sweep_wasm.js` module. The
// production build generates the real module into `src/wasm/`; tests alias that
// specifier to this file (see vitest.config.ts) so the service worker can be
// imported without first running `build:wasm`.
export const buildFromFlat = vi.fn();

export const SolverKind = { Dfs: 0 } as const;
export const HeuristicKind = { NoOp: 0, SmallestRegionFirst: 1 } as const;

const init = vi.fn();
export default init;
