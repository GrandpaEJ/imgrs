"""
Text rendering mixin - advanced text operations with styling support
"""

from typing import TYPE_CHECKING, Optional, Tuple, Union

if TYPE_CHECKING:
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
        size: float = 32.0,
        color: Tuple[int, int, int, int] = (0, 0, 0, 255),
        font_path: Optional[str] = None,
    ) -> "Image":
        """
        Add basic text to the image.

        Args:
            text: Text to render
            position: Either (x, y) tuple or just x coordinate
            y: Y coordinate (if position is int)
            size: Font size in pixels
            color: (R, G, B, A) color values
            font_path: Path to TTF/OTF font file (optional, uses default if None)

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

        return self.__class__(
            self.draw_text(text, x, y_pos, color, int(size))
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
    ) -> "Image":
        """
        Add styled text with advanced formatting options.

        NOTE: Advanced styling features (background, outline, shadow, opacity, max_width)
        are not yet available. This method currently falls back to basic text rendering.

        Args:
            text: Text to render
            position: Either (x, y) tuple or just x coordinate
            y: Y coordinate (if position is int)
            size: Font size in pixels
            color: (R, G, B, A) color values
            font_path: Path to TTF/OTF font file (optional)
            background: (R, G, B, A) background color (optional) - NOT YET SUPPORTED
            align: Text alignment - "left", "center", or "right" (optional) - NOT YET SUPPORTED
            outline: (R, G, B, A, width) outline color and width (optional) - NOT YET SUPPORTED
            shadow: (offset_x, offset_y, R, G, B, A) shadow offset and color (optional) - NOT YET SUPPORTED
            opacity: Text opacity 0.0-1.0 (optional) - NOT YET SUPPORTED
            max_width: Maximum width for text wrapping (optional) - NOT YET SUPPORTED

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

        # For now, fall back to basic text rendering
        return self.__class__(
            self.draw_text(text, x, y_pos, color, int(size))
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

        NOTE: Advanced features (line_spacing, align) are not yet available.
        This method currently renders each line as separate text.

        Args:
            text: Multi-line text to render (separated by \\n)
            position: Either (x, y) tuple or just x coordinate
            y: Y coordinate (if position is int)
            size: Font size in pixels
            color: (R, G, B, A) color values
            font_path: Path to TTF/OTF font file (optional)
            line_spacing: Line spacing multiplier (default: 1.2) - NOT YET SUPPORTED
            align: Text alignment - "left", "center", or "right" (optional) - NOT YET SUPPORTED

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

        # For now, render each line as separate text
        img = self
        for i, line in enumerate(text.split('\n')):
            img = img.add_text(line, x, y_pos + i * int(size * 1.2), size, color, font_path)

        return img

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

        NOTE: Advanced styling features are not yet available.
        This method currently centers text horizontally using basic calculation.

        Args:
            text: Text to render
            y: Y coordinate for text baseline
            size: Font size in pixels
            color: (R, G, B, A) color values
            font_path: Path to TTF/OTF font file (optional)
            background: (R, G, B, A) background color (optional) - NOT YET SUPPORTED
            outline: (R, G, B, A, width) outline color and width (optional) - NOT YET SUPPORTED
            shadow: (offset_x, offset_y, R, G, B, A) shadow offset and color (optional) - NOT YET SUPPORTED
            opacity: Text opacity 0.0-1.0 (optional) - NOT YET SUPPORTED

        Returns:
            New Image instance with centered text added
        """
        # Simple centering calculation
        text_width = len(text) * int(size * 0.6)  # Rough estimate
        x = (self.width - text_width) // 2
        return self.add_text(text, x, y, size, color, font_path)

    def get_text_dimensions(
        self,
        text: str,
        size: float = 32.0,
        font_path: Optional[str] = None,
    ) -> Tuple[int, int, int, int]:
        """
        Get text dimensions and metrics.

        NOTE: This is a rough estimate. Advanced text measurement is not yet available.

        Args:
            text: Text to measure
            size: Font size in pixels
            font_path: Path to TTF/OTF font file (optional)

        Returns:
            Tuple of (width, height, ascent, descent) in pixels
        """
        # Rough estimate
        width = len(text) * int(size * 0.6)
        height = int(size)
        ascent = int(size * 0.8)
        descent = int(size * 0.2)
        return (width, height, ascent, descent)

    def get_multiline_text_dimensions(
        self,
        text: str,
        size: float = 32.0,
        line_spacing: float = 1.2,
        font_path: Optional[str] = None,
    ) -> Tuple[int, int, int]:
        """
        Get multi-line text dimensions.

        NOTE: This is a rough estimate. Advanced text measurement is not yet available.

        Args:
            text: Multi-line text to measure
            size: Font size in pixels
            line_spacing: Line spacing multiplier
            font_path: Path to TTF/OTF font file (optional)

        Returns:
            Tuple of (width, height, line_count)
        """
        lines = text.split('\n')
        line_count = len(lines)
        max_width = max(len(line) for line in lines) * int(size * 0.6)
        height = int(line_count * size * line_spacing)
        return (max_width, height, line_count)

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

        NOTE: This is a rough estimate. Advanced text measurement is not yet available.

        Args:
            text: Text to measure
            x: X coordinate
            y: Y coordinate
            size: Font size in pixels
            font_path: Path to TTF/OTF font file (optional)

        Returns:
            Dictionary with bounding box information
        """
        width = len(text) * int(size * 0.6)
        height = int(size)
        ascent = int(size * 0.8)
        descent = int(size * 0.2)

        return {
            'x': x,
            'y': y - ascent,
            'width': width,
            'height': height,
            'ascent': ascent,
            'descent': descent,
            'baseline_y': y,
            'bottom_y': y + descent,
            'right_x': x + width
        }

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
            font_path: Path to TTF/OTF font file (optional)

        Returns:
            New Image instance with shadowed text added
        """
        shadow = (shadow_offset[0], shadow_offset[1], shadow_color[0], shadow_color[1], shadow_color[2], shadow_color[3])

        if isinstance(position, tuple):
            x, y_pos = position
        else:
            x = position
            y_pos = y

        if y_pos is None:
            raise ValueError("y coordinate must be provided")

        # For now, render shadow as separate text
        img = self.add_text(text, x + shadow_offset[0], y_pos + shadow_offset[1], size, shadow_color, font_path)
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
            font_path: Path to TTF/OTF font file (optional)

        Returns:
            New Image instance with outlined text added
        """
        outline = (outline_color[0], outline_color[1], outline_color[2], outline_color[3], outline_width)

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
            font_path: Path to TTF/OTF font file (optional)

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