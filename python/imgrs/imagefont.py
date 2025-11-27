"""
ImageFont module for imgrs - Font loading and management

Provides Pillow-compatible font loading functionality with support for:
- TTF (TrueType Font) files
- OTF (OpenType Font) files
- WOFF (Web Open Font Format) files
- Built-in fallback fonts
"""

from pathlib import Path
from typing import Optional, Tuple, Union


class Font:
    """
    Font object for text rendering.

    Compatible with Pillow's ImageFont interface.
    """

    def __init__(self, font_path: Optional[str] = None, size: int = 12):
        """
        Create a Font object.

        Args:
            font_path: Path to font file (TTF, OTF, WOFF) or None for default
            size: Font size in points
        """
        self.font_path = font_path
        self.size = size
        self._font_data = None

        if font_path:
            self._load_font(font_path)

    def _load_font(self, font_path: str) -> None:
        """Load font data from file."""
        path = Path(font_path)

        if not path.exists():
            raise FileNotFoundError(f"Font file not found: {font_path}")

        # Validate file extension
        if path.suffix.lower() not in [".ttf", ".otf", ".woff", ".woff2"]:
            raise ValueError(
                f"Unsupported font format: {path.suffix}. Supported: TTF, OTF, WOFF, WOFF2"
            )

        try:
            with open(path, "rb") as f:
                self._font_data = f.read()
        except Exception as e:
            raise RuntimeError(f"Failed to load font {font_path}: {e}")

    def get_font_path(self) -> Optional[str]:
        """Get the font file path."""
        return self.font_path

    def get_size(self) -> int:
        """Get the font size."""
        return self.size

    def get_font_data(self) -> Optional[bytes]:
        """Get the raw font data."""
        return self._font_data

    def __repr__(self) -> str:
        if self.font_path:
            return f"Font('{self.font_path}', size={self.size})"
        else:
            return f"Font(size={self.size})"


# Global font registry for caching loaded fonts
_font_cache = {}


def load(font_path: Union[str, Path], size: int = 12) -> Font:
    """
    Load a font from file.

    Args:
        font_path: Path to font file (TTF, OTF, WOFF, WOFF2)
        size: Font size in points

    Returns:
        Font object

    Raises:
        FileNotFoundError: If font file doesn't exist
        ValueError: If font format is unsupported
        RuntimeError: If font loading fails
    """
    font_path = str(font_path)

    # Create cache key
    cache_key = (font_path, size)

    # Check cache first
    if cache_key in _font_cache:
        return _font_cache[cache_key]

    # Load new font
    font = Font(font_path, size)

    # Cache the font
    _font_cache[cache_key] = font

    return font


def truetype(font_path: Union[str, Path], size: int = 12) -> Font:
    """
    Load a TrueType font.

    Alias for load() - maintains Pillow compatibility.

    Args:
        font_path: Path to TTF font file
        size: Font size in points

    Returns:
        Font object
    """
    return load(font_path, size)


def load_default(size: int = 12) -> Font:
    """
    Load the default fallback font.

    Returns:
        Font object using built-in font
    """
    cache_key = ("default", size)

    if cache_key in _font_cache:
        return _font_cache[cache_key]

    font = Font(None, size)
    _font_cache[cache_key] = font

    return font


# Common font loading functions for compatibility
def load_path(font_path: Union[str, Path], size: int = 12) -> Font:
    """Load font from path (alias for load)."""
    return load(font_path, size)


def get_font(font_path: Optional[Union[str, Path]] = None, size: int = 12) -> Font:
    """
    Get a font object.

    If font_path is None, returns default font.

    Args:
        font_path: Path to font file or None
        size: Font size in points

    Returns:
        Font object
    """
    if font_path is None:
        return load_default(size)
    else:
        return load(font_path, size)


# Font size utilities
def getsize(text: str, font: Font) -> Tuple[int, int]:
    """
    Get the size of text when rendered with the given font.

    This is a compatibility function that returns approximate dimensions.

    Args:
        text: Text to measure
        font: Font object

    Returns:
        Tuple of (width, height)
    """
    # Rough estimation - 0.6 width per character, height = font size
    width = len(text) * int(font.size * 0.6)
    height = font.size
    return (width, height)


def getbbox(text: str, font: Font) -> Tuple[int, int, int, int]:
    """
    Get the bounding box of text when rendered with the given font.

    This is a compatibility function that returns approximate bounding box.

    Args:
        text: Text to measure
        font: Font object

    Returns:
        Tuple of (left, top, right, bottom)
    """
    width, height = getsize(text, font)
    return (0, 0, width, height)


# Predefined font sizes for convenience
class FontSize:
    """Common font sizes."""

    SMALL = 10
    MEDIUM = 12
    LARGE = 16
    XLARGE = 20
    XXLARGE = 24


# Export public API
__all__ = [
    "Font",
    "load",
    "truetype",
    "load_default",
    "load_path",
    "get_font",
    "getsize",
    "getbbox",
    "FontSize",
]
