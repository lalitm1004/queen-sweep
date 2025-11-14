export interface SolveRequest {
    type: 'solve-request'
    colorRegions: number[][],
};

export interface SolveResponse_Success {
    type: 'solve-response-success'
    success: true
    queenPositions: number[][],
}

export interface SolveResponse_Failure {
    type: 'solve-response-failure'
    success: false
    code: string
    message?: string
}

export type SolveResponse = SolveResponse_Success | SolveResponse_Failure;

export const SOLVER_ERROR_CODES = {
    WASM_NOT_INITIALIZED: 'WASM_NOT_INITIALIZED',
    BOARD_INIT_FAILED: 'BOARD_INIT_FAILED',
    BOARD_UNSOLVABLE: 'BOARD_UNSOLVABLE'
} as const;
export type ErrorCode = typeof SOLVER_ERROR_CODES[keyof typeof SOLVER_ERROR_CODES];