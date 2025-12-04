"""
Imgrs - A high-performance, memory-safe image processing library

Provides the high-level API while addressing
performance and memory-safety issues through a Rust backend.
"""

from . import imagefont as ImageFont
from .enums import (
    BlendMode,
    ColorFormat,
    ImageFormat,
    ImageMode,
    MaskType,
    Resampling,
    Transpose,
)
from .image import Image
from .mixins.color_mixin import ColorMixin
from .operations import (
    blur,
    brightness,
    chroma_key,
    contrast,
    convert,
    crop,
    edge_detect,
    emboss,
    fromarray,
    new,
    open,
    paste,
    resize,
    rotate,
    save,
    sharpen,
    split,
)

__version__ = "0.3.8"
__author__ = "Grandpa EJ"

__all__ = [
    "BlendMode",
    "ColorFormat",
    "ColorMixin",
    "Image",
    "ImageFont",
    "ImageMode",
    "ImageFormat",
    "MaskType",
    "Resampling",
    "Transpose",
    "open",
    "new",
    "save",
    "resize",
    "crop",
    "rotate",
    "convert",
    "fromarray",
    "split",
    "paste",
    # Filters
    "blur",
    "chroma_key",
    "sharpen",
    "edge_detect",
    "emboss",
    "brightness",
    "contrast",
]
