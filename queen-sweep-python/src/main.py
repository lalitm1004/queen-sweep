from pathlib import Path

from loader import LevelLoader
from models import GameState
from graph_search import GraphSearch


def main():
    level_loader = LevelLoader(Path("../data/levels.jsonl"))
    level_generator = level_loader.get_levels_generator()

    level = next(level_generator)
    level = level_loader.get_random_level()

    print(f"ID - {level.id} | Size - {level.size}")

    initial_game_state = GameState.from_level(level)
    initial_game_state.pretty_print()

    solved = GraphSearch.depth_first_search(initial_game_state)
    if solved is not None:
        solved.pretty_print()
    else:
        print("No Solution")


if __name__ == "__main__":
    main()
