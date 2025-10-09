"""
Pixel operations mixin - pixel manipulation and analysis
"""

from typing import Tuple


class PixelMixin:
    """Mixin for pixel-level operations"""

    def getpixel(self, x: int, y: int) -> Tuple[int, int, int, int]:
        """
        Get pixel value at coordinates.

        Args:
            x: X coordinate
            y: Y coordinate

        Returns:
            (R, G, B, A) pixel values
        """
        return self._rust_image.getpixel(x, y)

    def putpixel(
        self, x: int, y: int, color: Tuple[int, int, int, int]
    ) -> "Image":
        """
        Set pixel value at coordinates.

        Args:
            x: X coordinate
            y: Y coordinate
            color: (R, G, B, A) color values

        Returns:
            New Image instance with modified pixel
        """
        return self.__class__(self._rust_image.putpixel(x, y, color))

    def histogram(self) -> Tuple[list, list, list, list]:
        """
        Calculate image histogram.

        Returns:
            Tuple of (R, G, B, A) histograms (256 values each)
        """
        return self._rust_image.histogram()

    def dominant_color(self) -> Tuple[int, int, int, int]:
        """
        Get the dominant color in the image.

        Returns:
            (R, G, B, A) color values
        """
        return self._rust_image.dominant_color()

    def average_color(self) -> Tuple[int, int, int, int]:
        """
        Get the average color of the image.

        Returns:
            (R, G, B, A) color values
        """
        return self._rust_image.average_color()

    def replace_color(
        self,
        target_color: Tuple[int, int, int, int],
        replacement_color: Tuple[int, int, int, int],
        tolerance: int = 0,
    ) -> "Image":
        """
        Replace a color in the image.

        Args:
            target_color: Color to replace (R, G, B, A)
            replacement_color: New color (R, G, B, A)
            tolerance: Color matching tolerance (0-255)

        Returns:
            New Image instance with replaced colors
        """
        return self.__class__(
            self._rust_image.replace_color(
                target_color, replacement_color, tolerance
            )
        )

    def threshold(self, threshold_value: int) -> "Image":
        """
        Apply threshold to create binary image.

        Args:
            threshold_value: Threshold value (0-255)

        Returns:
            New binary Image instance
        """
        return self.__class__(self._rust_image.threshold(threshold_value))

    def posterize(self, levels: int) -> "Image":
        """
        Reduce the number of color levels.

        Args:
            levels: Number of levels per channel

        Returns:
            New posterized Image instance
        """
        return self.__class__(self._rust_image.posterize(levels))

