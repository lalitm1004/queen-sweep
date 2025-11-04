const SOLVE_BUTTON_ID = "queensweep-solve-button";

const createSolveButton = (onClick: () => void): HTMLButtonElement => {
    const button = document.createElement('button');

    button.id = SOLVE_BUTTON_ID;
    button.style.cssText = `display: flex;justify-content: center;align-items: center;gap: 0.5rem;border-width: 3px;border-style: solid;border-color: oklch(52.7% 0.154 150.069);border-radius: 9999px;padding: 0.5rem;margin-right: 0.5rem;width: 100%;margin-top: 16px;margin-bottom: 16px;background-color: oklch(72.3% 0.219 149.579);color: oklch(14.5% 0 0);font-weight: 600;`;

    const iconUrl = chrome.runtime.getURL('assets/images/queen_icon.svg');

    button.innerHTML = `<img src="${iconUrl}" alt="queen" width="20" height="20"><span>Apply Solution</span>`;

    button.addEventListener('click', onClick);

    return button;
}

const removeSolveButton = (): void => {
    const btn = document.getElementById(SOLVE_BUTTON_ID);
    if (btn) btn.remove();
}

export { SOLVE_BUTTON_ID, createSolveButton, removeSolveButton }