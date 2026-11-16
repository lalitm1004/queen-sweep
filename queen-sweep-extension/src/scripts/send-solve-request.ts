import type { SolveRequest, SolveResponse } from "./types/messages.type";

export const sendSolveRequest = (colorRegions: number[][]): Promise<SolveResponse> => {
    const message: SolveRequest = {
        type: "solve-request",
        colorRegions,
    };

    console.log("[QueenSweep] Sending solve request to service worker:", message);

    return new Promise((resolve) => {
        chrome.runtime.sendMessage(message, (response: SolveResponse) => {
            console.log("[QueenSweep] Received solve response from service worker:", response);
            resolve(response);
        });
    });
};
