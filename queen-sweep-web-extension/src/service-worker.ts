import init, { QueensGame } from "./wasm/queen_sweep_wasm.js";
import type { SolveRequest, SolveResponse } from "./types/messages.type.js";


chrome.runtime.onInstalled.addListener(async () => {
    await init();
});

chrome.runtime.onMessage.addListener((
    msg: SolveRequest,
    _,
    sendResponse: (res: SolveResponse) => void
) => {
    if (msg.type === "solve-request") {
        const res = solveBoard(msg.colorRegions);
        sendResponse(res);
    }

    return true;
});

const solveBoard = (colorRegions: number[][]): SolveResponse => {
    const uint8arrays = colorRegions.map(r => new Uint8Array(r));

    try {
        const game = new QueensGame(uint8arrays);
        const solved = game.solve();

        if (solved) {
            const queenPositions: number[][] = [];

            for (const pair of solved.get_queen_positions()) {
                queenPositions.push([pair[0], pair[1]]);
            }

            return {
                type: 'solve-response-success',
                success: true,
                queenPositions
            };
        } else {
            return {
                type: 'solve-response-failure',
                success: false,
                message: 'Board has no solutions'
            }
        }
    } catch (e) {
        return {
            type: 'solve-response-failure',
            success: false,
            message: `Error initializing board: ${e}`
        }
    }
}