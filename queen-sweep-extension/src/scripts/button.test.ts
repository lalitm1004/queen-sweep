import { describe, expect, it, vi } from "vitest";
import {
    createSolveButton,
    injectSolveButton,
    removeSolveButton,
    updateButtonState,
} from "./button";

const SOLVE_BUTTON_ID = "queen-sweep-solve-button";

describe("createSolveButton", () => {
    it("is disabled and shows loading content in the loading state", () => {
        const button = createSolveButton({ state: "loading" }, () => {});

        expect(button.id).toBe(SOLVE_BUTTON_ID);
        expect(button.disabled).toBe(true);
        expect(button.innerHTML).toContain("Solving...");
        expect(button.innerHTML).not.toContain("Apply Solution");
    });

    it("is enabled and shows apply content in the ready state", () => {
        const button = createSolveButton({ state: "ready" }, () => {});

        expect(button.disabled).toBe(false);
        expect(button.innerHTML).toContain("Apply Solution");
    });

    it("invokes onClick only when in the ready state", () => {
        const onClick = vi.fn();

        const loading = createSolveButton({ state: "loading" }, onClick);
        loading.click();
        expect(onClick).not.toHaveBeenCalled();

        const ready = createSolveButton({ state: "ready" }, onClick);
        ready.click();
        expect(onClick).toHaveBeenCalledOnce();
    });
});

describe("updateButtonState", () => {
    it("updates an existing button's disabled state", () => {
        const button = createSolveButton({ state: "ready" }, () => {});
        document.body.appendChild(button);

        updateButtonState("loading");
        expect(button.disabled).toBe(true);
        expect(button.innerHTML).toContain("Solving...");

        updateButtonState("idle");
        expect(button.disabled).toBe(false);
        expect(button.innerHTML).toContain("Apply Solution");
    });

    it("does nothing when no button exists", () => {
        expect(() => updateButtonState("loading")).not.toThrow();
    });
});

describe("removeSolveButton", () => {
    it("removes the button from the DOM", () => {
        const button = createSolveButton({ state: "idle" }, () => {});
        document.body.appendChild(button);

        removeSolveButton();
        expect(document.getElementById(SOLVE_BUTTON_ID)).toBeNull();
    });

    it("is a no-op when the button is absent", () => {
        expect(() => removeSolveButton()).not.toThrow();
    });
});

describe("injectSolveButton", () => {
    it("inserts before the game container when present", () => {
        const container = document.createElement("div");
        container.className = "game-board";
        const parent = document.createElement("div");
        parent.appendChild(container);
        document.body.appendChild(parent);

        injectSolveButton({ state: "ready" }, () => {});

        const button = document.getElementById(SOLVE_BUTTON_ID);
        expect(button).not.toBeNull();
        expect(parent.firstChild).toBe(button);
    });

    it("appends to body when no game container exists", () => {
        injectSolveButton({ state: "idle" }, () => {});

        const button = document.getElementById(SOLVE_BUTTON_ID);
        expect(button).not.toBeNull();
        expect(document.body.lastChild).toBe(button);
    });

    it("replaces any existing button", () => {
        const first = createSolveButton({ state: "idle" }, () => {});
        document.body.appendChild(first);

        injectSolveButton({ state: "ready" }, () => {});

        const buttons = document.querySelectorAll(`#${SOLVE_BUTTON_ID}`);
        expect(buttons).toHaveLength(1);
        expect(document.body.contains(first)).toBe(false);
    });
});
