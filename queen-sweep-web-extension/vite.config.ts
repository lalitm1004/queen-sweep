import { defineConfig } from "vite";
import { resolve } from "path";

// plugins
import wasm from "vite-plugin-wasm";
import tailwindcss from "@tailwindcss/vite";
import topLevelAwait from "vite-plugin-top-level-await";
import { viteStaticCopy } from "vite-plugin-static-copy";

export default defineConfig({
    plugins: [
        wasm(),
        topLevelAwait(),
        tailwindcss(),
        viteStaticCopy({
            targets: [
                { src: 'src/manifest.json', dest: '.' },
                { src: 'src/wasm/*.js', dest: 'wasm' },
                { src: 'src/assets/images/*', dest: 'assets/images' },
            ],
        }),
    ],

    build: {
        target: "esnext",
        rollupOptions: {
            input: {
                'service-worker': 'src/scripts/service-worker.ts',
                'content-script': 'src/scripts/content-script.ts',
                popup: resolve(__dirname, 'src/popup/index.html'),
            },
            output: {
                dir: 'dist',
                manualChunks: undefined,
                entryFileNames: chunk => {
                    if (chunk.name === 'service-worker') return 'scripts/service-worker.js';
                    if (chunk.name === 'content-script') return 'scripts/content-script.js';
                    if (chunk.name === 'popup') return 'popup/popup.js';
                    return 'popup/[name].js';
                },
                assetFileNames: asset => {
                    const name = asset.names[0] || '';
                    if (name.includes('icon')) return 'assets/images/[name].[ext]';
                    if (name.endsWith('.wasm')) return 'wasm/[name].[ext]';
                    return 'popup/[name].[ext]';
                },
            },
        },
    },

    optimizeDeps: {
        exclude: ['wasm-module']
    }
})