import copy
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

    def can_place_queen(self, r: int, c: int) -> Tuple[bool, int]:
        # CHECK 0: bounds check
        if not (0 <= r < self.size) or not (0 <= c < self.size):
            raise IndexError(
                f"Cell ({r}, {c}) is out of bounds for board of size {self.size}"
            )

        # CHECK 1: cell must be empty
        if self.states[r, c] != CellState.EMPTY:
            return False, 0

        # CHECK 2: no other queen in the same row or column
        if np.any(self.queen_mask[r, :]) or np.any(self.queen_mask[:, c]):
            return False, 0

        # CHECK 3: no adjacent queens
        for dr, dc in GameState.NEIGHBOURS:
            nr, nc = r + dr, c + dc
            if 0 <= nr < self.size and 0 <= nc < self.size:
                if self.queen_mask[nr, nc]:
                    return False, 0

        # CHECK 4: 1-step lookahead
        blocked_mask: np.ndarray = self.states != CellState.EMPTY
        temp_mask = blocked_mask.copy()

        # block row, column, and adjacent cells
        temp_mask[r, :] = True
        temp_mask[:, c] = True
        for dr, dc in GameState.NEIGHBOURS:
            nr, nc = r + dr, c + dc
            if 0 <= nr < self.size and 0 <= nc < self.size:
                temp_mask[nr, nc] = True

        # count remaining colors
        remaining_colors = 0
        all_colors = np.unique(self.colors)
        for color in all_colors:
            # check if the color already has a queen
            has_queen = np.any(self.queen_mask & (self.colors == color))
            if has_queen:
                continue

            # otherwise, ensure at least one available cell remains
            if np.any((self.colors == color) & (~temp_mask)):
                remaining_colors += 1
            else:
                return False, 0  # cannot place queen, color would be unsolvable

        return True, remaining_colors

    def get_valid_queen_placements(self) -> List[Tuple[int, int]]:
        positions_with_remaining_colors: List[Tuple[Tuple[int, int], int]] = []

        for r in range(self.size):
            for c in range(self.size):
                valid, remaining_colors = self.can_place_queen(r, c)
                if valid:
                    positions_with_remaining_colors.append(((r, c), remaining_colors))

        # sort by remaining_colors ascending
        positions_with_remaining_colors.sort(key=lambda x: x[1])

        return [pos for pos, _ in positions_with_remaining_colors]

    def place_queen(self, r: int, c: int) -> "GameState":
        if not self.can_place_queen(r, c):
            raise ValueError(f"Cannot place queen at ({r}, {c})")

        new_state = copy.deepcopy(self)

        color = new_state.colors[r, c]
        new_state.states[new_state.colors == color] = CellState.BLOCKED

        # block entire row and col
        new_state.states[r, :] = CellState.BLOCKED
        new_state.states[:, c] = CellState.BLOCKED

        # block adjacent cells
        for dr, dc in GameState.NEIGHBOURS:
            nr, nc = r + dr, c + dc
            if 0 <= nr < new_state.size and 0 <= nc < new_state.size:
                new_state.states[nr, nc] = CellState.BLOCKED

        # place new queen and update queen_mask
        new_state.states[r, c] = CellState.QUEEN
        new_state.queen_mask = new_state.states == CellState.QUEEN

        return new_state

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

    # TEMPORARY
    def pretty_print(self) -> None:
        # ANSI color codes (bright variants)
        COLORS = [
            "\033[91m",  # red
            "\033[92m",  # green
            "\033[93m",  # yellow
            "\033[94m",  # blue
            "\033[95m",  # magenta
            "\033[96m",  # cyan
            "\033[97m",  # white
            "\033[90m",  # bright black / gray
            "\033[31m",  # dark red
            "\033[32m",  # dark green
            "\033[33m",  # dark yellow
            "\033[34m",  # dark blue
            "\033[35m",  # dark magenta
            "\033[36m",  # dark cyan
            "\033[37m",  # light gray
        ]
        RESET = "\033[0m"

        for r in range(self.size):
            row_str = ""
            for c in range(self.size):
                color_index = self.colors[r, c] % len(COLORS)
                color = COLORS[color_index]

                if self.states[r, c] == CellState.QUEEN:
                    row_str += f"{color}\033[1mQ{RESET} "
                elif self.states[r, c] == CellState.BLOCKED:
                    row_str += f"{color}x{RESET} "
                else:  # EMPTY cell
                    row_str += f"{color}.{RESET} "
            print(row_str.strip())
        print()
