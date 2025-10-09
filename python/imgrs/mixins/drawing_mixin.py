"""
Drawing operations mixin - shapes and text
"""

from typing import Tuple


class DrawingMixin:
    """Mixin for drawing operations"""

    def draw_rectangle(
        self,
        x: int,
        y: int,
        width: int,
        height: int,
        color: Tuple[int, int, int, int],
    ) -> "Image":
        """
        Draw a rectangle on the image.

        Args:
            x: X coordinate of top-left corner
            y: Y coordinate of top-left corner
            width: Rectangle width
            height: Rectangle height
            color: (R, G, B, A) color values

        Returns:
            New Image instance with rectangle drawn
        """
        return self.__class__(
            self._rust_image.draw_rectangle(x, y, width, height, color)
        )

    def draw_circle(
        self,
        center_x: int,
        center_y: int,
        radius: int,
        color: Tuple[int, int, int, int],
    ) -> "Image":
        """
        Draw a circle on the image.

        Args:
            center_x: X coordinate of circle center
            center_y: Y coordinate of circle center
            radius: Circle radius
            color: (R, G, B, A) color values

        Returns:
            New Image instance with circle drawn
        """
        return self.__class__(
            self._rust_image.draw_circle(center_x, center_y, radius, color)
        )

    def draw_line(
        self,
        x0: int,
        y0: int,
        x1: int,
        y1: int,
        color: Tuple[int, int, int, int],
    ) -> "Image":
        """
        Draw a line on the image.

        Args:
            x0: Starting X coordinate
            y0: Starting Y coordinate
            x1: Ending X coordinate
            y1: Ending Y coordinate
            color: (R, G, B, A) color values

        Returns:
            New Image instance with line drawn
        """
        return self.__class__(
            self._rust_image.draw_line(x0, y0, x1, y1, color)
        )

    def draw_text(
        self,
        text: str,
        x: int,
        y: int,
        color: Tuple[int, int, int, int],
        scale: int = 1,
    ) -> "Image":
        """
        Draw text on the image.

        Args:
            text: Text to draw
            x: X coordinate
            y: Y coordinate
            color: (R, G, B, A) color values
            scale: Text scale factor

        Returns:
            New Image instance with text drawn
        """
        return self.__class__(
            self._rust_image.draw_text(text, x, y, color, scale)
        )

