import json
from typing import Generator, List
from pathlib import Path

from models import Level


class LevelLoader:
    def __init__(self, filepath: Path) -> None:
        self.filepath = filepath
        self.levels: List[Level] = []
        self.__load_file()

    def __load_file(self) -> None:
        with open(self.filepath, "r", encoding="utf-8") as f:
            for line in f:
                line = line.strip()
                if not line:
                    continue

                data = json.loads(line)
                level = Level(**data)
                self.levels.append(level)

    def get_levels_generator(self) -> Generator[Level, None, None]:
        for level in self.levels:
            yield level


if __name__ == "__main__":
    ll = LevelLoader(Path("../data/levels.jsonl"))
    lg = ll.get_levels_generator()

    for i, lvl in enumerate(lg):
        if i > 5:
            break

        print(lvl.id, lvl.size)
        print(lvl.color_regions)
