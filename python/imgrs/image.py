"""
Simplified Image class using mixins for better maintainability
"""

from .mixins import (
    BlendingMixin,
    ColorMixin,
    CoreMixin,
    DrawingMixin,
    EffectsMixin,
    FilterMixin,
    MetadataMixin,
    PixelMixin,
    TextMixin,
    TransformMixin,
)


class Image(
    BlendingMixin,
    ColorMixin,
    CoreMixin,
    TransformMixin,
    FilterMixin,
    PixelMixin,
    DrawingMixin,
    EffectsMixin,
    MetadataMixin,
    TextMixin,
):
    """
    A high-performance image class backed by Rust.

    This class provides a Pillow-compatible API while leveraging Rust's
    performance and memory safety for all image operations.

    The class is organized using mixins for better code organization:
    - BlendingMixin: Advanced image compositing and blending modes
    - CoreMixin: I/O, constructors, properties
    - TransformMixin: Resize, crop, rotate, etc.
    - FilterMixin: All filter effects (blur, sharpen, edges, etc.) - 65+ filters
    - PixelMixin: Pixel-level operations and analysis
    - DrawingMixin: Drawing shapes and basic text rendering
    - TextMixin: Advanced text rendering with styling, alignment, and effects
    - EffectsMixin: Special effects (shadows, glow, drop shadows)
    - ColorMixin: Color operations and analysis
    - MetadataMixin: EXIF/metadata reading and GPS data
    """

    def __init__(self, rust_image=None):
        # Initialize the core mixin first to set up _rust_image properly
        CoreMixin.__init__(self, rust_image)
