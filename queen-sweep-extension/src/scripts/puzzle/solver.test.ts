import { beforeEach, describe, expect, it, vi } from "vitest";
import type { ButtonConfig } from "../button";

vi.mock("./extractor", () => ({ extractColorRegions: vi.fn() }));
vi.mock("./applier", () => ({ applySolution: vi.fn() }));
vi.mock("../send-solve-request", () => ({ sendSolveRequest: vi.fn() }));
vi.mock("../button", () => ({
    injectSolveButton: vi.fn(),
    updateButtonState: vi.fn(),
}));

import { extractColorRegions } from "./extractor";
import { applySolution } from "./applier";
import { sendSolveRequest } from "../send-solve-request";
import { injectSolveButton, updateButtonState } from "../button";
import { solvePuzzle } from "./solver";

const regions = [
    [0, 0, 1],
    [0, 1, 1],
];

beforeEach(() => {
    vi.clearAllMocks();
    vi.mocked(extractColorRegions).mockReturnValue(regions);
});

describe("solvePuzzle", () => {
    it("shows loading, then ready, and applies the solution on click", async () => {
        const queens: [number, number][] = [
            [0, 1],
            [1, 2],
        ];
        vi.mocked(sendSolveRequest).mockResolvedValue({
            type: "solve-response-success",
            success: true,
            queenPositions: queens,
        });

        await solvePuzzle();

        expect(extractColorRegions).toHaveBeenCalledOnce();
        expect(sendSolveRequest).toHaveBeenCalledWith(regions);

        // First call: loading state. Second call: ready state with positions.
        expect(injectSolveButton).toHaveBeenCalledTimes(2);
        const [loadingCall, readyCall] = vi.mocked(injectSolveButton).mock.calls;
        expect((loadingCall[0] as ButtonConfig).state).toBe("loading");
        expect((readyCall[0] as ButtonConfig).state).toBe("ready");
        expect((readyCall[0] as ButtonConfig).queenPositions).toEqual(queens);

        // Clicking the ready button applies the solution.
        (readyCall[1] as () => void)();
        expect(applySolution).toHaveBeenCalledWith(queens);
    });

    it("reverts to idle when the solver reports failure", async () => {
        vi.mocked(sendSolveRequest).mockResolvedValue({
            type: "solve-response-failure",
            success: false,
            code: "BOARD_UNSOLVABLE",
        });

        await solvePuzzle();

        expect(updateButtonState).toHaveBeenCalledWith("idle");
        expect(injectSolveButton).toHaveBeenCalledTimes(1);
    });

    it("reverts to idle when solving throws", async () => {
        vi.mocked(sendSolveRequest).mockRejectedValue(new Error("boom"));

        await solvePuzzle();

        expect(updateButtonState).toHaveBeenCalledWith("idle");
    });
});
