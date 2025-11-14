const createPointerEvent = (
    type: 'pointerdown' | 'pointerup',
    clientX: number,
    clientY: number,
    buttons: number
): PointerEvent => {
    return new PointerEvent(type, {
        bubbles: true,
        cancelable: true,
        view: window,
        clientX,
        clientY,
        button: 0,
        buttons,
        pointerId: 1,
        pointerType: 'mouse',
        isPrimary: true,
    });
};

const clickSquare = (square: HTMLElement): void => {
    const rect = square.getBoundingClientRect();
    const clientX = rect.left + rect.width / 2;
    const clientY = rect.top + rect.height / 2;

    const pointerDown = createPointerEvent('pointerdown', clientX, clientY, 1);
    const pointerUp = createPointerEvent('pointerup', clientX, clientY, 0);

    square.dispatchEvent(pointerDown);
    setTimeout(() => {
        square.dispatchEvent(pointerUp);
        square.dispatchEvent(pointerDown);
        setTimeout(() => square.dispatchEvent(pointerUp), 10);
    }, 10);
};

export const applySolution = (queenPositions: number[][]): void => {
    queenPositions.forEach(([row, col], index) => {
        const square = document.querySelector(
            `.square[data-row="${row}"][data-col="${col}"]`
        ) as HTMLElement;

        if (!square) {
            console.warn(`[QueenSweep] Square not found at [${row}, ${col}]`);
            return;
        }

        setTimeout(() => clickSquare(square), 50 * index);
    });
};