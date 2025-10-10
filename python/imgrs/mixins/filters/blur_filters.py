"""Advanced blur filter operations"""


class BlurFiltersMixin:
    """Mixin for advanced blur operations"""

    def box_blur(self, radius: int) -> "Image":
        """Apply box blur filter."""
        return self.__class__(self._rust_image.box_blur(radius))

    def motion_blur(self, size: int, angle: float) -> "Image":
        """Apply motion blur filter."""
        return self.__class__(self._rust_image.motion_blur(size, angle))

    def median_blur(self, radius: int) -> "Image":
        """Apply median blur filter."""
        return self.__class__(self._rust_image.median_blur(radius))

    def bilateral_blur(
        self, radius: int, sigma_color: float, sigma_space: float
    ) -> "Image":
        """Apply bilateral blur filter."""
        return self.__class__(
            self._rust_image.bilateral_blur(radius, sigma_color, sigma_space)
        )

    def radial_blur(self, strength: float) -> "Image":
        """Apply radial blur effect."""
        return self.__class__(self._rust_image.radial_blur(strength))

    def zoom_blur(self, strength: float) -> "Image":
        """Apply zoom blur effect."""
        return self.__class__(self._rust_image.zoom_blur(strength))

