import numpy as np
from typing import List

from cell_codec import CellState, encode_cell, decode_cell
from models import Level


class GameState:
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
