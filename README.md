# QueenSweep

A high-performance solver for the **Queens** puzzle (place one queen in every row,
column, and colored region with no two queens touching — not even diagonally),
shipped as a Rust core, WebAssembly bindings, and a Chrome extension.

## Workspace

- `crates/queen-sweep-core` — the solver core: a const-generic board, DFS solver,
  heuristics, and a runtime `dynamic` dispatch API that accepts plain `Vec`s.
- `crates/queen-sweep-wasm` — type-safe WebAssembly bindings built with
  `wasm-bindgen` + `tsify` (generates TypeScript declarations).
- `queen-sweep-extension` — a Manifest V3 Chrome extension that detects and
  solves QueensGame puzzles in the browser.

## Development

The repo ships a Nix devshell with Rust, the `wasm32-unknown-unknown` target,
`wasm-pack`, `binaryen`, and Node.js:

```sh
nix develop
cargo test --workspace
```

### WebAssembly bindings

```sh
cd crates/queen-sweep-wasm
wasm-pack build --target web
```

### Chrome extension

```sh
cd queen-sweep-extension
npm install
npm run build
```

Then load `queen-sweep-extension/dist` as an unpacked extension in Chrome. See
[`queen-sweep-extension/README.md`](queen-sweep-extension/README.md) for details.
