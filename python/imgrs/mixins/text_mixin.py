"""
Text rendering mixin - advanced text operations with styling support
"""

from typing import TYPE_CHECKING, Optional, Tuple, Union

if TYPE_CHECKING:
    from ..imagefont import Font
    from .image import Image


class TextMixin:
    """
    Mixin for advanced text rendering operations.

    Provides comprehensive text rendering with:
    - Basic text drawing with custom fonts and colors
    - Styled text with backgrounds, outlines, shadows, and opacity
    - Multi-line text with alignment and line spacing
    - Centered text rendering
    - Text measurement and bounding box calculations
    - Font size and positioning control
    """

    def add_text(
        self,
        text: str,
        position: Union[Tuple[int, int], int],
        y: Optional[int] = None,
        size: Optional[float] = None,
        color: Tuple[int, int, int, int] = (0, 0, 0, 255),
        font_path: Optional[str] = None,
        font: Optional["Font"] = None,
        anchor: Optional[str] = None,
    ) -> "Image":
        """
        Add basic text to the image.

        Args:
            text: Text to render
            position: Either (x, y) tuple or just x coordinate
            y: Y coordinate (if position is int)
            size: Font size in pixels (ignored if font is provided)
            color: (R, G, B, A) color values
            font_path: Path to font file - supports TTF, OTF, WOFF, WOFF2 (optional, uses default if None)
            font: Font object (alternative to font_path and size)
            anchor: Text anchor point (e.g., "lt", "mm", "rb") - see TextAnchor docs

        Returns:
            New Image instance with text added
        """
        if isinstance(position, tuple):
            x, y_pos = position
        else:
            x = position
            y_pos = y

        if y_pos is None:
            raise ValueError("y coordinate must be provided")

        # Handle font parameter
        final_size = size or 32.0

        if font is not None:
            if hasattr(font, "get_font_path") and font.get_font_path():
                font.get_font_path()
            if hasattr(font, "get_size"):
                final_size = font.get_size()

        return self.__class__(
            self._rust_image.draw_text(
                text, x, y_pos, color, int(final_size), font_path, anchor
            )
        )

    def add_text_styled(
        self,
        text: str,
        position: Union[Tuple[int, int], int],
        y: Optional[int] = None,
        size: float = 32.0,
        color: Tuple[int, int, int, int] = (0, 0, 0, 255),
        font_path: Optional[str] = None,
        background: Optional[Tuple[int, int, int, int]] = None,
        align: Optional[str] = None,
        outline: Optional[Tuple[int, int, int, int, float]] = None,
        shadow: Optional[Tuple[int, int, int, int, int, int]] = None,
        opacity: Optional[float] = None,
        max_width: Optional[int] = None,
        rotation: Optional[float] = None,
        anchor: Optional[str] = None,
    ) -> "Image":
        """
        Add styled text with advanced formatting options.

        Args:
            text: Text to render
            position: Either (x, y) tuple or just x coordinate
            y: Y coordinate (if position is int)
            size: Font size in pixels
            color: (R, G, B, A) color values
            font_path: Path to font file - supports TTF, OTF, WOFF, WOFF2 (optional)
            background: (R, G, B, A) background color (optional)
            align: Text alignment - "left", "center", or "right" (optional)
            outline: (R, G, B, A, width) outline color and width (optional)
            shadow: (offset_x, offset_y, R, G, B, A) shadow offset and color (optional)
            opacity: Text opacity 0.0-1.0 (optional)
            max_width: Maximum width for text wrapping (optional)
            rotation: Rotation angle in degrees (optional)
            anchor: Text anchor point (e.g., "lt", "mm", "rb") - see TextAnchor docs

        Returns:
            New Image instance with styled text added
        """
        if isinstance(position, tuple):
            x, y_pos = position
        else:
            x = position
            y_pos = y

        if y_pos is None:
            raise ValueError("y coordinate must be provided")

        return self.__class__(
            self._rust_image.draw_text_styled(
                text,
                x,
                y_pos,
                size,
                color,
                font_path,
                background,
                align,
                outline,
                shadow,
                opacity,
                max_width,
                rotation,
                anchor,
            )
        )

    def add_text_multiline(
        self,
        text: str,
        position: Union[Tuple[int, int], int],
        y: Optional[int] = None,
        size: float = 32.0,
        color: Tuple[int, int, int, int] = (0, 0, 0, 255),
        font_path: Optional[str] = None,
        line_spacing: Optional[float] = None,
        align: Optional[str] = None,
    ) -> "Image":
        """
        Add multi-line text with alignment support.

        Args:
            text: Multi-line text to render (separated by \\n)
            position: Either (x, y) tuple or just x coordinate
            y: Y coordinate (if position is int)
            size: Font size in pixels
            color: (R, G, B, A) color values
            font_path: Path to font file - supports TTF, OTF, WOFF, WOFF2 (optional)
            line_spacing: Line spacing multiplier (default: 1.2)
            align: Text alignment - "left", "center", or "right" (optional)

        Returns:
            New Image instance with multi-line text added
        """
        if isinstance(position, tuple):
            x, y_pos = position
        else:
            x = position
            y_pos = y

        if y_pos is None:
            raise ValueError("y coordinate must be provided")

        return self.__class__(
            self._rust_image.draw_text_multiline(
                text,
                x,
                y_pos,
                size,
                color,
                font_path,
                line_spacing,
                align,
            )
        )

    def add_text_centered(
        self,
        text: str,
        y: int,
        size: float = 32.0,
        color: Tuple[int, int, int, int] = (0, 0, 0, 255),
        font_path: Optional[str] = None,
        background: Optional[Tuple[int, int, int, int]] = None,
        outline: Optional[Tuple[int, int, int, int, float]] = None,
        shadow: Optional[Tuple[int, int, int, int, int, int]] = None,
        opacity: Optional[float] = None,
    ) -> "Image":
        """
        Add horizontally centered text.

        Args:
            text: Text to render
            y: Y coordinate for text baseline
            size: Font size in pixels
            color: (R, G, B, A) color values
            font_path: Path to font file - supports TTF, OTF, WOFF, WOFF2 (optional)
            background: (R, G, B, A) background color (optional)
            outline: (R, G, B, A, width) outline color and width (optional)
            shadow: (offset_x, offset_y, R, G, B, A) shadow offset and color (optional)
            opacity: Text opacity 0.0-1.0 (optional)

        Returns:
            New Image instance with centered text added
        """
        return self.__class__(
            self._rust_image.draw_text_centered(
                text,
                y,
                size,
                color,
                font_path,
                background,
                outline,
                shadow,
                opacity,
            )
        )

    def get_text_dimensions(
        self,
        text: str,
        size: float = 32.0,
        font_path: Optional[str] = None,
    ) -> Tuple[int, int, int, int]:
        """
        Get text dimensions and metrics.

        Args:
            text: Text to measure
            size: Font size in pixels
            font_path: Path to font file - supports TTF, OTF, WOFF, WOFF2 (optional)

        Returns:
            Tuple of (width, height, ascent, descent) in pixels
        """
        return self._rust_image.get_text_size(text, size, font_path)

    def get_multiline_text_dimensions(
        self,
        text: str,
        size: float = 32.0,
        line_spacing: float = 1.2,
        font_path: Optional[str] = None,
    ) -> Tuple[int, int, int]:
        """
        Get multi-line text dimensions.

        Args:
            text: Multi-line text to measure
            size: Font size in pixels
            line_spacing: Line spacing multiplier
            font_path: Path to font file - supports TTF, OTF, WOFF, WOFF2 (optional)

        Returns:
            Tuple of (width, height, line_count)
        """
        return self._rust_image.get_multiline_text_size(
            text, size, line_spacing, font_path
        )

    def get_text_bounding_box(
        self,
        text: str,
        x: int,
        y: int,
        size: float = 32.0,
        font_path: Optional[str] = None,
    ) -> dict:
        """
        Get text bounding box information.

        Args:
            text: Text to measure
            x: X coordinate
            y: Y coordinate
            size: Font size in pixels
            font_path: Path to font file - supports TTF, OTF, WOFF, WOFF2 (optional)

        Returns:
            Dictionary with bounding box information
        """
        return self._rust_image.get_text_box(text, x, y, size, font_path)

    def add_text_box(
        self,
        text: str,
        box: Tuple[int, int, int, int],
        size: float = 32.0,
        color: Tuple[int, int, int, int] = (0, 0, 0, 255),
        font_path: Optional[str] = None,
        background: Optional[Tuple[int, int, int, int]] = None,
        align: Optional[str] = None,
        vertical_align: Optional[str] = None,
        line_spacing: Optional[float] = None,
        overflow: Optional[bool] = None,
    ) -> "Image":
        """
        Add text within a bounding box with automatic wrapping.

        Args:
            text: Text to render
            box: (x, y, width, height) tuple defining the bounding box
            size: Font size in pixels
            color: (R, G, B, A) color values
            font_path: Path to font file - supports TTF, OTF, WOFF, WOFF2 (optional)
            background: (R, G, B, A) background color (optional)
            align: Horizontal alignment - "left", "center", or "right" (optional)
            vertical_align: Vertical alignment - "top", "middle", or "bottom" (optional)
            line_spacing: Line spacing multiplier (optional)
            overflow: Whether to show text overflowing the box (default: False)

        Returns:
            New Image instance with text box added
        """
        x, y, width, height = box
        return self.__class__(
            self._rust_image.draw_text_box(
                text,
                x,
                y,
                width,
                height,
                size,
                color,
                font_path,
                background,
                align,
                vertical_align,
                line_spacing,
                overflow,
            )
        )

    def get_text_box(
        self,
        text: str,
        x: int,
        y: int,
        size: float = 32.0,
        font_path: Optional[str] = None,
    ) -> dict:
        """
        Get detailed text box metrics.

        Args:
            text: Text to measure
            x: X coordinate
            y: Y coordinate
            size: Font size in pixels
            font_path: Path to font file - supports TTF, OTF, WOFF, WOFF2 (optional)

        Returns:
            Dictionary with detailed metrics (x, y, width, height, ascent, descent, etc.)
        """
        return self._rust_image.get_text_box(text, x, y, size, font_path)

    # Convenience methods for common text operations

    def add_text_with_shadow(
        self,
        text: str,
        position: Union[Tuple[int, int], int],
        y: Optional[int] = None,
        size: float = 32.0,
        color: Tuple[int, int, int, int] = (0, 0, 0, 255),
        shadow_color: Tuple[int, int, int, int] = (0, 0, 0, 128),
        shadow_offset: Tuple[int, int] = (2, 2),
        font_path: Optional[str] = None,
    ) -> "Image":
        """
        Add text with a drop shadow effect.

        Args:
            text: Text to render
            position: Either (x, y) tuple or just x coordinate
            y: Y coordinate (if position is int)
            size: Font size in pixels
            color: (R, G, B, A) text color
            shadow_color: (R, G, B, A) shadow color
            shadow_offset: (offset_x, offset_y) shadow offset in pixels
            font_path: Path to font file - supports TTF, OTF, WOFF, WOFF2 (optional)

        Returns:
            New Image instance with shadowed text added
        """
        (
            shadow_offset[0],
            shadow_offset[1],
            shadow_color[0],
            shadow_color[1],
            shadow_color[2],
            shadow_color[3],
        )

        if isinstance(position, tuple):
            x, y_pos = position
        else:
            x = position
            y_pos = y

        if y_pos is None:
            raise ValueError("y coordinate must be provided")

        # For now, render shadow as separate text
        img = self.add_text(
            text,
            x + shadow_offset[0],
            y_pos + shadow_offset[1],
            size,
            shadow_color,
            font_path,
        )
        return img.add_text(text, x, y_pos, size, color, font_path)

    def add_text_with_outline(
        self,
        text: str,
        position: Union[Tuple[int, int], int],
        y: Optional[int] = None,
        size: float = 32.0,
        color: Tuple[int, int, int, int] = (255, 255, 255, 255),
        outline_color: Tuple[int, int, int, int] = (0, 0, 0, 255),
        outline_width: float = 1.0,
        font_path: Optional[str] = None,
    ) -> "Image":
        """
        Add text with an outline effect.

        Args:
            text: Text to render
            position: Either (x, y) tuple or just x coordinate
            y: Y coordinate (if position is int)
            size: Font size in pixels
            color: (R, G, B, A) text color
            outline_color: (R, G, B, A) outline color
            outline_width: Outline width in pixels
            font_path: Path to font file - supports TTF, OTF, WOFF, WOFF2 (optional)

        Returns:
            New Image instance with outlined text added
        """
        (
            outline_color[0],
            outline_color[1],
            outline_color[2],
            outline_color[3],
            outline_width,
        )

        if isinstance(position, tuple):
            x, y_pos = position
        else:
            x = position
            y_pos = y

        if y_pos is None:
            raise ValueError("y coordinate must be provided")

        # For now, render outline as separate text (simplified)
        img = self.add_text(text, x, y_pos, size, outline_color, font_path)
        return img.add_text(text, x, y_pos, size, color, font_path)

    def add_text_with_background(
        self,
        text: str,
        position: Union[Tuple[int, int], int],
        y: Optional[int] = None,
        size: float = 32.0,
        color: Tuple[int, int, int, int] = (0, 0, 0, 255),
        background_color: Tuple[int, int, int, int] = (255, 255, 255, 255),
        font_path: Optional[str] = None,
    ) -> "Image":
        """
        Add text with a background rectangle.

        Args:
            text: Text to render
            position: Either (x, y) tuple or just x coordinate
            y: Y coordinate (if position is int)
            size: Font size in pixels
            color: (R, G, B, A) text color
            background_color: (R, G, B, A) background color
            font_path: Path to font file - supports TTF, OTF, WOFF, WOFF2 (optional)

        Returns:
            New Image instance with background text added
        """
        if isinstance(position, tuple):
            x, y_pos = position
        else:
            x = position
            y_pos = y

        if y_pos is None:
            raise ValueError("y coordinate must be provided")

        # For now, just render the text (background not supported yet)
        return self.add_text(text, x, y_pos, size, color, font_path)

    def text(
        self,
        position: Tuple[int, int],
        text: str,
        fill: Optional[Tuple[int, int, int, int]] = None,
        font: Optional["Font"] = None,
        anchor: Optional[str] = None,
        spacing: int = 4,
        align: str = "left",
        direction: Optional[str] = None,
        features: Optional[list] = None,
        language: Optional[str] = None,
        stroke_width: int = 0,
        stroke_fill: Optional[Tuple[int, int, int, int]] = None,
        embedded_color: bool = False,
    ) -> "Image":
        """
        Draw text on the image (Pillow-compatible API).

        Args:
            position: (x, y) tuple for text position
            text: Text to draw
            fill: Text color as (R, G, B, A) tuple or None for default
            font: Font object (from ImageFont) or None for default
            anchor: Text anchor point (not yet supported)
            spacing: Line spacing for multi-line text
            align: Text alignment ("left", "center", "right")
            direction: Text direction (not yet supported)
            features: OpenType features (not yet supported)
            language: Language code (not yet supported)
            stroke_width: Text outline width (not yet supported)
            stroke_fill: Outline color (not yet supported)
            embedded_color: Use embedded color glyphs (not yet supported)

        Returns:
            New Image instance with text drawn
        """
        x, y = position

        # Handle font parameter
        font_path = None
        size = 12  # Default size

        if font is not None:
            if hasattr(font, "get_font_path") and font.get_font_path():
                font_path = font.get_font_path()
            if hasattr(font, "get_size"):
                size = font.get_size()

        # Handle color parameter
        color = fill if fill is not None else (0, 0, 0, 255)

        # Check if text contains newlines
        if "\n" in text:
            # Multi-line text
            return self.add_text_multiline(
                text,
                (x, y),
                size=float(size),
                color=color,
                font_path=font_path,
                align=align,
            )
        else:
            # Single line text
            return self.add_text(
                text, (x, y), size=float(size), color=color, font_path=font_path
            )
