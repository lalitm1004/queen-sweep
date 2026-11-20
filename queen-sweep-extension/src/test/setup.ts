import { afterEach, vi } from "vitest";

// A minimal, per-test-resettable `chrome` stub for the extension's isolated
// world. Individual tests replace the behaviour of specific methods via
// `vi.mocked(...)` or by reassigning the stubs below.
export const chromeStub = {
    runtime: {
        sendMessage: vi.fn(),
        getURL: vi.fn((path: string) => `chrome-extension://queen-sweep/${path}`),
        onMessage: {
            addListener: vi.fn(),
            removeListener: vi.fn(),
        },
    },
};

(globalThis as unknown as { chrome: typeof chromeStub }).chrome = chromeStub;

// jsdom does not implement `Element.prototype.getBoundingClientRect` with a
// useful default; provide a stable zero-rect that tests can override.
if (!Element.prototype.getBoundingClientRect) {
    Element.prototype.getBoundingClientRect = () =>
        ({
            left: 0,
            top: 0,
            right: 0,
            bottom: 0,
            width: 0,
            height: 0,
            x: 0,
            y: 0,
            toJSON: () => ({}),
        }) as DOMRect;
}

afterEach(() => {
    vi.clearAllMocks();
    document.body.innerHTML = "";
});
