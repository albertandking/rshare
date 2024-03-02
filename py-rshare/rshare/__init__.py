from .rshare import (
    __version__,
    akversion,
    get_title,
    calculate_moving_average_rs,
    calculate_moving_average_in_rs,
    mycode,
)
from rshare.fetch import (
    pyakversion,
    calculate_moving_average_py,
    fetch_name,
)

from rshare.utils._rshare_version import get_rshare_version

__version__: str = get_rshare_version()
del get_rshare_version
