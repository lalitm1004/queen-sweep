import numpy as np
from enum import IntEnum
from typing import Final, List, Tuple

from models import Level


class CellState(IntEnum):
    EMPTY = 0
    BLOCKED = 1
    QUEEN = 2


class GameState:
    # fmt: off
    NEIGHBOURS: Final[List[Tuple[int, int]]] = [
        (-1, -1), (-1,  0), (-1,  1), ( 0, -1),
        ( 0,  1), ( 1, -1), ( 1,  0), ( 1,  1),
    ]
    # fmt:on

    def __init__(self, states: np.ndarray, colors: np.ndarray) -> None:
        self.states = states
        self.colors = colors
        self.size = states.shape[0]

        self.queen_mask = self.states == CellState.QUEEN

    @classmethod
    def from_level(cls, level: Level) -> "GameState":
        size = level.size
        states = np.full((size, size), CellState.EMPTY, dtype=np.uint8)
        colors = np.zeros((size, size), dtype=np.uint8)

        for r in range(size):
            for c in range(size):
                color_char = level.color_regions[r][c].upper()
                colors[r, c] = ord(color_char) - ord("A")
        return cls(states, colors)

    def can_place_queen(self, r: int, c: int) -> bool: ...

    def is_goal_state(self) -> bool:
        # CHECK 1: only 1 queen per row and column
        if np.any(np.sum(self.queen_mask, axis=1) > 1):
            return False

        if np.any(np.sum(self.queen_mask, axis=0) > 1):
            return False

        # CHECK 2: no queens adjacent to one another
        queen_positions = np.argwhere(self.queen_mask)
        for r, c in queen_positions:
            for dr, dc in GameState.NEIGHBOURS:
                nr, nc = r + dr, c + dc
                if 0 <= nr < self.size and 0 <= nc < self.size:
                    if self.queen_mask[nr, nc]:
                        return False

        # CHECK 3: every color region has exactly 1 queen
        queen_colors = self.colors[self.queen_mask]
        colors_with_queen, counts = np.unique(queen_colors, return_counts=True)

        if np.any(counts != 1):
            return False

        all_colors = np.unique(self.colors)
        if set(colors_with_queen) != set(all_colors):
            return False

        # winning state
        return True
