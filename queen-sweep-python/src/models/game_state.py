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

    def can_place_queen(self, r: int, c: int) -> Tuple[bool, int]: ...

    def get_valid_queen_placements(self) -> List[Tuple[int, int]]: ...

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

    def is_goal_state(self) -> bool: ...

    def pretty_print(self) -> None:
        init(autoreset=True)

        BACK_COLORS = [
            Back.LIGHTRED_EX,
            Back.LIGHTGREEN_EX,
            Back.LIGHTYELLOW_EX,
            Back.LIGHTBLUE_EX,
            Back.LIGHTMAGENTA_EX,
            Back.LIGHTCYAN_EX,
            Back.RED,
            Back.GREEN,
            Back.YELLOW,
            Back.BLUE,
            Back.MAGENTA,
            Back.CYAN,
            Back.WHITE,
            Back.BLACK,
        ]

        print("  " + " ".join(str(i) for i in range(self.size)))
        for r in range(self.size):
            row_str = f"{r} "
            for c in range(self.size):
                color_index = self.colors[r, c] % len(BACK_COLORS)
                back_color = BACK_COLORS[color_index]

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
