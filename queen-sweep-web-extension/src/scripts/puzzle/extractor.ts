export const extractColorRegions = (): number[][] => {
    const squares = document.querySelectorAll('.square');

    const rows = new Set<number>();
    const cols = new Set<number>();

    squares.forEach((square) => {
        const row = parseInt(square.getAttribute('data-row') || '0', 10);
        const col = parseInt(square.getAttribute('data-col') || '0', 10);
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

    squares.forEach((square) => {
        const row = parseInt(square.getAttribute('data-row') || '0', 10);
        const col = parseInt(square.getAttribute('data-col') || '0', 10);

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
};