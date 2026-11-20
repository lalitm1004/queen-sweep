import { fileURLToPath, URL } from "node:url";
import { defineConfig } from "vitest/config";

const wasmMock = fileURLToPath(
    new URL("./src/test/mocks/queen_sweep_wasm.ts", import.meta.url),
);

export default defineConfig({
    resolve: {
        alias: [
            {
                // The service worker imports the generated wasm bindings at this
                // relative path; in tests it is generated (and gitignored), so we
                // redirect it to a controllable mock.
                find: /^\.\.\/wasm\/queen_sweep_wasm\.js$/,
                replacement: wasmMock,
            },
        ],
    },
    test: {
        environment: "jsdom",
        setupFiles: ["src/test/setup.ts"],
        include: ["src/**/*.test.ts"],
        coverage: {
            provider: "v8",
            reporter: ["text", "html", "lcov"],
            include: ["src/scripts/**/*.ts"],
            exclude: ["src/scripts/**/*.test.ts", "src/test/**"],
        },
    },
});
