from enum import IntEnum
from typing import Tuple


class CellState(IntEnum):
    EMPTY = 0
    BLOCKED = 1
    QUEEN = 2


def encode_cell(state: CellState, color: str) -> int:
    color_val = ord(color.upper()) - ord("A")
    if not (0 <= color_val <= 25):
        raise ValueError(f"Color must be an uppercase letter A-Z, got {color}")

    return (state << 5) | (color_val & 0b11111)


def decode_cell(cell: int) -> Tuple[CellState, str]:
    state = CellState((cell >> 5) & 0b11)

    color_val = cell & 0b11111
    color = chr(color_val + ord("A"))

    return (state, color)
