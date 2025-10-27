import init, { QueensGame } from "../wasm/queen_sweep_wasm.js"

async function initWasm() {
    const wasmPath = chrome.runtime.getURL("wasm/queen_sweep_wasm_bg.wasm")
    console.log("[WASM] Loading from", wasmPath)

    await init(wasmPath)
    console.log("[WASM] Initialized successfully")
}

async function main() {
    await initWasm()

    console.log("[WASM] Creating game instance...")
    try {
        const game = new QueensGame([new Uint8Array([1, 2, 3])])
        console.log("[WASM] Game object created:", game)

        const solved = game.solve()
        if (solved) {
            console.log("[WASM] Solved! Positions:", solved.get_queen_positions())
        } else {
            console.log("[WASM] No solution.")
        }
    } catch (err) {
        console.error("[WASM ERROR]", err)
    }
}

main()
