import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";

vi.mock("./puzzle/solver", () => ({ solvePuzzle: vi.fn() }));
vi.mock("./button", () => ({ removeSolveButton: vi.fn() }));

import { solvePuzzle } from "./puzzle/solver";
import { removeSolveButton } from "./button";
import { isOnPuzzlePage, setupUrlMonitoring } from "./url-monitor";

function withUrl(url: string): void {
    vi.stubGlobal("location", { href: url });
}

class FakeMutationObserver {
    static instances: FakeMutationObserver[] = [];
    callback: () => void;
    constructor(callback: () => void) {
        this.callback = callback;
        FakeMutationObserver.instances.push(this);
    }
    observe(): void {}
    disconnect(): void {}
}

beforeEach(() => {
    vi.stubGlobal("MutationObserver", FakeMutationObserver);
    FakeMutationObserver.instances = [];
    vi.clearAllMocks();
});

afterEach(() => {
    vi.unstubAllGlobals();
});

describe("isOnPuzzlePage", () => {
    it("matches level pages", () => {
        withUrl("https://queensgame.vercel.app/level/12");
        expect(isOnPuzzlePage()).toBe(true);
    });

    it("matches community-level pages", () => {
        withUrl("https://queensgame.vercel.app/community-level/7");
        expect(isOnPuzzlePage()).toBe(true);
    });

    it("matches bonus-level pages", () => {
        withUrl("https://queensgame.vercel.app/bonus-level/3");
        expect(isOnPuzzlePage()).toBe(true);
    });

    it("rejects non-puzzle pages", () => {
        withUrl("https://queensgame.vercel.app/");
        expect(isOnPuzzlePage()).toBe(false);

        withUrl("https://queensgame.vercel.app/about");
        expect(isOnPuzzlePage()).toBe(false);

        withUrl("https://queensgame.vercel.app/level");
        expect(isOnPuzzlePage()).toBe(false);
    });
});

describe("setupUrlMonitoring", () => {
    it("solves the puzzle when navigating to a puzzle page", () => {
        withUrl("https://queensgame.vercel.app/");
        setupUrlMonitoring();
        const observer = FakeMutationObserver.instances[0];

        withUrl("https://queensgame.vercel.app/level/5");
        observer.callback();

        expect(solvePuzzle).toHaveBeenCalledOnce();
        expect(removeSolveButton).not.toHaveBeenCalled();
    });

    it("removes the button when leaving a puzzle page", () => {
        withUrl("https://queensgame.vercel.app/level/5");
        setupUrlMonitoring();
        const observer = FakeMutationObserver.instances[0];

        withUrl("https://queensgame.vercel.app/");
        observer.callback();

        expect(removeSolveButton).toHaveBeenCalledOnce();
        expect(solvePuzzle).not.toHaveBeenCalled();
    });

    it("does nothing when the URL has not changed", () => {
        withUrl("https://queensgame.vercel.app/level/5");
        setupUrlMonitoring();
        const observer = FakeMutationObserver.instances[0];

        observer.callback();

        expect(solvePuzzle).not.toHaveBeenCalled();
        expect(removeSolveButton).not.toHaveBeenCalled();
    });
});
