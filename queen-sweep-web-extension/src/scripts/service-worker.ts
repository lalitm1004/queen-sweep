import init, { QueensGame } from "../wasm/queen_sweep_wasm.js";
import { SolveRequest, SolveResponse, ERROR_CODES } from "./types/messages.type.js";

let wasmReady: boolean = false;
chrome.runtime.onInstalled.addListener(async () => {
    try {
        await init();
        wasmReady = true;
        console.log('[QueenSweep] WASM initialized successfully');
    } catch (e) {
        console.error(`[QueenSweep] Failed to initialize WASM:`, e);
    }
});

chrome.runtime.onMessage.addListener((
    msg: SolveRequest,
    _,
    sendResponse: (res: SolveResponse) => void
) => {
    if (msg.type === "solve-request") {
        const res = solveBoard(msg.colorRegions);
        sendResponse(res);
        return true;
    }
});

const solveBoard = (colorRegions: number[][]): SolveResponse => {
    if (!wasmReady) {
        console.warn('[QueenSweep] solveBoard called before WASM initialization');
        return {
            type: 'solve-response-failure',
            success: false,
            code: ERROR_CODES.WASM_NOT_INITIALIZED,
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