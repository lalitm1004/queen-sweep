import numpy as np
from typing import Final, List, Tuple

from cell_codec import CellState, encode_cell, decode_cell
from models import Level


class GameState:
    NEIGHBOURS: Final[List[Tuple[int, int]]] = [
        (dr, dc) for dr in (-1, 0, 1) for dc in (-1, 0, 1) if not (dr == 0 and dc == 0)
    ]

    def __init__(self, grid: np.ndarray, size: int) -> None:
        self.grid = grid
        self.size = size

    @classmethod
    def from_level(cls, level: Level) -> "GameState":
        size = level.size

        grid = np.array(
            [
                [
                    encode_cell(CellState.EMPTY, level.color_regions[r][c])
                    for c in range(size)
                ]
                for r in range(size)
            ],
            dtype=np.uint8,
        )

        return cls(grid, size)

    def is_goal_state(self) -> bool:
        region_queen_count = {}

        rows_with_queen = set()
        cols_with_queen = set()

        for r in range(self.size):
            for c in range(self.size):
                state, color = decode_cell(self.grid[r, c])

                if state != CellState.QUEEN:
                    continue

                if r in rows_with_queen or c in cols_with_queen:
                    return False

                rows_with_queen.add(r)
                cols_with_queen.add(c)

                for dr, dc in GameState.NEIGHBOURS:
                    nr, nc = r + dr, c + dc
                    if 0 <= nr < self.size and 0 <= nc < self.size:
                        n_state, _ = decode_cell(self.grid[nr, nc])
                        if n_state == CellState.QUEEN:
                            return False

                region_queen_count[color] = region_queen_count.get(color, 0) + 1

        for r in range(self.size):
            for c in range(self.size):
                _, color = decode_cell(self.grid[r, c])
                if color not in region_queen_count:
                    # some region has no queen
                    return False

        # check that no region has more than one queen
        if any(count != 1 for count in region_queen_count.values()):
            return False

        return True
