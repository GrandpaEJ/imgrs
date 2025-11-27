"""
Advanced text rendering mixin for imgrs.
"""

from typing import TYPE_CHECKING, Dict, Optional, Tuple, Union

if TYPE_CHECKING:
    from ..image import Image


class TextMixin:
    """
    Advanced text rendering mixin providing comprehensive text capabilities.

    Features:
    - Flexible text positioning (tuple or separate x,y parameters)
    - Styled text with outlines, shadows, and backgrounds
    - Multi-line text rendering with customizable line spacing
    - Text measurement and bounding box calculations
    """

    def add_text(
        self,
        text: str,
        x_or_position: Union[int, Tuple[int, int]],
        y: Optional[int] = None,
        size: int = 40,
        color: Tuple[int, int, int, int] = (0, 0, 0, 255),
    ) -> "Image":
        """
        Add text to the image with flexible positioning.

        Args:
            text: Text to add
            x_or_position: X coordinate or (x, y) tuple
            y: Y coordinate (if x_or_position is int)
            size: Font size (approximated to scale)
            color: (R, G, B, A) color values

        Returns:
            New Image instance with text added
        """
        if isinstance(x_or_position, tuple):
            x, y_pos = x_or_position
        else:
            x = x_or_position
            if y is None:
                raise ValueError("y coordinate must be provided when x_or_position is int")
            y_pos = y

        # Approximate size to scale (8px base font * scale)
        scale = max(1, size // 8)
        return self.draw_text(text, x, y_pos, color, scale)

    def add_text_styled(
        self,
        text: str,
        position: Tuple[int, int],
        size: int = 40,
        color: Tuple[int, int, int, int] = (0, 0, 0, 255),
        outline: Optional[Tuple[int, int, int, int, float]] = None,
        shadow: Optional[Tuple[int, int, int, int, int, int]] = None,
        background: Optional[Tuple[int, int, int, int]] = None,
    ) -> "Image":
        """
        Add styled text with outline, shadow, and background support.

        Args:
            text: Text to add
            position: (x, y) tuple for text position
            size: Font size
            color: (R, G, B, A) color values
            outline: (R, G, B, A, width) for outline
            shadow: (offset_x, offset_y, R, G, B, A) for shadow
            background: (R, G, B, A) for background box

        Returns:
            New Image instance with styled text
        """
        x, y = position

        # Draw background if specified
        if background is not None:
            width, height = self.get_text_size(text, size)
            self._draw_filled_rect(x, y, width, height, background)

        # Draw shadow if specified
        if shadow is not None:
            offset_x, offset_y, sr, sg, sb, sa = shadow
            self.draw_text(text, x + offset_x, y + offset_y, (sr, sg, sb, sa), max(1, size // 8))

        # Draw outline if specified
        if outline is not None:
            or_, og, ob, oa, width = outline
            outline_color = (or_, og, ob, oa)
            for dx in range(-int(width), int(width) + 1):
                for dy in range(-int(width), int(width) + 1):
                    if abs(dx) + abs(dy) <= width:
                        self.draw_text(text, x + dx, y + dy, outline_color, max(1, size // 8))

        # Draw main text
        return self.draw_text(text, x, y, color, max(1, size // 8))

    def add_text_multiline(
        self,
        text: str,
        position: Tuple[int, int],
        size: int = 40,
        color: Tuple[int, int, int, int] = (0, 0, 0, 255),
        line_spacing: float = 1.2,
    ) -> "Image":
        """
        Add multi-line text with customizable line spacing.

        Args:
            text: Multi-line text (separated by \\n)
            position: (x, y) tuple for text position
            size: Font size
            color: (R, G, B, A) color values
            line_spacing: Spacing between lines (multiplier)

        Returns:
            New Image instance with multi-line text
        """
        x, y = position
        lines = text.split("\n")
        result = self

        for i, line in enumerate(lines):
            line_y = y + int(i * size * line_spacing)
            result = result.draw_text(line, x, line_y, color, max(1, size // 8))

        return result

    def get_text_size(self, text: str, size: int = 40) -> Tuple[int, int]:
        """
        Get the dimensions of text when rendered.

        Args:
            text: Text to measure
            size: Font size

        Returns:
            (width, height) tuple
        """
        # Approximate: 8px base width per char, 8px height, scaled
        scale = max(1, size // 8)
        width = len(text) * 8 * scale
        height = 8 * scale
        return (width, height)

    def get_text_box(self, text: str, x: int, y: int, size: int = 40) -> Dict[str, int]:
        """
        Get complete text bounding box information.

        Args:
            text: Text to measure
            x: X position
            y: Y position
            size: Font size

        Returns:
            Dictionary with box information
        """
        width, height = self.get_text_size(text, size)
        return {
            "x": x,
            "y": y,
            "width": width,
            "height": height,
            "baseline_y": y + height,
        }

    def _draw_filled_rect(
        self,
        x: int,
        y: int,
        width: int,
        height: int,
        color: Tuple[int, int, int, int],
    ) -> "Image":
        """
        Draw a filled rectangle (helper method).

        Args:
            x: X coordinate
            y: Y coordinate
            width: Rectangle width
            height: Rectangle height
            color: (R, G, B, A) color values

        Returns:
            New Image instance with rectangle drawn
        """
        result = self
        for py in range(y, y + height):
            for px in range(x, x + width):
                # Use putpixel if available, otherwise skip
                if hasattr(result, 'putpixel'):
                    result = result.putpixel(px, py, color)
        return result