export interface SolveRequest {
    type: 'solve-request';
    colorRegions: number[][];
}

export interface SolveResponse_Success {
    type: 'solve-response-success';
    success: true;
    queenPositions: [number, number][];
    size?: number;
    numStatesVisited?: number;
}

export interface SolveResponse_Failure {
    type: 'solve-response-failure';
    success: false;
    code: ErrorCode;
    message?: string;
}

export type SolveResponse = SolveResponse_Success | SolveResponse_Failure;

export const SOLVER_ERROR_CODES = {
    // client-side codes
    WASM_NOT_INITIALIZED: 'WASM_NOT_INITIALIZED',
    BOARD_UNSOLVABLE: 'BOARD_UNSOLVABLE',
    // build errors reported by the wasm layer (BuildErrorCode)
    EMPTY: 'EMPTY',
    NON_SQUARE: 'NON_SQUARE',
    NOT_PERFECT_SQUARE: 'NOT_PERFECT_SQUARE',
    UNSUPPORTED_SIZE: 'UNSUPPORTED_SIZE',
    INVALID_INPUT: 'INVALID_INPUT',
    BOARD_INIT_FAILED: 'BOARD_INIT_FAILED',
} as const;

export type ErrorCode = typeof SOLVER_ERROR_CODES[keyof typeof SOLVER_ERROR_CODES];
