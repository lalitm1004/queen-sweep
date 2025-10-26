from typing import Optional, Set

from models import GameState


class GraphSearch:
    @staticmethod
    def depth_first_search(state: GameState) -> Optional[GameState]:
        seen: Set[GameState] = set()
        return GraphSearch.__dfs_helper(state, seen)

    @staticmethod
    def __dfs_helper(state: GameState, seen: Set[GameState]) -> Optional[GameState]:
        if state in seen:
            return None

        seen.add(state)

        if state.is_goal_state():
            return state

        for r, c in state.get_valid_queen_placements():
            new_state = state.place_queen(r, c)
            solution = GraphSearch.__dfs_helper(new_state, seen)
            if solution is not None:
                return solution

        return None
