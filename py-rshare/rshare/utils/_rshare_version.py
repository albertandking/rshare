try:
    import rshare.rshare as rk

    _RSHARE_VERSION = rk.__version__
except ImportError:
    # This is only useful for documentation
    import warnings

    warnings.warn("Polars binary is missing!", stacklevel=2)
    _RSHARE_VERSION = ""


def get_rshare_version() -> str:
    """
    Return the version of the Python RShare package as a string.

    If the RShare binary is missing, returns an empty string.
    """
    return _RSHARE_VERSION
