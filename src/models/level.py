from typing import List
from pydantic import BaseModel


class Level(BaseModel):
    id: int
    size: int
    color_regions: List[List[str]]
