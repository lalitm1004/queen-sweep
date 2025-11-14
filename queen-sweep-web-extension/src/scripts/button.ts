export type ButtonState = 'idle' | 'loading' | 'ready';

export interface ButtonConfig {
    state: ButtonState;
    queenPositions?: number[][];
}

const SOLVE_BUTTON_ID = "queen-sweep-solve-button";

const getButtonStyles = (state: ButtonState): string => {
    const baseStyles = `
        display: flex;
        justify-content: center;
        align-items: center;
        gap: 0.5rem;
        border-width: 3px;
        border-style: solid;
        border-radius: 9999px;
        padding: 0.5rem;
        margin-right: 0.5rem;
        width: 100%;
        margin-top: 16px;
        margin-bottom: 16px;
        font-weight: 600;
        transition: all 0.2s ease;
    `;

    if (state === 'loading') {
        return baseStyles + `
            border-color: oklch(52.7% 0.154 150.069);
            background-color: oklch(72.3% 0.219 149.579);
            color: oklch(14.5% 0 0);
            opacity: 0.7;
            cursor: progress;
        `;
    }

    return baseStyles + `
        border-color: oklch(52.7% 0.154 150.069);
        background-color: oklch(72.3% 0.219 149.579);
        color: oklch(14.5% 0 0);
        cursor: pointer;
    `;
};

const getButtonContent = (state: ButtonState): string => {
    const iconUrl = chrome.runtime.getURL('assets/images/queen_icon.svg');

    if (state === 'loading') {
        return `
            <img src="${iconUrl}" alt="queen" width="20" height="20">
            <span>Solving...</span>
        `;
    }

    return `
        <img src="${iconUrl}" alt="queen" width="20" height="20">
        <span>Apply Solution</span>
    `;
};

export const createSolveButton = (config: ButtonConfig, onClick: () => void): HTMLButtonElement => {
    const button = document.createElement('button');

    button.id = SOLVE_BUTTON_ID;
    button.style.cssText = getButtonStyles(config.state);
    button.innerHTML = getButtonContent(config.state);
    button.disabled = config.state === 'loading';

    if (config.state === 'ready') {
        button.addEventListener('click', onClick);
    }

    return button;
};

export const updateButtonState = (state: ButtonState): void => {
    const button = document.getElementById(SOLVE_BUTTON_ID) as HTMLButtonElement;
    if (!button) return;

    button.style.cssText = getButtonStyles(state);
    button.innerHTML = getButtonContent(state);
    button.disabled = state === 'loading';
};

export const removeSolveButton = (): void => {
    const button = document.getElementById(SOLVE_BUTTON_ID);
    if (button) button.remove();
};

export const injectSolveButton = (config: ButtonConfig, onClick: () => void): void => {
    removeSolveButton();

    const button = createSolveButton(config, onClick);
    const container = document.querySelector('[class*="game"]');

    if (container && container.parentNode) {
        container.parentNode.insertBefore(button, container);
    } else {
        document.body.appendChild(button);
    }
};