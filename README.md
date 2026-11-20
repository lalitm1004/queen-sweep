![Queen Sweep Logo](.github/assets/queen-sweep.png)

# ♛ QueenSweep

**QueenSweep** is a high-performance solver for [LinkedIn's Queens](https://www.linkedin.com/showcase/queens-game), featuring a Rust-based depth first search engine, WebAssembly compilation, and a Chromium extension for seamless in-browser solving.

> [!NOTE]
> Because LinkedIn prohibits automated interaction with its platform, the Chromium extension is only supported on the [QueensGame website](https://queensgame.vercel.app) created by [samimsu](https://github.com/samimsu)

## 📋 Table of Contents
1. [Features](#-features)
2. [Architecture](#️-architecture)
3. [Project Structure](#-project-structure)
4. [Install from GitHub Releases](#-install-from-github-releases)
5. [Running Locally](#-running-locally)

## 📃 Features
- **🚀 Blazingly Fast**: Solves the majority of boards in a few milliseconds with depth-first search and aggressive constraint propagation
- **📐 Const-Generic Core**: The board is sized at compile time, with a runtime `dynamic` layer that dispatches `Vec` input to the right monomorphization
- **🧠 Heuristic-Driven Search**: Pluggable, configurable heuristics (`NoOp`, `SmallestRegionFirst`) to aggressively prune the search space
- **🌐 WebAssembly Runtime**: Compiled from Rust for near-native execution speed directly in the browser, with generated TypeScript types
- **🛰️ Integrated Browser Extension**: One-click solver injected directly in to the puzzle website

## 🏗️ Architecture

```text
queen-sweep-core  ->  queen-sweep-wasm  ->  queen-sweep-extension  ->  QueensGame
(board, DFS,           (wasm-bindgen +       (service worker +          (puzzle site)
 heuristics,            tsify types)          content script)
 dynamic API)
```

## 📂 Project Structure
```sh
# All major sub-directories
queen-sweep/
├── crates/
│   ├── queen-sweep-core/      # Core rust engine (+ runtime `dynamic` API)
│   └── queen-sweep-wasm/      # WASM bindings (TypeScript types)
└── queen-sweep-extension/     # Chromium extension
```

Each directory contains its own README with detailed information:
- [Core Engine](./crates/queen-sweep-core/README.md)
- [WASM Bindings](./crates/queen-sweep-wasm/README.md)
- [Web Extension](./queen-sweep-extension/README.md)

## 📦 Install from GitHub Releases

Grab the prebuilt extension straight from the [Releases](https://github.com/lalitm1004/queen-sweep/releases) page - no build step required.

1. Open the [Releases](https://github.com/lalitm1004/queen-sweep/releases) page
2. Download the latest `queen-sweep-extension-v*.zip`
3. Unzip it
4. Navigate to `chrome://extensions/` (or `brave://extensions/`)
5. Enable `Developer mode`
6. Click `Load unpacked` and select the unzipped folder

## 💻 Running Locally
Ensure the following prerequisites are available:
- [Rust](https://rustup.rs/)
- [Node.js](https://nodejs.org)
- [wasm-pack](https://github.com/drager/wasm-pack)

> [!TIP]
> The repository ships a Nix devshell that provides Rust, the `wasm32-unknown-unknown` target, `wasm-pack`, `binaryen`, and Node.js. Run `nix develop` to enter it.

### Installing the Chromium Extension

1. Clone the repository
```bash
git clone https://github.com/lalitm1004/queen-sweep.git
cd queen-sweep
```

2. Build the WASM module
```bash
cd queen-sweep-extension
npm run build:wasm
```

3. Build the extension
```bash
npm i
npm run build
```

4. Load in Chrome
   - Navigate to `brave://extensions/` **or** your extension manager
   - Enable `Developer mode`
   - Click `Load unpacked`
   - Select the `queen-sweep-extension/dist` directory

### Running the core engine
```bash
cargo test -p queen-sweep-core
```

The core exposes both a const-generic API (`QueensBoard<N>`) and a runtime `dynamic` API. See the [Core Engine README](./crates/queen-sweep-core/README.md) for a complete usage example.
