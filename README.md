![Queen Sweep Logo](.github/assets/queen-sweep.png)

# QueenSweep

**QueenSweep** is a high-performance solver for [LinkedIn's Queens](https://www.linkedin.com/showcase/queens-game), featuring a Rust-based depth first search engine, WebAssembly compilation, and a chromium extension for seamless in-browser solving.

> Built with performance in mind, it can solve most boards in under 5 milliseconds.

## Table of Contents
1. [Features](#features)
2. [Demo]
3. [Architecture]
4. [Project Structure](#project-structure)
5. [Running Locally]

## Features
- **🚀 Blazingly Fast**: Solves majority of boards in under `5ms`
- **🧠 Heuristic-Driven Search**: Pluggable, configurable heuristics to aggressively prune the search space
- **🌐 WebAssembly Runtime**: Compiled from Rust for near-native execution speed directly in the browser
- **🛰️ Integrated Browser Extension**: One-click solver injected directly in to the puzzle website

## Project Structure
```sh
# All major sub-directories
queen-sweep/
├── queen-sweep-core/           # Core rust engine
├── queen-sweep-macros/         # Procedural macros for core engine
├── queen-sweep-wasm/           # WASM bindings
└── queen-sweep-web-extension/  # Chromium extension
```

Each directory contains its own README with detailed information:
- [Core Engine](./queen-sweep-core/README.md)
- [Macros](./queen-sweep-macros/README.md)
- [WASM Bindings](./queen-sweep-wasm/README.md)
- [Chromium Extension](./queen-sweep-web-extension/README.md)