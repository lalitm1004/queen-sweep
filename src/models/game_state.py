import numpy as np
from typing import Final, List, Tuple

from cell_codec import CellState, encode_cell
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
        states = np.right_shift(self.grid, 5) & 0b11
        colors = self.grid & 0b11111

        # create a mask of where queens are
        queen_mask = states == CellState.QUEEN

        # CHECK 1: only 1 queen per row and column
        if np.any(np.sum(queen_mask, axis=1) > 1):
            return False

        if np.any(np.sum(queen_mask, axis=0) > 1):
            return False

        # CHECK 2: no queens adjacent to one another
        queen_positions = np.argwhere(queen_mask)
        for r, c in queen_positions:
            for dr, dc in GameState.NEIGHBOURS:
                nr, nc = r + dr, c + dc
                if 0 <= nr < self.size and 0 <= nc < self.size:
                    if queen_mask[nr, nc]:
                        return False

        # CHECK 3: every color region has exactly 1 queen
        queen_colors = colors[queen_mask]
        colors_with_queen, counts = np.unique(queen_colors, return_counts=True)

        if np.any(counts != 1):
            return False

        all_colors = np.unique(colors)
        if set(colors_with_queen) != set(all_colors):
            return False

        # winning state
        return True
