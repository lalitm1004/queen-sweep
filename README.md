![Queen Sweep Logo](.github/assets/queen-sweep-logo.png)

# QueenSweep

**QueenSweep** is a high-performance solver for [LinkedIn's Queens](https://www.linkedin.com/showcase/queens-game), featuring a Rust-based depth first search engine, WebAssembly compilation, and a chromium extension for seamless in-browser solving.

> [!NOTE]
> Because LinkedIn prohibits automated interaction with its platform, the Chromium extension is only supported on the [QueensGame website](https://queensgame.vercel.app) created by [samimsu](https://github.com/samimsu)

## Table of Contents
1. [Features](#features)
2. [Demo](#demo)
3. [Architecture]
4. [Project Structure](#project-structure)
5. [Running Locally]

## Features
- **🚀 Blazingly Fast**: Solves majority of boards in under 5 milliseconds
- **🧠 Heuristic-Driven Search**: Pluggable, configurable heuristics to aggressively prune the search space
- **🌐 WebAssembly Runtime**: Compiled from Rust for near-native execution speed directly in the browser
- **🛰️ Integrated Browser Extension**: One-click solver injected directly in to the puzzle website

## Demo
The chromium extension injects an **Apply Solution** button once it detects a valid solution for the puzzle curently displayed on the page

<div align="center">

![QueenSweep Demo GIF](.github/assets/demo.gif)

</div>

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