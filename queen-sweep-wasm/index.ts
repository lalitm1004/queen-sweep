import { GameStateWasm } from './pkg/queen_sweep_wasm.js';

function main() {
    const colorRegions = [
        [0, 1, 1, 1, 2, 2, 2, 2, 2, 2, 2],
        [0, 3, 3, 1, 1, 1, 2, 2, 2, 2, 2],
        [0, 3, 3, 1, 1, 2, 2, 2, 2, 2, 2],
        [0, 0, 0, 0, 2, 2, 2, 4, 4, 4, 4],
        [0, 5, 5, 6, 7, 7, 2, 2, 2, 4, 4],
        [0, 5, 5, 6, 7, 7, 7, 7, 7, 7, 4],
        [0, 5, 5, 6, 7, 7, 7, 7, 7, 7, 4],
        [0, 5, 5, 6, 7, 7, 8, 9, 7, 7, 4],
        [0, 5, 5, 6, 7, 7, 8, 9, 7, 7, 8],
        [10, 5, 5, 6, 7, 7, 8, 9, 7, 7, 8],
        [10, 10, 10, 6, 6, 8, 8, 8, 8, 8, 8],
    ];

    const colorRegionsUint8 = colorRegions.map(row => new Uint8Array(row));
    const game = new GameStateWasm(colorRegionsUint8);

    console.log("Original board:", game.get_states_2d());

    const solved = game.solve();

    if (solved) {
        console.log("Solved board:", solved.get_states_2d());
        console.log("Queen positions:", solved.get_queen_positions());
    } else {
        console.log("No solution found");
    }
}

main();
