# rshare.pyi
from typing import List

def fetch_title(url: str) -> str: ...


def calculate_moving_average_rs(
    data: List[float], window_size: int
) -> List[float]: ...