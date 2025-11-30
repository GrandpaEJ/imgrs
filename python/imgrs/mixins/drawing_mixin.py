"""
Drawing operations mixin - shapes and advanced text rendering
"""

from typing import TYPE_CHECKING, Tuple

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
            center_x: X coordinate of star center
            center_y: Y coordinate of star center
            outer_radius: Outer radius of the star
            inner_radius: Inner radius of the star
            points: Number of points on the star
            color: (R, G, B, A) color values

        Returns:
            New Image instance with star drawn
        """
        return self.__class__(
            self._rust_image.draw_star(
                center_x, center_y, outer_radius, inner_radius, points, color
            )
        )

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
            x1: X coordinate of first vertex
            y1: Y coordinate of first vertex
            x2: X coordinate of second vertex
            y2: Y coordinate of second vertex
            x3: X coordinate of third vertex
            y3: Y coordinate of third vertex
            color: (R, G, B, A) color values

        Returns:
            New Image instance with triangle drawn
        """
        return self.__class__(
            self._rust_image.draw_triangle(x1, y1, x2, y2, x3, y3, color)
        )

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
            center_x: X coordinate of ellipse center
            center_y: Y coordinate of ellipse center
            radius_x: Horizontal radius
            radius_y: Vertical radius
            color: (R, G, B, A) color values

        Returns:
            New Image instance with ellipse drawn
        """
        return self.__class__(
            self._rust_image.draw_ellipse(center_x, center_y, radius_x, radius_y, color)
        )

    def draw_polygon(
        self,
        points: list,
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
        Draw a regular polygon on the image.

        Args:
            center_x: X coordinate of center
            center_y: Y coordinate of center
            radius: Radius
            sides: Number of sides
            color: (R, G, B, A) color values
            rotation: Rotation angle

        Returns:
            New Image instance with regular polygon drawn
        """
        return self.__class__(
            self._rust_image.draw_regular_polygon(
                center_x, center_y, radius, sides, color, rotation
            )
        )

    def draw_text(
        self,
        text: str,
        x: int,
        y: int,
        color: Tuple[int, int, int, int],
        scale: int = 1,
        font_path: str = None,
        anchor: str = None,
    ) -> "Image":
        """
        Draw text on the image.

        Args:
            text: Text to draw
            x: X coordinate
            y: Y coordinate
            color: (R, G, B, A) color values
            scale: Text scale factor
            font_path: Path to font file
            anchor: Text anchor point

        Returns:
            New Image instance with text drawn
        """
        return self.__class__(
            self._rust_image.draw_text(text, x, y, color, scale, font_path, anchor)
        )
