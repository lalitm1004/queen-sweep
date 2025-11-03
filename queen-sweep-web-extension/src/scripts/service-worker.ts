import init, { QueensGame } from "../wasm/queen_sweep_wasm.js";
import { SolveRequest, SolveResponse, ERROR_CODES } from "./types/messages.type.js";

let wasmInitPromise: Promise<void> | null = null;
const ensureWasm = () => {
    if (!wasmInitPromise) {
        console.log("[QueenSweep] Initializing WASM...");
        wasmInitPromise = init().then(() => {
            console.log("[QueenSweep] WASM initialized");
        }).catch(err => {
            console.error("[QueenSweep] Failed to initialize WASM", err);
            wasmInitPromise = null;
            throw err;
        });
    }

    return wasmInitPromise;
}


const solveBoard = async (colorRegions: number[][]): Promise<SolveResponse> => {
    try {
        await ensureWasm();
    } catch {
        return {
            type: "solve-response-failure",
            success: false,
            code: ERROR_CODES.WASM_NOT_INITIALIZED
        };
    }

    const uint8arrays = colorRegions.map(r => new Uint8Array(r));

    let game: QueensGame;
    try {
        game = new QueensGame(uint8arrays);
    } catch (e) {
        return {
            type: 'solve-response-failure',
            success: false,
            code: ERROR_CODES.BOARD_INIT_FAILED,
            message: (e instanceof Error) ? e.message : String(e)
        };
    }

    const solved = game.solve();

    if (!solved) {
        return {
            type: 'solve-response-failure',
            success: false,
            code: ERROR_CODES.BOARD_UNSOLVABLE,
        };
    }

    const queenPositions: number[][] = [];
    for (const pair of solved.get_queen_positions()) {
        queenPositions.push([pair[0], pair[1]]);
    }

    return {
        type: 'solve-response-success',
        success: true,
        queenPositions
    };
}

chrome.runtime.onMessage.addListener((
    msg: SolveRequest,
    _,
    sendResponse: (res: SolveResponse) => void
) => {
    if (msg.type === "solve-request") {
        console.log('[QueenSweep] Recieved solveRequest:', msg);
        solveBoard(msg.colorRegions).then(res => {
            sendResponse(res)
            console.log('[QueenSweep] Send solveResponse:', msg);
        });
        return true;
    }
});
