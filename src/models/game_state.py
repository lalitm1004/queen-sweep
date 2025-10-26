import numpy as np
from colorama import Fore, Back, Style, init
from enum import IntEnum
from typing import Final, List, Tuple

from models import Level


class CellState(IntEnum):
    EMPTY = 0
    BLOCKED = 1
    QUEEN = 2


class GameState:
    # fmt: off
    BACK_COLORS: Final[List[str]] = [
        Back.LIGHTRED_EX, Back.LIGHTGREEN_EX, Back.LIGHTYELLOW_EX, Back.LIGHTBLUE_EX,
        Back.LIGHTMAGENTA_EX, Back.LIGHTCYAN_EX, Back.RED, Back.GREEN, Back.YELLOW,
        Back.BLUE, Back.MAGENTA, Back.CYAN, Back.WHITE, Back.BLACK,
    ]
    # fmt:on

    def __init__(self, states: np.ndarray, colors: np.ndarray) -> None:
        self.states = states
        self.colors = colors
        self.size = states.shape[0]

        self.queen_mask = self.states == CellState.QUEEN
        self.empty_mask = self.states == CellState.EMPTY
        self.unique_colors = np.unique(colors)

        self.color_masks = {color: self.colors == color for color in self.unique_colors}

        self.colors_with_queens = set(self.colors[self.queen_mask].tolist())

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

    def can_place_queen(self, r: int, c: int) -> bool:
        if self.states[r, c] != CellState.EMPTY:
            return False

        queen_color = self.colors[r, c]

        will_be_blocked = np.zeros((self.size, self.size), dtype=bool)

        # block row and column
        will_be_blocked[r, :] = True
        will_be_blocked[:, c] = True

        # block neighbors using vectorized operations
        neighbor_rows = np.clip(
            r + np.array([-1, -1, -1, 0, 0, 1, 1, 1]), 0, self.size - 1
        )
        neighbor_cols = np.clip(
            c + np.array([-1, 0, 1, -1, 1, -1, 0, 1]), 0, self.size - 1
        )
        will_be_blocked[neighbor_rows, neighbor_cols] = True

        # block color region using precomputed mask
        will_be_blocked[self.color_masks[queen_color]] = True

        # check all other color regions in vectorized manner
        for color in self.unique_colors:
            if color == queen_color or color in self.colors_with_queens:
                continue

            # check if region has any empty cells that won't be blocked
            region_valid = self.empty_mask & self.color_masks[color] & ~will_be_blocked
            if not np.any(region_valid):
                return False

        return True

    def get_valid_queen_placements(self) -> List[Tuple[int, int]]:
        colors_needing_queens = [
            c for c in self.unique_colors if c not in self.colors_with_queens
        ]

        # precompute empty cells for each region
        color_sizes = []
        for color in colors_needing_queens:
            empty_in_region = np.sum(self.empty_mask & self.color_masks[color])
            if empty_in_region > 0:
                color_sizes.append((empty_in_region, color))

        color_sizes.sort()

        valid_placements: List[Tuple[int, int]] = []
        for _, color in color_sizes:
            # get all empty cells in this region at once
            empty_in_region = self.empty_mask & self.color_masks[color]
            rows, cols = np.where(empty_in_region)

            # check each cell
            for r, c in zip(rows, cols):
                if self.can_place_queen(r, c):
                    valid_placements.append((r, c))

        return valid_placements

    def place_queen(self, r: int, c: int) -> "GameState":
        # avoid mutation
        new_states = self.states.copy()

        # block row and column
        new_states[r, :] = CellState.BLOCKED
        new_states[:, c] = CellState.BLOCKED

        # block neighbors using vectorized clip
        neighbor_rows = np.clip(
            r + np.array([-1, -1, -1, 0, 0, 1, 1, 1]), 0, self.size - 1
        )
        neighbor_cols = np.clip(
            c + np.array([-1, 0, 1, -1, 1, -1, 0, 1]), 0, self.size - 1
        )
        new_states[neighbor_rows, neighbor_cols] = CellState.BLOCKED

        # block color region using precomputed mask
        queen_color = self.colors[r, c]
        new_states[self.color_masks[queen_color]] = CellState.BLOCKED

        # place new queen
        new_states[r, c] = CellState.QUEEN

        return GameState(new_states, self.colors)

    def is_goal_state(self) -> bool:
        for color in self.unique_colors:
            queen_count = np.sum(self.queen_mask & self.color_masks[color])
            if queen_count != 1:
                return False
        return True

    def pretty_print(self) -> None:
        init(autoreset=True)

        print("   " + " ".join(str(i) for i in range(self.size)))
        for r in range(self.size):
            row_str = f"{r:2} "
            for c in range(self.size):
                color_index = self.colors[r, c] % len(GameState.BACK_COLORS)
                back_color = GameState.BACK_COLORS[color_index]

                if self.states[r, c] == CellState.QUEEN:
                    row_str += f"{back_color}{Fore.BLACK}♛ {Style.RESET_ALL}"
                elif self.states[r, c] == CellState.BLOCKED:
                    row_str += f"{back_color}{Fore.BLACK}{Style.DIM}✖ {Style.RESET_ALL}"
                else:
                    row_str += f"{back_color}  {Style.RESET_ALL}"

            print(row_str)
        print()

    def __hash__(self) -> int:
        return hash((self.states.tobytes(), self.colors.tobytes()))

    def __eq__(self, other) -> bool:
        if not isinstance(other, GameState):
            return False
        return np.array_equal(self.states, other.states) and np.array_equal(
            self.colors, other.colors
        )
