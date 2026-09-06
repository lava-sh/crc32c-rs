__all__ = (
    "ReadableBuffer",
)

import sys
from typing import TypeAlias

if sys.version_info >= (3, 12):
    from collections.abc import Buffer
else:
    from typing_extensions import Buffer

ReadableBuffer: TypeAlias = Buffer
