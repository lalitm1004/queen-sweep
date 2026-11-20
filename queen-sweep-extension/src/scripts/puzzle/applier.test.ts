import { afterEach, beforeEach, describe, expect, it, vi } from "vitest";
import { applySolution } from "./applier";

// jsdom's PointerEvent does not faithfully reproduce all init-dict fields, so a
// recording stub lets us assert exactly what the applier constructs/dispatches.
class FakePointerEvent extends Event {
    clientX: number;
    clientY: number;
    button: number;
    buttons: number;
    pointerId: number;
    pointerType: string;
    isPrimary: boolean;

    constructor(type: string, init: Record<string, unknown>) {
        super(type);
        this.clientX = init.clientX as number;
        this.clientY = init.clientY as number;
        this.button = init.button as number;
        this.buttons = init.buttons as number;
        this.pointerId = init.pointerId as number;
        this.pointerType = init.pointerType as string;
        this.isPrimary = init.isPrimary as boolean;
    }
}

function makeSquare(row: number, col: number): HTMLElement {
    const el = document.createElement("div");
    el.className = "square";
    el.setAttribute("data-row", String(row));
    el.setAttribute("data-col", String(col));
    document.body.appendChild(el);
    return el;
}

describe("applySolution", () => {
    beforeEach(() => {
        vi.useFakeTimers();
        vi.stubGlobal("PointerEvent", FakePointerEvent);
    });

    afterEach(() => {
        vi.useRealTimers();
        vi.unstubAllGlobals();
    });

    it("dispatches a double-click pointer sequence at the square center", () => {
        const square = makeSquare(1, 2);
        vi.spyOn(square, "getBoundingClientRect").mockReturnValue({
            left: 10,
            top: 20,
            width: 30,
            height: 40,
        } as DOMRect);

        const seen: FakePointerEvent[] = [];
        square.addEventListener("pointerdown", (e) => seen.push(e as FakePointerEvent));
        square.addEventListener("pointerup", (e) => seen.push(e as FakePointerEvent));

        applySolution([[1, 2]]);

        // The click for index 0 is scheduled at 0ms.
        vi.advanceTimersByTime(0);
        vi.advanceTimersByTime(10);
        vi.advanceTimersByTime(10);

        expect(seen.map((e) => e.type)).toEqual([
            "pointerdown",
            "pointerup",
            "pointerdown",
            "pointerup",
        ]);

        // Center of (10,20,30,40) is (25, 40).
        for (const event of seen) {
            expect(event.clientX).toBe(25);
            expect(event.clientY).toBe(40);
        }

        // pointerdown carries `buttons: 1`, pointerup carries `buttons: 0`.
        expect(seen[0].buttons).toBe(1);
        expect(seen[1].buttons).toBe(0);
        expect(seen[2].buttons).toBe(1);
        expect(seen[3].buttons).toBe(0);
    });

    it("staggeres multiple queens by 50ms each", () => {
        const a = makeSquare(0, 0);
        const b = makeSquare(1, 1);
        vi.spyOn(a, "getBoundingClientRect").mockReturnValue({
            left: 0,
            top: 0,
            width: 10,
            height: 10,
        } as DOMRect);
        vi.spyOn(b, "getBoundingClientRect").mockReturnValue({
            left: 100,
            top: 0,
            width: 10,
            height: 10,
        } as DOMRect);

        const aEvents: string[] = [];
        const bEvents: string[] = [];
        a.addEventListener("pointerdown", () => aEvents.push("down"));
        b.addEventListener("pointerdown", () => bEvents.push("down"));

        applySolution([
            [0, 0],
            [1, 1],
        ]);

        // First queen fires immediately; the second has not yet.
        vi.advanceTimersByTime(0);
        expect(aEvents).toEqual(["down"]);
        expect(bEvents).toEqual([]);

        // After 50ms the second queen's click is scheduled.
        vi.advanceTimersByTime(50);
        expect(bEvents).toEqual(["down"]);
    });

    it("warns when a target square is missing", () => {
        const warn = vi.spyOn(console, "warn").mockImplementation(() => {});

        applySolution([[5, 5]]);
        vi.advanceTimersByTime(0);

        expect(warn).toHaveBeenCalledWith(
            expect.stringContaining("Square not found at [5, 5]"),
        );
        warn.mockRestore();
    });
});
