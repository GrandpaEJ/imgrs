"""
Simplified Image class using mixins for better maintainability
"""

from .mixins import (
    ColorMixin,
    CoreMixin,
    DrawingMixin,
    EffectsMixin,
    FilterMixin,
    MetadataMixin,
    PixelMixin,
    TransformMixin,
)


class Image(
    ColorMixin,
    CoreMixin,
    TransformMixin,
    FilterMixin,
    PixelMixin,
    DrawingMixin,
    EffectsMixin,
    MetadataMixin,
):
    """
    A high-performance image class backed by Rust.

    This class provides a Pillow-compatible API while leveraging Rust's
    performance and memory safety for all image operations.

    The class is organized using mixins for better code organization:
    - CoreMixin: I/O, constructors, properties
    - TransformMixin: Resize, crop, rotate, etc.
    - FilterMixin: All filter effects (blur, sharpen, edges, etc.) - 65+ filters
    - PixelMixin: Pixel-level operations and analysis
    - DrawingMixin: Drawing shapes and advanced text rendering
    - EffectsMixin: Special effects (shadows, glow, drop shadows)
    - ColorMixin: Color operations and analysis
    - MetadataMixin: EXIF/metadata reading and GPS data
    """

    pass  # All functionality is provided by mixins
