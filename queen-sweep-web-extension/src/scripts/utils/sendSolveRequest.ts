import type { SolveRequest, SolveResponse } from "../types/messages.type";

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

export default sendSolveRequest;