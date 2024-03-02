# rshare.pyi
from typing import List
import numpy as np


def akversion(name: str, url:str) -> str:
    """
    Serialize a Python object to JSON including transforming and filtering data.

    This is effectively a standalone version of [`SchemaSerializer.to_json`][pydantic_core.SchemaSerializer.to_json].

    Arguments:
        value: The Python object to serialize.
        indent: If `None`, the JSON will be compact, otherwise it will be pretty-printed with the indent provided.
        include: A set of fields to include, if `None` all fields are included.
        exclude: A set of fields to exclude, if `None` no fields are excluded.
        by_alias: Whether to use the alias names of fields.
        exclude_none: Whether to exclude fields that have a value of `None`.
        round_trip: Whether to enable serialization and validation round-trip support.
        timedelta_mode: How to serialize `timedelta` objects, either `'iso8601'` or `'float'`.
        bytes_mode: How to serialize `bytes` objects, either `'utf8'` or `'base64'`.
        inf_nan_mode: How to serialize `Infinity`, `-Infinity` and `NaN` values, either `'null'` or `'constants'`.
        serialize_unknown: Attempt to serialize unknown types, `str(value)` will be used, if that fails
            `"<Unserializable {value_type} object>"` will be used.
        fallback: A function to call when an unknown value is encountered,
            if `None` a [`PydanticSerializationError`][pydantic_core.PydanticSerializationError] error is raised.

    Raises:
        PydanticSerializationError: If serialization fails and no `fallback` function is provided.

    Returns:
       JSON bytes.
    """

def get_title(url: str) -> str: ...

def calculate_moving_average_rs(data: np.ndarray, window_size: int) -> np.ndarray: ...

def calculate_moving_average_in_rs(data: np.ndarray, window_size: int) -> np.ndarray: ...

def mycode() -> str: ...