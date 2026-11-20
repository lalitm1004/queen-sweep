import { beforeEach, describe, expect, it, vi } from "vitest";
import type { SolveRequest, SolveResponse } from "./types/messages.type";

type WasmMock = {
    default: ReturnType<typeof vi.fn>;
    buildFromFlat: ReturnType<typeof vi.fn>;
};

// Loads a fresh service-worker and wasm-mock module instance so the cached
// `wasmInitPromise` (and mock call history) does not leak between tests.
async function setup(): Promise<WasmMock> {
    vi.resetModules();
    const wasm = (await import("../wasm/queen_sweep_wasm.js")) as unknown as WasmMock;
    await import("./service-worker");
    return wasm;
}

type Listener = (
    msg: SolveRequest,
    sender: unknown,
    sendResponse: (res: SolveResponse) => void,
) => boolean | undefined;

function currentListener(): Listener {
    const calls = vi.mocked(chrome.runtime.onMessage.addListener).mock.calls;
    return calls[calls.length - 1][0] as Listener;
}

function dispatch(msg: SolveRequest): Promise<SolveResponse> {
    return new Promise((resolve) => {
        const returned = currentListener()(msg, {}, resolve);
        // The listener must return `true` to keep the message channel open for
        // the async response.
        expect(returned).toBe(true);
    });
}

const quadrant = [
    [0, 0, 1, 1],
    [0, 0, 1, 1],
    [2, 2, 3, 3],
    [2, 2, 3, 3],
];

beforeEach(() => {
    vi.clearAllMocks();
});

describe("service worker solve request handling", () => {
    it("reports WASM_NOT_INITIALIZED when wasm init fails", async () => {
        const { default: init } = await setup();
        init.mockRejectedValue(new Error("fetch failed"));

        const response = await dispatch({
            type: "solve-request",
            colorRegions: quadrant,
        });

        expect(response).toEqual({
            type: "solve-response-failure",
            success: false,
            code: "WASM_NOT_INITIALIZED",
        });
    });

    it("returns queen positions on a successful solve", async () => {
        const { default: init, buildFromFlat } = await setup();
        init.mockResolvedValue(undefined);
        buildFromFlat.mockReturnValue({
            solve: () => ({
                solved: true,
                queens: [
                    [0, 1],
                    [1, 3],
                    [2, 0],
                    [3, 2],
                ],
                size: 4,
                numStatesVisited: 5,
            }),
        });

        const response = await dispatch({
            type: "solve-request",
            colorRegions: quadrant,
        });

        expect(response).toEqual({
            type: "solve-response-success",
            success: true,
            queenPositions: [
                [0, 1],
                [1, 3],
                [2, 0],
                [3, 2],
            ],
            size: 4,
            numStatesVisited: 5,
        });

        // The solver is invoked with a flattened color list and the chosen
        // solver/heuristic kinds.
        expect(buildFromFlat).toHaveBeenCalledWith(
            quadrant.flat(),
            0, // SolverKind.Dfs
            1, // HeuristicKind.SmallestRegionFirst
        );
    });

    it("maps a build error to its stable code and message", async () => {
        const { default: init, buildFromFlat } = await setup();
        init.mockResolvedValue(undefined);
        buildFromFlat.mockImplementation(() => {
            throw { code: "UNSUPPORTED_SIZE", message: "board size 2 is not supported" };
        });

        const response = await dispatch({
            type: "solve-request",
            colorRegions: [
                [0, 1],
                [1, 0],
            ],
        });

        expect(response).toEqual({
            type: "solve-response-failure",
            success: false,
            code: "UNSUPPORTED_SIZE",
            message: "board size 2 is not supported",
        });
    });

    it("reports BOARD_UNSOLVABLE when the solver finds no solution", async () => {
        const { default: init, buildFromFlat } = await setup();
        init.mockResolvedValue(undefined);
        buildFromFlat.mockReturnValue({
            solve: () => ({ solved: false, queens: [], size: 4, numStatesVisited: 1 }),
        });

        const response = await dispatch({
            type: "solve-request",
            colorRegions: quadrant,
        });

        expect(response).toEqual({
            type: "solve-response-failure",
            success: false,
            code: "BOARD_UNSOLVABLE",
        });
    });

    it("ignores messages that are not solve-requests", async () => {
        const { default: init, buildFromFlat } = await setup();
        init.mockResolvedValue(undefined);

        const sendResponse = vi.fn();
        // A non-solve-request message must not trigger solving.
        currentListener()({ type: "other" } as unknown as SolveRequest, {}, sendResponse);

        expect(sendResponse).not.toHaveBeenCalled();
        expect(buildFromFlat).not.toHaveBeenCalled();
    });
});
