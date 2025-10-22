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
    NEIGHBORS: Final[List[Tuple[int, int]]] = [
        (-1, -1), (-1,  0), (-1,  1), ( 0, -1),
        ( 0,  1), ( 1, -1), ( 1,  0), ( 1,  1),
    ]

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

        # block neighbors
        for dr, dc in GameState.NEIGHBORS:
            nr, nc = r + dr, c + dc
            if 0 <= nr < self.size and 0 <= nc < self.size:
                will_be_blocked[nr, nc] = True

        # block color region
        will_be_blocked[self.colors == queen_color] = True

        # check other color region in one pass
        unique_colors = np.unique(self.colors)
        for color in unique_colors:
            if color == queen_color:
                continue

            color_mask = self.colors == color

            # skip if region already has a queen
            if np.any(self.queen_mask & color_mask):
                continue

            # check if region has any empty cells that won't be blocked
            region_empty = (
                (self.states == CellState.EMPTY) & color_mask & ~will_be_blocked
            )
            if not np.any(region_empty):
                return False

        # valid move
        return True

    def get_valid_queen_placements(self) -> List[Tuple[int, int]]:
        unique_colors = np.unique(self.colors)

        color_sizes = []
        for color in unique_colors:
            color_mask = self.colors == color

            # skip regions that already have a queen
            if np.any(self.queen_mask & color_mask):
                continue

            # count empty cells in this region
            empty_in_region = np.sum((self.states == CellState.EMPTY) & color_mask)
            if empty_in_region > 0:
                color_sizes.append((empty_in_region, color))

        color_sizes.sort()

        valid_placements: List[Tuple[int, int]] = []
        for _, color in color_sizes:
            color_mask = self.colors == color
            empty_mask = (self.states == CellState.EMPTY) & color_mask

            # get coordinates of empty cells in this region
            rows, cols = np.where(empty_mask)

            # check each cell and add if valid
            for r, c in zip(rows, cols):
                if self.can_place_queen(r, c):
                    valid_placements.append((r, c))

        return valid_placements

    def place_queen(self, r: int, c: int) -> "GameState":
        # avoid mutation
        new_states = self.states.copy()
        new_colors = self.colors.copy()

        # block row and col
        new_states[r, :] = CellState.BLOCKED
        new_states[:, c] = CellState.BLOCKED

        # block neighbors
        for dr, dc in GameState.NEIGHBORS:
            nr, nc = r + dr, c + dc
            if 0 <= nr < self.size and 0 <= nc < self.size:
                new_states[nr, nc] = CellState.BLOCKED

        # block color region
        queen_color = self.colors[r, c]
        color_region_mask = self.colors == queen_color
        new_states[color_region_mask] = CellState.BLOCKED

        # place new queen
        new_states[r, c] = CellState.QUEEN

        return GameState(new_states, new_colors)

    def is_goal_state(self) -> bool:
        unique_colors = np.unique(self.colors)

        for color in unique_colors:
            color_mask = self.colors == color
            queen_count = np.sum(self.queen_mask & color_mask)

            if queen_count != 1:
                return False

        return True

    def pretty_print(self) -> None:
        init(autoreset=True)

        print("  " + " ".join(str(i) for i in range(self.size)))
        for r in range(self.size):
            row_str = f"{r} "
            for c in range(self.size):
                color_index = self.colors[r, c] % len(GameState.BACK_COLORS)
                back_color = GameState.BACK_COLORS[color_index]

                if self.states[r, c] == CellState.QUEEN:
                    # Queen: white text on colored background, bold
                    row_str += f"{back_color}{Fore.BLACK}♛ {Style.RESET_ALL}"
                elif self.states[r, c] == CellState.BLOCKED:
                    # Blocked: dim X on colored background
                    row_str += f"{back_color}{Fore.BLACK}{Style.DIM}✖ {Style.RESET_ALL}"
                else:
                    # Empty: colored background with space
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
