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

        // Extract background color from style
        const style = square.getAttribute('style') || '';
        const bgColorMatch = style.match(/background-color:\s*rgb\(([^)]+)\)/);

        if (bgColorMatch) {
            const color = `rgb(${bgColorMatch[1]})`;

            // Assign color index
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
        console.log("[QueenSweep] Sending solve request to service-worker:", msg);

        chrome.runtime.sendMessage(msg, (response: SolveResponse) => {
            console.log("[QueenSweep] Received solve response from service-worker:", response);
            resolve(response);
        });
    });
}

const main = async () => {
    console.log('[QueenSweep] Extracting color regions');
    const regions = extractColorRegions();

    const result = await sendSolveRequest(regions);

    if (result.success) {
        console.log(result.queenPositions);
    }
}

main();