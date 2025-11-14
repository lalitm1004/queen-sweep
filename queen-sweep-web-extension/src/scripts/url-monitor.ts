import { solvePuzzle } from './puzzle/solver';
import { removeSolveButton } from './button';


export const isOnPuzzlePage = (): boolean => {
    const url = location.href;
    return (
        url.includes('/level/') ||
        url.includes('/community-level/') ||
        url.includes('/bonus-level/')
    );
};


export const setupUrlMonitoring = (): void => {
    let lastUrl = location.href;

    const checkForUrlChange = (): void => {
        const currentUrl = location.href;

        if (currentUrl !== lastUrl) {
            if (isOnPuzzlePage()) {
                solvePuzzle();
            } else {
                removeSolveButton();
            }
            lastUrl = currentUrl;
        }
    };

    // monitor DOM changes to detect SPA navigation
    new MutationObserver(checkForUrlChange).observe(document, {
        subtree: true,
        childList: true,
    });
};