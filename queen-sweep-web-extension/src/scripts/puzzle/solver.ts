import { extractColorRegions } from './extractor';
import { applySolution } from './applier';
import { sendSolveRequest } from '../send-solve-request';
import { injectSolveButton, updateButtonState, type ButtonConfig } from '../button';

export const solvePuzzle = async (): Promise<void> => {
    try {
        console.log('[QueenSweep] Starting puzzle solve');

        const loadingConfig: ButtonConfig = { state: 'loading' };
        injectSolveButton(loadingConfig, () => { });

        console.log('[QueenSweep] Extracting color regions');
        const regions = extractColorRegions();

        const result = await sendSolveRequest(regions);

        if (!result.success) {
            console.warn('[QueenSweep] No solution found');
            updateButtonState('idle');
            return;
        }

        console.log('[QueenSweep] Solution received, updating button');

        const readyConfig: ButtonConfig = {
            state: 'ready',
            queenPositions: result.queenPositions,
        };

        injectSolveButton(readyConfig, () => applySolution(result.queenPositions));
    } catch (error) {
        console.error('[QueenSweep] Error solving puzzle:', error);
        updateButtonState('idle');
    }
};