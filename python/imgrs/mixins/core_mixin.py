"""
Core image operations mixin - I/O, constructors, properties
"""

from pathlib import Path
from typing import Any, Optional, Tuple, Union

try:
    import numpy as np
    HAS_NUMPY = True
except ImportError:
    np = None
    HAS_NUMPY = False


class CoreMixin:
    """Mixin for core image operations"""

    def __init__(self, rust_image=None):
        """Initialize an Image instance."""
        from .._core import Image as RustImage
        
        if RustImage is None:
            raise ImportError(
                "Imgrs Rust extension not available. "
                "Please install with: pip install imgrs"
            )

        if rust_image is None:
            rust_image = RustImage()
        self._rust_image = rust_image

    @classmethod
    def open(
        cls,
        fp: Union[str, Path, bytes],
        mode: Optional[str] = None,
        formats: Optional[list] = None,
    ) -> "Image":
        """
        Open an image file.

        Args:
            fp: File path, file object, or bytes
            mode: Optional mode hint
            formats: Optional list of formats to try

        Returns:
            Image instance
        """
        from .._core import Image as RustImage
        
        if isinstance(fp, Path):
            fp = str(fp)

        rust_image = RustImage.open(fp)
        return cls(rust_image)

    @classmethod
    def new(
        cls,
        mode: str,
        size: Tuple[int, int],
        color: Union[int, Tuple[int, ...], str] = 0,
    ) -> "Image":
        """
        Create a new image with the given mode and size.

        Args:
            mode: Image mode (e.g., 'RGB', 'RGBA', 'L', 'LA')
            size: Image size as (width, height)
            color: Fill color

        Returns:
            New Image instance
        """
        from .._core import Image as RustImage
        
        # Convert color to RGBA tuple
        rgba_color = cls._parse_color(color, mode)

        rust_image = RustImage.new(mode, size, rgba_color)
        return cls(rust_image)

    @classmethod
    def fromarray(
        cls,
        obj: Any,
        mode: Optional[str] = None,
    ) -> "Image":
        """
        Create an image from a numpy array.

        Args:
            obj: Numpy array
            mode: Optional mode hint

        Returns:
            Image instance
        """
        from .._core import Image as RustImage
        
        if not HAS_NUMPY:
            raise ImportError(
                "NumPy is required for fromarray. Install with: pip install numpy"
            )

        if not isinstance(obj, np.ndarray):
            raise TypeError(f"Expected numpy.ndarray, got {type(obj)}")

        # Validate array
        if obj.ndim not in (2, 3):
            raise ValueError(
                f"Expected 2D or 3D array, got {obj.ndim}D array"
            )

        # Convert to contiguous array if needed
        if not obj.flags["C_CONTIGUOUS"]:
            obj = np.ascontiguousarray(obj)

        rust_image = RustImage.fromarray(obj, mode)
        return cls(rust_image)

    @staticmethod
    def _parse_color(
        color: Union[int, Tuple[int, ...], str], mode: str
    ) -> Tuple[int, int, int, int]:
        """Parse color input into RGBA tuple."""
        # Handle integer input
        if isinstance(color, int):
            if mode in ("L", "LA"):
                return (color, color, color, 255)
            return (color, color, color, 255)

        # Handle tuple input
        if isinstance(color, (tuple, list)):
            if len(color) == 3:
                return tuple(color) + (255,)
            elif len(color) == 4:
                return tuple(color)
            elif len(color) == 1:
                return (color[0], color[0], color[0], 255)
            else:
                raise ValueError(f"Invalid color tuple length: {len(color)}")

        # Handle string color names
        if isinstance(color, str):
            color_map = {
                "black": (0, 0, 0, 255),
                "white": (255, 255, 255, 255),
                "red": (255, 0, 0, 255),
                "green": (0, 255, 0, 255),
                "blue": (0, 0, 255, 255),
                "yellow": (255, 255, 0, 255),
                "cyan": (0, 255, 255, 255),
                "magenta": (255, 0, 255, 255),
                "transparent": (0, 0, 0, 0),
            }
            color_lower = color.lower()
            if color_lower in color_map:
                return color_map[color_lower]
            raise ValueError(f"Unknown color name: {color}")

        raise TypeError(f"Invalid color type: {type(color)}")

    def save(self, fp: Union[str, Path], format: Optional[str] = None) -> None:
        """
        Save the image to a file.

        Args:
            fp: File path or file object
            format: Optional format override
        """
        if isinstance(fp, Path):
            fp = str(fp)
        self._rust_image.save(fp, format)

    def to_bytes(self) -> bytes:
        """Convert image to bytes."""
        return self._rust_image.to_bytes()

    def copy(self) -> "Image":
        """Create a copy of the image."""
        return self.__class__(self._rust_image.copy())

    # Properties
    @property
    def size(self) -> Tuple[int, int]:
        """Image size as (width, height)."""
        return self._rust_image.size

    @property
    def width(self) -> int:
        """Image width in pixels."""
        return self._rust_image.width

    @property
    def height(self) -> int:
        """Image height in pixels."""
        return self._rust_image.height

    @property
    def mode(self) -> str:
        """Image mode (e.g., 'RGB', 'RGBA', 'L')."""
        return self._rust_image.mode

    @property
    def format(self) -> Optional[str]:
        """Image format (e.g., 'JPEG', 'PNG')."""
        return self._rust_image.format

    @property
    def info(self) -> dict:
        """Image metadata dictionary."""
        return {}

    def __repr__(self) -> str:
        """String representation of the image."""
        return self._rust_image.__repr__()

    def __eq__(self, other) -> bool:
        """Compare two images for equality."""
        if not isinstance(other, self.__class__):
            return False

        return (
            self.size == other.size
            and self.mode == other.mode
            and self.to_bytes() == other.to_bytes()
        )

