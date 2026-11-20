import { beforeEach, describe, expect, it, vi } from "vitest";

vi.mock("./url-monitor", () => ({
    isOnPuzzlePage: vi.fn(),
    setupUrlMonitoring: vi.fn(),
}));
vi.mock("./puzzle/solver", () => ({
    solvePuzzle: vi.fn(),
}));

// `content-script.ts` runs `main()` on import, so each test configures the
// mocked `isOnPuzzlePage` result first, then re-imports a fresh module instance.
async function loadContentScript(onPuzzlePage: boolean): Promise<{
    isOnPuzzlePage: ReturnType<typeof vi.fn>;
    setupUrlMonitoring: ReturnType<typeof vi.fn>;
    solvePuzzle: ReturnType<typeof vi.fn>;
}> {
    vi.resetModules();

    const urlMonitor = await import("./url-monitor");
    const solver = await import("./puzzle/solver");

    const isOnPuzzlePage = urlMonitor.isOnPuzzlePage as ReturnType<typeof vi.fn>;
    const setupUrlMonitoring = urlMonitor.setupUrlMonitoring as ReturnType<typeof vi.fn>;
    const solvePuzzle = solver.solvePuzzle as ReturnType<typeof vi.fn>;

    isOnPuzzlePage.mockReturnValue(onPuzzlePage);

    await import("./content-script");

    return { isOnPuzzlePage, setupUrlMonitoring, solvePuzzle };
}

beforeEach(() => {
    vi.clearAllMocks();
});

describe("content script entry point", () => {
    it("sets up URL monitoring and solves on a puzzle page", async () => {
        const { isOnPuzzlePage, setupUrlMonitoring, solvePuzzle } =
            await loadContentScript(true);

        expect(setupUrlMonitoring).toHaveBeenCalledOnce();
        expect(isOnPuzzlePage).toHaveBeenCalledOnce();
        expect(solvePuzzle).toHaveBeenCalledOnce();
    });

    it("sets up URL monitoring but does not solve off a puzzle page", async () => {
        const { setupUrlMonitoring, solvePuzzle } =
            await loadContentScript(false);

        expect(setupUrlMonitoring).toHaveBeenCalledOnce();
        expect(solvePuzzle).not.toHaveBeenCalled();
    });
});
