import { isOnPuzzlePage, setupUrlMonitoring } from "./url-monitor";
import { solvePuzzle } from "./puzzle/solver";

const main = (): void => {
    console.log('[QueenSweep] Content script initialized');

    setupUrlMonitoring();

    if (isOnPuzzlePage()) {
        solvePuzzle();
    }
}

main();