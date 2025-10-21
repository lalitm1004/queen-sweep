from pathlib import Path

from loader import LevelLoader
from models import GameState
from dfs import depth_first_search


def main():
    level_loader = LevelLoader(Path("../data/levels.jsonl"))
    level_generator = level_loader.get_levels_generator()

    level = next(level_generator)

    print(level.id, level.size)
    for i in range(level.size):
        print(level.color_regions[i])

    initial_game_state = GameState.from_level(level)
    initial_game_state.pretty_print()

    solved_game_state = depth_first_search(initial_game_state)
    if solved_game_state is None:
        print("No Solution Found")
        return

    solved_game_state.pretty_print()


if __name__ == "__main__":
    main()
