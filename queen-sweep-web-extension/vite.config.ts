import { defineConfig } from "vite";
import { resolve } from "path";

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
                { src: 'src/assets/*.png', dest: 'assets' },
                { src: 'src/assets/*.svg', dest: 'assets' },
            ],
        }),
    ],

    build: {
        target: "esnext",
        rollupOptions: {
            input: {
                'service-worker': 'src/scripts/service-worker.ts',
                'content-script-queensgame': 'src/scripts/content-script-queensgame.ts',
                popup: resolve(__dirname, 'src/popup/index.html'),
            },
            output: {
                dir: 'dist',
                entryFileNames: chunk => {
                    if (chunk.name.startsWith('content-script-')) return `scripts/${chunk.name}.js`;

                    if (chunk.name === 'service-worker') return 'scripts/service-worker.js';
                    if (chunk.name === 'popup') return 'popup/popup.js';
                    return 'popup/[name].js';
                },
                assetFileNames: asset => {
                    const name = asset.names[0] || '';
                    if (name.includes('icon')) return 'assets/[name].[ext]';
                    if (name.endsWith('.wasm')) return 'wasm/[name].[ext]';
                    return 'popup/[name].[ext]';
                },
                chunkFileNames: 'chunks/[name].js',
            },
        },
    },

    optimizeDeps: {
        exclude: ['wasm-module']
    }
})