import { describe, expect, it } from "vitest";
import { extractColorRegions } from "./extractor";

function square(row: number, col: number, rgb?: string): HTMLElement {
    const el = document.createElement("div");
    el.className = "square";
    el.setAttribute("data-row", String(row));
    el.setAttribute("data-col", String(col));
    if (rgb) {
        el.setAttribute("style", `background-color: rgb(${rgb});`);
    }
    document.body.appendChild(el);
    return el;
}

describe("extractColorRegions", () => {
    it("builds a color matrix with sequential indices by first appearance", () => {
        square(0, 0, "255, 0, 0");
        square(0, 1, "0, 255, 0");
        square(1, 0, "0, 255, 0");
        square(1, 1, "0, 0, 255");

        expect(extractColorRegions()).toEqual([
            [0, 1],
            [1, 2],
        ]);
    });

    it("assigns the same index to repeated rgb values", () => {
        square(0, 0, "10, 20, 30");
        square(0, 1, "10, 20, 30");
        square(1, 0, "10, 20, 30");
        square(1, 1, "10, 20, 30");

        expect(extractColorRegions()).toEqual([
            [0, 0],
            [0, 0],
        ]);
    });

    it("leaves cells without a parseable style as -1", () => {
        square(0, 0, "255, 0, 0");
        square(0, 1);
        square(1, 0, "0, 0, 255");
        square(1, 1);

        expect(extractColorRegions()).toEqual([
            [0, -1],
            [1, -1],
        ]);
    });

    it("derives dimensions from the maximum row/col indices", () => {
        square(0, 0, "255, 0, 0");
        square(2, 2, "0, 0, 255");

        expect(extractColorRegions()).toEqual([
            [0, -1, -1],
            [-1, -1, -1],
            [-1, -1, 1],
        ]);
    });

    it("ignores elements that are not .square", () => {
        const other = document.createElement("div");
        other.className = "not-a-square";
        other.setAttribute("data-row", "0");
        other.setAttribute("data-col", "0");
        document.body.appendChild(other);

        square(0, 0, "255, 0, 0");

        expect(extractColorRegions()).toEqual([[0]]);
    });

    it("treats rgb channel spacing flexibly", () => {
        // The production regex is lenient about whitespace around the values.
        const el = square(0, 0, "255,0,0");
        el.setAttribute("style", "background-color: rgb(255,0,0);");
        square(0, 1, "0, 0, 0");

        expect(extractColorRegions()).toEqual([[0, 1]]);
    });

    it("returns an empty board when there are no squares", () => {
        expect(extractColorRegions()).toEqual([]);
    });
});
