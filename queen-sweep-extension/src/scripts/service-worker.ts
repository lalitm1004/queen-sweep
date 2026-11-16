import init, {
    buildFromFlat,
    HeuristicKind,
    SolverKind,
} from "../wasm/queen_sweep_wasm.js";
import type { BuildError } from "../wasm/queen_sweep_wasm.js";
import {
    SOLVER_ERROR_CODES,
    type ErrorCode,
    type SolveRequest,
    type SolveResponse,
} from "./types/messages.type";

const WASM_PATH = "wasm/queen_sweep_wasm_bg.wasm";

let wasmInitPromise: Promise<void> | null = null;

const ensureWasm = (): Promise<void> => {
    if (!wasmInitPromise) {
        console.log("[QueenSweep] Initializing WASM...");
        wasmInitPromise = init(chrome.runtime.getURL(WASM_PATH))
            .then(() => {
                console.log("[QueenSweep] WASM initialized");
            })
            .catch((err) => {
                console.error("[QueenSweep] Failed to initialize WASM", err);
                wasmInitPromise = null;
                throw err;
            });
    }

    return wasmInitPromise;
};

const solveBoard = async (colorRegions: number[][]): Promise<SolveResponse> => {
    try {
        await ensureWasm();
    } catch {
        return {
            type: "solve-response-failure",
            success: false,
            code: SOLVER_ERROR_CODES.WASM_NOT_INITIALIZED,
        };
    }

    let solver;
    try {
        solver = buildFromFlat(
            colorRegions.flat(),
            SolverKind.Dfs,
            HeuristicKind.SmallestRegionFirst,
        );
    } catch (e) {
        const err = e as Partial<BuildError>;
        return {
            type: "solve-response-failure",
            success: false,
            code: (err.code ?? SOLVER_ERROR_CODES.BOARD_INIT_FAILED) as ErrorCode,
            message: err.message,
        };
    }

    const result = solver.solve();

    if (!result.solved) {
        return {
            type: "solve-response-failure",
            success: false,
            code: SOLVER_ERROR_CODES.BOARD_UNSOLVABLE,
        };
    }

    return {
        type: "solve-response-success",
        success: true,
        queenPositions: result.queens,
        size: result.size,
        numStatesVisited: result.numStatesVisited,
    };
};

chrome.runtime.onMessage.addListener((
    msg: SolveRequest,
    _sender,
    sendResponse: (res: SolveResponse) => void,
) => {
    if (msg.type === "solve-request") {
        console.log("[QueenSweep] Received solve request:", msg);
        solveBoard(msg.colorRegions).then((res) => {
            console.log("[QueenSweep] Sending solve response:", res);
            sendResponse(res);
        });
        return true;
    }
});
