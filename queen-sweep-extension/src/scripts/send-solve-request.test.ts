import { describe, expect, it, vi } from "vitest";
import { sendSolveRequest } from "./send-solve-request";
import type { SolveResponse } from "./types/messages.type";

const success: SolveResponse = {
    type: "solve-response-success",
    success: true,
    queenPositions: [[0, 1]],
    size: 4,
    numStatesVisited: 3,
};

describe("sendSolveRequest", () => {
    it("sends a solve-request message with the color regions", async () => {
        const sendMessage = vi.mocked(chrome.runtime.sendMessage);
        sendMessage.mockImplementation((_msg, cb) => {
            (cb as (r: SolveResponse) => void)(success);
        });

        const regions = [
            [0, 0, 1],
            [0, 1, 1],
        ];

        const response = await sendSolveRequest(regions);

        expect(sendMessage).toHaveBeenCalledOnce();
        const [message] = sendMessage.mock.calls[0];
        expect(message).toEqual({ type: "solve-request", colorRegions: regions });
        expect(response).toBe(success);
    });

    it("resolves with the failure response when the solver fails", async () => {
        const failure: SolveResponse = {
            type: "solve-response-failure",
            success: false,
            code: "BOARD_UNSOLVABLE",
        };

        vi.mocked(chrome.runtime.sendMessage).mockImplementation((_msg, cb) => {
            (cb as (r: SolveResponse) => void)(failure);
        });

        const response = await sendSolveRequest([[0]]);
        expect(response).toEqual(failure);
    });
});
