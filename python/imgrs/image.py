"""
Simplified Image class using mixins for better maintainability
"""

from .mixins import (
    CoreMixin,
    TransformMixin,
    FilterMixin,
    PixelMixin,
    DrawingMixin,
    EffectsMixin,
    EmojiMixin,
    MetadataMixin,
    TextMixin,
)


class Image(
    CoreMixin,
    TransformMixin,
    FilterMixin,
    PixelMixin,
    DrawingMixin,
    EffectsMixin,
    EmojiMixin,
    MetadataMixin,
    TextMixin,
):
    """
    A high-performance image class backed by Rust.

    This class provides a Pillow-compatible API while leveraging Rust's
    performance and memory safety for all image operations.

    The class is organized using mixins for better code organization:
    - CoreMixin: I/O, constructors, properties
    - TransformMixin: Resize, crop, rotate, etc.
    - FilterMixin: All filter effects (blur, sharpen, edges, etc.)
    - PixelMixin: Pixel-level operations
    - DrawingMixin: Drawing shapes and text
    - EffectsMixin: Special effects (shadows, glow)
    - EmojiMixin: Emoji overlays
    - MetadataMixin: EXIF/metadata reading
    - TextMixin: Rich text rendering with full styling
    """

    pass  # All functionality is provided by mixins

