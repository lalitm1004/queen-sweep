// Node target uses require() or import default, no need to call init()
import { WasmGameState } from './pkg/queen_sweep_wasm.js';

// Example usage
async function main() {
    const colorRegions = [
        [0, 0, 1, 1, 1, 2, 2, 2],
        [0, 3, 1, 3, 1, 4, 2, 2],
        [0, 3, 1, 3, 1, 2, 2, 2],
        [0, 3, 3, 3, 1, 5, 6, 2],
        [0, 3, 3, 3, 1, 5, 6, 6],
        [0, 3, 7, 3, 1, 5, 6, 6],
        [7, 3, 7, 3, 1, 5, 5, 6],
        [7, 7, 7, 7, 6, 6, 6, 6]
    ];

    const game = new WasmGameState(colorRegions);

    console.log("Original board:", game.get_states_2d());

    const solved = game.solve();

    if (solved) {
        console.log("Solved board:", solved.get_states_2d());
    } else {
        console.log("No solution found");
    }
}

main();
