import { defineConfig } from "vite";

import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";
import { viteStaticCopy } from "vite-plugin-static-copy";

export default defineConfig({
    plugins: [
        wasm(),
        topLevelAwait(),
        viteStaticCopy({
            targets: [
                { src: 'src/manifest.json', dest: '.' },
                { src: 'src/wasm/*.js', dest: 'wasm' },
            ],
        }),
    ],

    build: {
        target: "esnext",
        rollupOptions: {
            input: {
                'service-worker': 'src/service-worker.ts',
                'content-script': 'src/content-script.ts',
            },
            output: {
                dir: 'dist',
                entryFileNames: '[name].js',
                chunkFileNames: 'chunks/[name].js',
                assetFileNames: asset => {
                    const name = asset.names[0] || '';
                    if (name.includes('icon')) return 'assets/[name].[ext]';
                    if (name.endsWith('.wasm')) return 'wasm/[name].[ext]';
                    return 'popup/[name].[ext]';
                }
            },
        },
    },

    optimizeDeps: {
        exclude: ['wasm-module']
    }
})