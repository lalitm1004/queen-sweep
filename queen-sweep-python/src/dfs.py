from typing import Optional

from models import GameState


def depth_first_search(state: GameState) -> Optional[GameState]:
    if state.is_goal_state():
        return state

    # import time
    # time.sleep(1)

    for r, c in state.get_valid_queen_placements():
        new_state = state.place_queen(r, c)
        # new_state.pretty_print()
        solution = depth_first_search(new_state)
        if solution is not None:
            return solution

    return None
