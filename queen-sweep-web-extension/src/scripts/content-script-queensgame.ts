import { createSolveButton, removeSolveButton } from "./utils/button";
import sendSolveRequest from "./utils/sendSolveRequest";

const extractColorRegions = (): number[][] => {
    const squares = document.querySelectorAll('.square');

    const rows = new Set<number>();
    const cols = new Set<number>();

    squares.forEach(square => {
        const row = parseInt(square.getAttribute('data-row') || '0');
        const col = parseInt(square.getAttribute('data-col') || '0');
        rows.add(row);
        cols.add(col);
    });

    const numRows = Math.max(...rows) + 1;
    const numCols = Math.max(...cols) + 1;

    const colorRegions: number[][] = Array.from({ length: numRows }, () =>
        Array(numCols).fill(-1)
    );

    const colorMap = new Map<string, number>();
    const colors: string[] = [];
    let colorIndex = 0;

    squares.forEach(square => {
        const row = parseInt(square.getAttribute('data-row') || '0');
        const col = parseInt(square.getAttribute('data-col') || '0');

        // extract background color from style
        const style = square.getAttribute('style') || '';
        const bgColorMatch = style.match(/background-color:\s*rgb\(([^)]+)\)/);

        if (bgColorMatch) {
            const color = `rgb(${bgColorMatch[1]})`;

            if (!colorMap.has(color)) {
                colorMap.set(color, colorIndex);
                colors.push(color);
                colorIndex++;
            }

            colorRegions[row][col] = colorMap.get(color)!;
        }
    });

    return colorRegions;
}


const applySolution = (queenPositions: number[][]): void => {
    queenPositions.forEach(([row, col], index) => {
        const square = document.querySelector(
            `.square[data-row="${row}"][data-col="${col}"]`
        ) as HTMLElement;

        if (!square) {
            console.warn(`[QueenSweep] Square not found at [${row}, ${col}]`);
        }

        setTimeout(() => {
            const rect = square.getBoundingClientRect();
            const clientX = rect.left + rect.width / 2;
            const clientY = rect.top + rect.height / 2;

            const down = new PointerEvent('pointerdown', {
                bubbles: true,
                cancelable: true,
                view: window,
                clientX,
                clientY,
                button: 0,
                buttons: 1,
                pointerId: 1,
                pointerType: 'mouse',
                isPrimary: true
            });

            const up = new PointerEvent('pointerup', {
                bubbles: true,
                cancelable: true,
                view: window,
                clientX,
                clientY,
                button: 0,
                buttons: 0,
                pointerId: 1,
                pointerType: 'mouse',
                isPrimary: true
            });

            square.dispatchEvent(down);
            setTimeout(() => {
                square.dispatchEvent(up);
                square.dispatchEvent(down);
                setTimeout(() => square.dispatchEvent(up), 10);
            }, 10);
        }, 50 * index);
    });
};

const injectSolveButton = (queenPositions: number[][]): void => {
    removeSolveButton();

    const button = createSolveButton(() => applySolution(queenPositions));

    const container = document.querySelector('[class*="game"]');

    if (container && container.parentNode) {
        container.parentNode.insertBefore(button, container);
    } else {
        document.body.appendChild(button);
    }
};

const solvePuzzle = async (): Promise<void> => {
    try {
        console.log('[QueenSweep] Extracting color regions');
        const regions = extractColorRegions();

        const result = await sendSolveRequest(regions);

        if (!result.success) {
            console.warn('[QueenSweep] No solution found');
            return;
        }

        injectSolveButton(result.queenPositions);
    } catch (error) {
        console.error('[QueenSweep] Error solving puzzle:', error);
    }
};

const isOnPuzzlePage = (): boolean => {
    const url = location.href;
    return url.includes('/level/') ||
        url.includes('/community-level/') ||
        url.includes('/bonus-level/');
};

const setupUrlMonitoring = (): void => {
    let lastUrl = location.href;

    const checkForUrlChange = () => {
        const currentUrl = location.href;
        if (currentUrl !== lastUrl) {
            if (isOnPuzzlePage()) {
                solvePuzzle();
            } else {
                removeSolveButton();
            }
        }

        lastUrl = currentUrl;
    };

    // monitor DOM changes to detect SPA navigation
    new MutationObserver(checkForUrlChange).observe(document, {
        subtree: true,
        childList: true
    });
};

const main = (): void => {
    console.log('[QueenSweep] Content script initialized');

    setupUrlMonitoring();

    if (isOnPuzzlePage()) {
        solvePuzzle();
    }
};

main();