import { SolveRequest, SolveResponse } from "../types/messages.type";

const sendSolveRequest = (colorRegions: number[][]): Promise<SolveResponse> => {
    const msg: SolveRequest = {
        type: 'solve-request',
        colorRegions,
    };

    console.log('[QueenSweep] Sending SolveRequest to service-worker:', msg);
    return new Promise(resolve => {
        chrome.runtime.sendMessage(msg, (response: SolveResponse) => {
            console.log('[QueenSweep] Receieved SolveResponse from service-worker:', response);
            resolve(response);
        });
    });
}
export default sendSolveRequest;