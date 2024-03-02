from .rshare import *
from rshare.fetch import *

from rshare.utils._rshare_version import get_rshare_version

__version__: str = get_rshare_version()
del get_rshare_version
