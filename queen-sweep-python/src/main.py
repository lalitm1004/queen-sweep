from pathlib import Path

from loader import LevelLoader
from models import GameState
from dfs import depth_first_search


def main():
    level_loader = LevelLoader(Path("../data/levels.jsonl"))
    level_generator = level_loader.get_levels_generator()

    level = next(level_generator)
    level = next(level_generator)

    print(f"ID - {level.id} | Size - {level.size}")

    initial_game_state = GameState.from_level(level)
    initial_game_state.pretty_print()

    while True:
        r = int(input("r > "))
        c = int(input("c > "))

        initial_game_state = initial_game_state.place_queen(r, c)
        initial_game_state.pretty_print()


if __name__ == "__main__":
    main()
