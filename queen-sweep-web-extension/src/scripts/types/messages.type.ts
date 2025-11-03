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
    message?: string
}

export type SolveResponse = SolveResponse_Success | SolveResponse_Failure