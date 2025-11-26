"""
Drawing operations mixin - shapes and advanced text rendering
"""

from typing import TYPE_CHECKING, List, Optional, Tuple, Union

if TYPE_CHECKING:
    from .image import Image


class DrawingMixin:
    """
    Mixin for drawing operations including shapes and advanced text rendering.

    Provides comprehensive drawing capabilities:
    - Basic shapes: rectangles, circles, lines, triangles, polygons, stars, ellipses
    - Advanced text: add_text, add_text_styled, add_text_multiline, get_text_size, get_text_box
    - Text positioning: flexible coordinate system with tuple or separate x,y parameters
    """

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
        return self.__class__(self._rust_image.draw_line(x0, y0, x1, y1, color))

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
        return self.__class__(self._rust_image.draw_text(text, x, y, color, scale))

    def draw_triangle(
        self,
        x1: int,
        y1: int,
        x2: int,
        y2: int,
        x3: int,
        y3: int,
        color: Tuple[int, int, int, int],
    ) -> "Image":
        """
        Draw a triangle on the image.

        Args:
            x1, y1: First vertex
            x2, y2: Second vertex
            x3, y3: Third vertex
            color: (R, G, B, A) color values

        Returns:
            New Image instance with triangle drawn
        """
        return self.__class__(
            self._rust_image.draw_triangle(x1, y1, x2, y2, x3, y3, color)
        )

    def draw_star(
        self,
        center_x: int,
        center_y: int,
        outer_radius: int,
        inner_radius: int,
        points: int,
        color: Tuple[int, int, int, int],
    ) -> "Image":
        """
        Draw a star on the image.

        Args:
            center_x: X coordinate of center
            center_y: Y coordinate of center
            outer_radius: Radius of outer points
            inner_radius: Radius of inner points
            points: Number of points
            color: (R, G, B, A) color values

        Returns:
            New Image instance with star drawn
        """
        return self.__class__(
            self._rust_image.draw_star(
                center_x, center_y, outer_radius, inner_radius, points, color
            )
        )

    def draw_polygon(
        self,
        points: List[Tuple[int, int]],
        color: Tuple[int, int, int, int],
    ) -> "Image":
        """
        Draw a polygon on the image.

        Args:
            points: List of (x, y) tuples
            color: (R, G, B, A) color values

        Returns:
            New Image instance with polygon drawn
        """
        return self.__class__(self._rust_image.draw_polygon(points, color))

    def draw_ellipse(
        self,
        center_x: int,
        center_y: int,
        radius_x: int,
        radius_y: int,
        color: Tuple[int, int, int, int],
    ) -> "Image":
        """
        Draw an ellipse on the image.

        Args:
            center_x: X coordinate of center
            center_y: Y coordinate of center
            radius_x: Horizontal radius
            radius_y: Vertical radius
            color: (R, G, B, A) color values

        Returns:
            New Image instance with ellipse drawn
        """
        return self.__class__(
            self._rust_image.draw_ellipse(center_x, center_y, radius_x, radius_y, color)
        )

    def draw_regular_polygon(
        self,
        center_x: int,
        center_y: int,
        radius: int,
        sides: int,
        color: Tuple[int, int, int, int],
        rotation: float = 0.0,
    ) -> "Image":
        """
        Draw a regular polygon (triangle, square, pentagon, etc.).

        Args:
            center_x: X coordinate of center
            center_y: Y coordinate of center
            radius: Radius (distance from center to vertices)
            sides: Number of sides (3+)
            color: (R, G, B, A) color values
            rotation: Rotation angle in degrees

        Returns:
            New Image instance with regular polygon drawn
        """
        return self.__class__(
            self._rust_image.draw_regular_polygon(
                center_x, center_y, radius, sides, color, rotation
            )
        )

    def add_text(
        self,
        text: str,
        x_or_position: Union[int, Tuple[int, int]],
        y: Optional[int] = None,
        size: int = 40,
        color: Tuple[int, int, int, int] = (0, 0, 0, 255),
    ) -> "Image":
        """
        Add text to the image (convenience method).

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
            x, y = x_or_position
        else:
            x = x_or_position
            if y is None:
                raise ValueError("y coordinate must be provided")

        # Approximate size to scale (8px base font * scale)
        scale = max(1, size // 8)
        return self.draw_text(text, x, y, color, scale)

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
        Add styled text to the image.

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
        # For now, just draw basic text (advanced styling not implemented in Rust yet)
        x, y = position
        return self.add_text(text, x, y, size, color)

    def add_text_multiline(
        self,
        text: str,
        position: Tuple[int, int],
        size: int = 40,
        color: Tuple[int, int, int, int] = (0, 0, 0, 255),
        line_spacing: float = 1.2,
    ) -> "Image":
        """
        Add multi-line text to the image.

        Args:
            text: Multi-line text (separated by \n)
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
            result = result.add_text(line, x, line_y, size, color)

        return result

    @classmethod
    def get_text_size(cls, text: str, size: int = 40) -> Tuple[int, int]:
        """
        Get the size of text when rendered.

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

    @classmethod
    def get_text_box(cls, text: str, x: int, y: int, size: int = 40) -> dict:
        """
        Get text bounding box information.

        Args:
            text: Text to measure
            x: X position
            y: Y position
            size: Font size

        Returns:
            Dictionary with box information
        """
        width, height = cls.get_text_size(text, size)
        return {
            "x": x,
            "y": y,
            "width": width,
            "height": height,
            "baseline_y": y + height,
        }
