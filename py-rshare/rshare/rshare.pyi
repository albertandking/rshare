# rshare.pyi
from typing import List
import numpy as np


def get_ak_version(name: str) -> str:
    """
    Serialize a Python object to JSON including transforming and filtering data.

    This is effectively a standalone version of [`SchemaSerializer.to_json`][pydantic_core.SchemaSerializer.to_json].

    Arguments:
        name: The Python object to serialize.

    Raises:
        PydanticSerializationError: If serialization fails and no `fallback` function is provided.

    Returns:
       str
    """

def get_title(url: str) -> str: ...

def calculate_moving_average_rs(data: np.ndarray, window_size: int) -> np.ndarray: ...

def calculate_moving_average_in_rs(data: np.ndarray, window_size: int) -> np.ndarray: ...

def mycode() -> str: ...