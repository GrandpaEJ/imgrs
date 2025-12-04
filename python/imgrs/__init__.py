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
    # Basic operations
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
    # Format conversion operations
    batch_convert,
    convert_format,
    convert_to_bytes,
    detect_format,
    get_format_capabilities,
    get_format_recommendation,
    get_supported_formats,
    is_supported_format,
    optimize_for_format,
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
    # Basic operations
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
    # Format conversion operations
    "batch_convert",
    "convert_format",
    "convert_to_bytes",
    "detect_format",
    "get_format_capabilities",
    "get_format_recommendation",
    "get_supported_formats",
    "is_supported_format",
    "optimize_for_format",
]
