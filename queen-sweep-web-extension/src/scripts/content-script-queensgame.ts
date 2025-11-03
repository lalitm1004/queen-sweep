import { SolveRequest, SolveResponse } from "./types/messages.type";

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

const sendSolveRequest = (colorRegions: number[][]): Promise<SolveResponse> => {
    const msg: SolveRequest = {
        type: 'solve-request',
        colorRegions,
    };

    return new Promise(resolve => {
        chrome.runtime.sendMessage(msg, (response: SolveResponse) => resolve(response));
    });
}

const solvePuzzle = async (): Promise<void> => {
    try {
        console.log('[QueenSweep] Extracting color regions');
        const regions = extractColorRegions();

        const result = await sendSolveRequest(regions);

        if (result.success) {
            console.log('[QueenSweep] Solution found:', result.queenPositions);

            // TODO: Apply solution to UI
        } else {
            console.warn('[QueenSweep] No solution found');
        }
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
        if (currentUrl !== lastUrl && isOnPuzzlePage()) {
            solvePuzzle();
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