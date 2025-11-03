import { SolveRequest, SolveResponse } from "./types/messages.type";

const extractColorRegions = (): number[][] => {
    // dummy board to test with
    const colorRegions: number[][] = [
        [0, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2],
        [0, 3, 3, 1, 1, 1, 2, 2, 2, 2, 2],
        [0, 3, 3, 1, 1, 2, 2, 2, 2, 2, 2],
        [0, 0, 0, 0, 2, 2, 2, 4, 4, 4, 4],
        [0, 5, 5, 6, 7, 7, 2, 2, 2, 4, 4],
        [0, 5, 5, 6, 7, 7, 7, 7, 7, 7, 4],
        [0, 5, 5, 6, 7, 7, 7, 7, 7, 7, 4],
        [0, 5, 5, 6, 7, 7, 8, 9, 7, 7, 4],
        [0, 5, 5, 6, 7, 7, 8, 9, 7, 7, 8],
        [10, 5, 5, 6, 7, 7, 8, 9, 7, 7, 8],
        [10, 10, 10, 6, 6, 8, 8, 8, 8, 8, 8],
    ];

    return colorRegions;
}

const sendSolveRequest = (colorRegions: number[][]): Promise<SolveResponse> => {
    const msg: SolveRequest = {
        type: 'solve-request',
        colorRegions,
    };

    return new Promise(resolve => {
        console.log("[QueenSweep] Sending solve request to service-worker");

        chrome.runtime.sendMessage(msg, (response: SolveResponse) => {
            console.log("[QueenSweep] Received solve response from service-worker", response);
            resolve(response);
        });
    });
}

const main = async () => {
    console.log('[QueenSweep] Extracting color regions');
    const regions = extractColorRegions();

    const result = await sendSolveRequest(regions);

    if (result.success) {
        console.log(result.queenPositions);
    }
}

main();