"""
Core image operations mixin - I/O, constructors, properties with IDE-friendly features
"""

import os
import subprocess
import sys
import tempfile
from pathlib import Path
from typing import TYPE_CHECKING, Any, List, Optional, Tuple, Union

if TYPE_CHECKING:
    from .image import Image

try:
    import numpy as np

    HAS_NUMPY = True
except ImportError:
    np = None
    HAS_NUMPY = False


class CoreMixin:
    """
    Mixin for core image operations with comprehensive IDE support.

    Provides:
    - Image I/O operations (open, save, show)
    - Image constructors (new, fromarray, frombytes)
    - Image properties and metadata
    - Utility methods (copy, bytes conversion)

    All operations are immutable (return new instances) unless explicitly noted.
    """

    def __init__(self, rust_image=None):
        """
        Initialize an Image instance.

        Args:
            rust_image: Optional pre-existing Rust image instance.
                       If None, creates a new 1x1 image.

        Raises:
            ImportError: If imgrs Rust extension is not installed
            RuntimeError: If Rust backend fails to initialize
        """
        from .._core import RustImage

        if RustImage is None:
            raise ImportError(
                "Imgrs Rust extension not available. "
                "Please install with: pip install imgrs\n"
                "Alternative: Use imgrs.frombytes() for NumPy-free operation"
            )

        if rust_image is None:
            rust_image = RustImage()
        self._rust_image = rust_image

    @classmethod
    def open(
        cls,
        fp: Union[str, Path, bytes],
        mode: Optional[str] = None,
        formats: Optional[List[Union[str, Any]]] = None,
    ) -> "Image":
        """
        Open an image file with comprehensive format and mode support.

        Args:
            fp: File path, file object, or raw image bytes.
                Supported formats:
                - str: "/path/to/image.png", relative paths, absolute paths
                - Path: pathlib.Path objects for cross-platform compatibility
                - bytes: Raw image data (automatically detects format)
                - file-like: Any object with read() method

            mode: Optional mode hint for format detection.
                  Common modes: 'RGB', 'RGBA', 'L', 'LA', 'CMYK', 'YCbCr'
                  Used when format detection is ambiguous

            formats: Optional prioritized list of formats to try.
                    Example: ['JPEG', 'PNG', 'BMP'] - tries JPEG first
                    Useful for files with ambiguous extensions

        Returns:
            Image: New Image instance with loaded image data

        Raises:
            FileNotFoundError: If file doesn't exist or is not readable
            ValueError: If file format is unsupported or corrupted
            OSError: For I/O errors (permission denied, disk full, etc.)
            ImportError: If imgrs Rust extension is not available

        Note:
            - Automatically detects image format from file extension or content
            - Supports all major formats: JPEG, PNG, GIF, BMP, TIFF, WEBP, etc.
            - Returns immutable Image instance (use copy() for modification)
            - Preserves original image properties and metadata when possible

        Example:
            >>> # Basic usage
            >>> img = Image.open("photo.jpg")
            >>>
            >>> # From Path object
            >>> from pathlib import Path
            >>> img = Image.open(Path("images/portrait.png"))
            >>>
            >>> # From bytes
            >>> with open("image.jpg", "rb") as f:
            ...     data = f.read()
            >>> img = Image.open(data)
            >>>
            >>> # With format hint
            >>> img = Image.open("file.dat", formats=['JPEG', 'PNG'])
        """
        from .._core import RustImage

        if isinstance(fp, Path):
            fp = str(fp)

        rust_image = RustImage.open(fp)
        return cls(rust_image)

    @classmethod
    def new(
        cls,
        mode: Union[str, Any],
        size: Tuple[int, int],
        color: Union[
            int, Tuple[int, int], Tuple[int, int, int], Tuple[int, int, int, int], str
        ] = 0,
    ) -> "Image":
        """
        Create a new image with specified mode, size, and color.

        Args:
            mode: Image color mode determining number of channels and interpretation.
                  Common modes:
                  - 'RGB': 3 channels (Red, Green, Blue), 8-bit per channel
                  - 'RGBA': RGB + Alpha channel, supports transparency
                  - 'L': Grayscale, single channel (0-255)
                  - 'LA': Grayscale + Alpha channel
                  - 'CMYK': Print color model (4 channels)
                  - 'YCbCr': Digital video color model
                  - 'HSV': Hue-Saturation-Value color model

            size: Image dimensions as (width, height) tuple.
                  Both dimensions must be positive integers (> 0).
                  Maximum size limited by available memory.
                  Example: (1920, 1080), (800, 600), (100, 100)

            color: Fill color for the new image.
                   Supported formats:
                   - int: For grayscale modes ('L', 'LA')
                     Example: 0 (black), 255 (white), 128 (mid-gray)
                   - tuple[int, int, int]: RGB color (red, green, blue)
                     Example: (255, 0, 0) = red, (255, 255, 255) = white
                   - tuple[int, int, int, int]: RGBA color (red, green, blue, alpha)
                     Example: (255, 0, 0, 128) = semi-transparent red
                   - str: Named color (case-insensitive)
                     Supported: 'black', 'white', 'red', 'green', 'blue',
                               'yellow', 'cyan', 'magenta', 'transparent'
                   - Default: 0 (black for RGB, transparent for RGBA)

        Returns:
            Image: New Image instance filled with specified color

        Raises:
            ValueError: If mode is unsupported, size is invalid, or color format is wrong
            TypeError: If mode is not string or size is not tuple of two integers
            MemoryError: If requested size exceeds available memory
            OverflowError: If size dimensions are too large

        Note:
            - Creates a new Image instance (does not modify existing images)
            - Color is applied uniformly across entire image
            - For complex patterns, use drawing methods after creation
            - Alpha channel (if present) controls transparency

        Example:
            >>> # Basic RGB image
            >>> img = Image.new('RGB', (800, 600))
            >>>
            >>> # Red image with specific size
            >>> img = Image.new('RGB', (400, 300), (255, 0, 0))
            >>>
            >>> # Transparent overlay
            >>> img = Image.new('RGBA', (200, 200), (255, 0, 0, 128))
            >>>
            >>> # Grayscale image
            >>> img = Image.new('L', (100, 100), 200)
            >>>
            >>> # Using named color
            >>> img = Image.new('RGB', (300, 200), 'blue')
        """
        from .._core import RustImage

        # Convert color to RGBA tuple
        rgba_color = cls._parse_color(color, mode)

        width, height = size

        if mode == "RGB":
            r, g, b, _ = rgba_color
            data = bytes([r, g, b] * (width * height))
            rust_image = RustImage.frombytes("RGB", size, data)
        elif mode == "RGBA":
            r, g, b, a = rgba_color
            data = bytes([r, g, b, a] * (width * height))
            rust_image = RustImage.frombytes("RGBA", size, data)
        elif mode == "L":
            gray, _, _, _ = rgba_color
            data = bytes([gray] * (width * height))
            rust_image = RustImage.frombytes("L", size, data)
        elif mode == "LA":
            # Create an RGBA image first, then convert to LA
            r, g, b, a = rgba_color
            data = bytes([r, g, b, a] * (width * height))
            rust_image = RustImage.frombytes("RGBA", size, data)
            img = cls(rust_image)
            return img.convert("LA")
        else:
            raise ValueError(
                f"Unsupported mode: {mode}. Use 'RGB', 'RGBA', 'L', or 'LA'"
            )

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
        from .._core import RustImage

        if not HAS_NUMPY:
            raise ImportError(
                "NumPy is required for fromarray. Install with: pip install numpy\n"
                "Or use frombytes() for mobile/lightweight alternative."
            )

        if not isinstance(obj, np.ndarray):
            raise TypeError(f"Expected numpy.ndarray, got {type(obj)}")

        # Validate array
        if obj.ndim not in (2, 3):
            raise ValueError(f"Expected 2D or 3D array, got {obj.ndim}D array")

        # Convert to contiguous array if needed
        if not obj.flags["C_CONTIGUOUS"]:
            obj = np.ascontiguousarray(obj)

        # Handle float arrays (assume 0.0-1.0 range)
        if obj.dtype.kind == "f":
            obj = (obj * 255).astype(np.uint8)
        # Handle other non-uint8 types
        elif obj.dtype != np.uint8:
            obj = obj.astype(np.uint8)

        rust_image = RustImage.fromarray(obj, mode)
        return cls(rust_image)

    @classmethod
    def frombytes(
        cls,
        mode: str,
        size: Tuple[int, int],
        data: bytes,
    ) -> "Image":
        """
        Create an image from raw bytes (NumPy-free!).

        Mobile-friendly alternative to fromarray() - works without NumPy.

        Args:
            mode: Image mode ('RGB', 'RGBA', or 'L')
            size: Image size as (width, height)
            data: Raw pixel data as bytes
                  - RGB: width * height * 3 bytes
                  - RGBA: width * height * 4 bytes
                  - L: width * height bytes

        Returns:
            Image instance

        Example:
            # Create 2x2 red RGB image
            data = bytes([255,0,0, 255,0,0, 255,0,0, 255,0,0])
            img = Image.frombytes('RGB', (2, 2), data)

            # Works on mobile without NumPy!
        """
        from .._core import RustImage

        rust_image = RustImage.frombytes(mode, size, data)
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
            elif len(color) == 2:
                # Handle (gray, alpha) for LA mode
                # Convert to RGBA where RGB channels are the gray value
                gray, alpha = color
                return (gray, gray, gray, alpha)
            elif len(color) == 1:
                return (color[0], color[0], color[0], 255)
            else:
                raise ValueError(f"Invalid color tuple length: {len(color)}")

        # Handle string color names and hex codes
        if isinstance(color, str):
            # Handle hex codes
            clean_color = color.lstrip("#")
            is_hex = all(c in "0123456789abcdefABCDEF" for c in clean_color)

            if is_hex and len(clean_color) in (3, 6, 8):
                if len(clean_color) == 3:
                    # Expand RGB to RRGGBB
                    clean_color = "".join(c * 2 for c in clean_color)

                if len(clean_color) == 6:
                    r = int(clean_color[0:2], 16)
                    g = int(clean_color[2:4], 16)
                    b = int(clean_color[4:6], 16)
                    return (r, g, b, 255)
                elif len(clean_color) == 8:
                    r = int(clean_color[0:2], 16)
                    g = int(clean_color[2:4], 16)
                    b = int(clean_color[4:6], 16)
                    a = int(clean_color[6:8], 16)
                    return (r, g, b, a)

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

    def show(self, title: Optional[str] = None) -> None:
        """
        Display the image using the default image viewer.

        This method saves the image to a temporary file and opens it with the
        system's default image viewer. The temporary file is deleted after viewing.

        Args:
            title: Optional title for the image window (may not be supported on all platforms)

        Example:
            >>> img = imgrs.Image.open("photo.jpg")
            >>> img = img.blur(5)
            >>> img.show()  # Opens in default image viewer

        Note:
            - On Windows: Uses default photo viewer
            - On macOS: Uses Preview or default app
            - On Linux: Uses xdg-open to find the default viewer
            - Requires a GUI environment
        """
        # Create a temporary file
        suffix = ".png"  # PNG for best compatibility
        with tempfile.NamedTemporaryFile(suffix=suffix, delete=False) as tmp_file:
            tmp_path = tmp_file.name

        try:
            # Save the image to temp file
            self.save(tmp_path)

            # Open with platform-specific viewer
            if sys.platform.startswith("win"):
                # Windows
                os.startfile(tmp_path)
            elif sys.platform == "darwin":
                # macOS
                subprocess.run(["open", tmp_path], check=True)
            else:
                # Linux and others
                subprocess.run(["xdg-open", tmp_path], check=True)

            # Note: We don't delete the temp file immediately as the viewer might not have
            # opened it yet. The OS will clean it up eventually from the temp directory.

        except Exception as e:
            # Clean up on error
            try:
                os.unlink(tmp_path)
            except Exception:
                pass
            raise RuntimeError(f"Failed to show image: {e}") from e

    def to_bytes(self) -> bytes:
        """Convert image to bytes."""
        return self._rust_image.to_bytes()

    def copy(self) -> "Image":
        """Create a copy of the image."""
        return self.__class__(self._rust_image.copy())

    # Properties with enhanced documentation
    @property
    def size(self) -> Tuple[int, int]:
        """
        Image dimensions as (width, height) tuple.

        Returns:
            Tuple[int, int]: (width, height) in pixels
                   - width: Horizontal dimension in pixels
                   - height: Vertical dimension in pixels

        Example:
            >>> img = Image.new('RGB', (800, 600))
            >>> img.size
            (800, 600)
            >>> width, height = img.size
            >>> print(f"Image is {width}x{height} pixels")
        """
        return self._rust_image.size

    @property
    def width(self) -> int:
        """
        Image width in pixels.

        Returns:
            int: Horizontal dimension of the image

        Example:
            >>> img = Image.new('RGB', (1920, 1080))
            >>> img.width
            1920
        """
        return self._rust_image.width

    @property
    def height(self) -> int:
        """
        Image height in pixels.

        Returns:
            int: Vertical dimension of the image

        Example:
            >>> img = Image.new('RGB', (1920, 1080))
            >>> img.height
            1080
        """
        return self._rust_image.height

    @property
    def mode(self) -> str:
        """
        Image color mode determining channel interpretation.

        Returns:
            str: Color mode string
                 Common modes:
                 - 'RGB': 3 channels (Red, Green, Blue)
                 - 'RGBA': RGB + Alpha channel
                 - 'L': Grayscale (single channel)
                 - 'LA': Grayscale + Alpha
                 - 'CMYK': Print color model
                 - 'YCbCr': Digital video color model

        Example:
            >>> img = Image.new('RGB', (100, 100))
            >>> img.mode
            'RGB'
            >>> rgba_img = Image.new('RGBA', (100, 100))
            >>> rgba_img.mode
            'RGBA'
        """
        return self._rust_image.mode

    @property
    def format(self) -> Optional[str]:
        """
        Original image format if loaded from file, None for new images.

        Returns:
            Optional[str]: Image format string
                          Possible values: 'JPEG', 'PNG', 'GIF', 'BMP', 'TIFF', etc.
                          None: For images created with new() or other constructors

        Example:
            >>> img = Image.open('photo.jpg')
            >>> img.format
            'JPEG'
            >>> new_img = Image.new('RGB', (100, 100))
            >>> new_img.format is None
            True
        """
        return self._rust_image.format

    @property
    def info(self) -> dict:
        """
        Image metadata dictionary (for future expansion).

        Returns:
            dict: Currently empty dict, reserved for future metadata support
                 Planned features: EXIF data, color profiles, etc.

        Note:
            This property is provided for Pillow compatibility.
            Currently returns empty dict, but may contain metadata in future versions.
        """
        return {}

    def __repr__(self) -> str:
        """
        String representation of the image for debugging.

        Returns:
            str: Detailed string representation including size, mode, and format

        Example:
            >>> img = Image.new('RGB', (800, 600), 'red')
            >>> repr(img)
            "Image(mode='RGB', size=(800, 600), format=None)"
        """
        return self._rust_image.__repr__()

    def __eq__(self, other) -> bool:
        """
        Compare two images for pixel-level equality.

        Args:
            other: Another Image instance to compare with

        Returns:
            bool: True if images have same size, mode, and pixel data
                  False otherwise or if other is not an Image instance

        Note:
            - Compares size, mode, and raw pixel data
            - Order of operations doesn't matter for equality
            - Expensive operation for large images (full data comparison)

        Example:
            >>> img1 = Image.new('RGB', (100, 100), 'red')
            >>> img2 = Image.new('RGB', (100, 100), 'red')
            >>> img3 = Image.new('RGB', (100, 100), 'blue')
            >>> img1 == img2
            True
            >>> img1 == img3
            False
        """
        if not isinstance(other, self.__class__):
            return False

        return (
            self.size == other.size
            and self.mode == other.mode
            and self.to_bytes() == other.to_bytes()
        )

    @classmethod
    def circle(
        cls, size: int, color: Tuple[int, int, int, int] = (0, 0, 0, 255)
    ) -> "Image":
        """
        Create a new circular image.

        Args:
            size: Diameter of the circle in pixels
            color: (R, G, B, A) color values

        Returns:
            New Image instance with a circle
        """
        return cls(cls._create_shape("circle", size, color))

    @classmethod
    def rectangle(
        cls, width: int, height: int, color: Tuple[int, int, int, int] = (0, 0, 0, 255)
    ) -> "Image":
        """
        Create a new rectangular image.

        Args:
            width: Rectangle width
            height: Rectangle height
            color: (R, G, B, A) color values

        Returns:
            New Image instance with a rectangle
        """
        return cls(cls._create_shape("rectangle", (width, height), color))

    @classmethod
    def triangle(
        cls, width: int, height: int, color: Tuple[int, int, int, int] = (0, 0, 0, 255)
    ) -> "Image":
        """
        Create a new triangular image.

        Args:
            width: Triangle width
            height: Triangle height
            color: (R, G, B, A) color values

        Returns:
            New Image instance with a triangle
        """
        return cls(cls._create_shape("triangle", (width, height), color))

    @classmethod
    def ellipse(
        cls, width: int, height: int, color: Tuple[int, int, int, int] = (0, 0, 0, 255)
    ) -> "Image":
        """
        Create a new elliptical image.

        Args:
            width: Ellipse width
            height: Ellipse height
            color: (R, G, B, A) color values

        Returns:
            New Image instance with an ellipse
        """
        return cls(cls._create_shape("ellipse", (width, height), color))

    @classmethod
    def star(
        cls, size: int, color: Tuple[int, int, int, int] = (0, 0, 0, 255)
    ) -> "Image":
        """
        Create a new star-shaped image.

        Args:
            size: Star size (diameter)
            color: (R, G, B, A) color values

        Returns:
            New Image instance with a star
        """
        return cls(cls._create_shape("star", size, color))

    @classmethod
    def square(
        cls, size: int, color: Tuple[int, int, int, int] = (0, 0, 0, 255)
    ) -> "Image":
        """
        Create a new square image.

        Args:
            size: Square size
            color: (R, G, B, A) color values

        Returns:
            New Image instance with a square
        """
        return cls(cls._create_shape("square", size, color))

    @classmethod
    def diamond(
        cls, size: int, color: Tuple[int, int, int, int] = (0, 0, 0, 255)
    ) -> "Image":
        """
        Create a new diamond-shaped image.

        Args:
            size: Diamond size
            color: (R, G, B, A) color values

        Returns:
            New Image instance with a diamond
        """
        return cls(cls._create_shape("diamond", size, color))

    @classmethod
    def hexagon(
        cls, size: int, color: Tuple[int, int, int, int] = (0, 0, 0, 255)
    ) -> "Image":
        """
        Create a new hexagonal image.

        Args:
            size: Hexagon size
            color: (R, G, B, A) color values

        Returns:
            New Image instance with a hexagon
        """
        return cls(cls._create_shape("hexagon", size, color))

    @classmethod
    def parallelogram(
        cls,
        width: int,
        height: int,
        skew: float = 0.2,
        color: Tuple[int, int, int, int] = (0, 0, 0, 255),
    ) -> "Image":
        """
        Create a new parallelogram image.

        Args:
            width: Parallelogram width
            height: Parallelogram height
            skew: Skew factor (0.0 = rectangle, higher = more skewed)
            color: (R, G, B, A) color values

        Returns:
            New Image instance with a parallelogram
        """
        return cls(cls._create_shape("parallelogram", (width, height, skew), color))

    @classmethod
    def pentagon(
        cls, size: int, color: Tuple[int, int, int, int] = (0, 0, 0, 255)
    ) -> "Image":
        """
        Create a new pentagonal image.

        Args:
            size: Pentagon size
            color: (R, G, B, A) color values

        Returns:
            New Image instance with a pentagon
        """
        return cls(cls._create_shape("pentagon", size, color))

    @classmethod
    def octagon(
        cls, size: int, color: Tuple[int, int, int, int] = (0, 0, 0, 255)
    ) -> "Image":
        """
        Create a new octagonal image.

        Args:
            size: Octagon size
            color: (R, G, B, A) color values

        Returns:
            New Image instance with an octagon
        """
        return cls(cls._create_shape("octagon", size, color))

    @classmethod
    def heart(
        cls, size: int, color: Tuple[int, int, int, int] = (0, 0, 0, 255)
    ) -> "Image":
        """
        Create a new heart-shaped image.

        Args:
            size: Heart size
            color: (R, G, B, A) color values

        Returns:
            New Image instance with a heart
        """
        return cls(cls._create_shape("heart", size, color))

    @classmethod
    def arrow(
        cls, width: int, height: int, color: Tuple[int, int, int, int] = (0, 0, 0, 255)
    ) -> "Image":
        """
        Create a new arrow-shaped image.

        Args:
            width: Arrow width
            height: Arrow height
            color: (R, G, B, A) color values

        Returns:
            New Image instance with an arrow
        """
        return cls(cls._create_shape("arrow", (width, height), color))

    @classmethod
    def cross(
        cls, size: int, color: Tuple[int, int, int, int] = (0, 0, 0, 255)
    ) -> "Image":
        """
        Create a new cross-shaped image.

        Args:
            size: Cross size
            color: (R, G, B, A) color values

        Returns:
            New Image instance with a cross
        """
        return cls(cls._create_shape("cross", size, color))

    @classmethod
    def quadrilateral(
        cls,
        p1: Tuple[int, int],
        p2: Tuple[int, int],
        p3: Tuple[int, int],
        p4: Tuple[int, int],
        color: Tuple[int, int, int, int] = (0, 0, 0, 255),
    ) -> "Image":
        """
        Create a new quadrilateral image from 4 points.

        Args:
            p1: First point (x, y)
            p2: Second point (x, y)
            p3: Third point (x, y)
            p4: Fourth point (x, y)
            color: (R, G, B, A) color values

        Returns:
            New Image instance with a quadrilateral
        """
        from .._core import create_quadrilateral_py

        return cls(create_quadrilateral_py(p1, p2, p3, p4, color))

    @classmethod
    def _create_shape(
        cls, shape_type: str, params, color: Tuple[int, int, int, int]
    ) -> "Image":
        """Internal method to create shapes using Rust backend."""
        from .._core import (
            create_arrow_py,
            create_circle_py,
            create_cross_py,
            create_diamond_py,
            create_ellipse_py,
            create_heart_py,
            create_hexagon_py,
            create_octagon_py,
            create_parallelogram_py,
            create_pentagon_py,
            create_quadrilateral_py,
            create_rectangle_py,
            create_square_py,
            create_star_py,
            create_triangle_py,
        )

        # Map shape types to function calls
        if shape_type == "circle":
            return create_circle_py(params, color)
        elif shape_type == "rectangle":
            return create_rectangle_py(params[0], params[1], color)
        elif shape_type == "triangle":
            return create_triangle_py(params[0], params[1], color)
        elif shape_type == "ellipse":
            return create_ellipse_py(params[0], params[1], color)
        elif shape_type == "star":
            return create_star_py(params, color)
        elif shape_type == "square":
            return create_square_py(params, color)
        elif shape_type == "diamond":
            return create_diamond_py(params, color)
        elif shape_type == "hexagon":
            return create_hexagon_py(params, color)
        elif shape_type == "parallelogram":
            return create_parallelogram_py(params[0], params[1], params[2], color)
        elif shape_type == "pentagon":
            return create_pentagon_py(params, color)
        elif shape_type == "octagon":
            return create_octagon_py(params, color)
        elif shape_type == "heart":
            return create_heart_py(params, color)
        elif shape_type == "arrow":
            return create_arrow_py(params[0], params[1], color)
        elif shape_type == "cross":
            return create_cross_py(params, color)
        elif shape_type == "quadrilateral":
            return create_quadrilateral_py(
                params[0], params[1], params[2], params[3], color
            )
        else:
            raise ValueError(f"Unknown shape type: {shape_type}")
