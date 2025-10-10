"""CSS-like filter operations"""


class CSSFiltersMixin:
    """Mixin for CSS-like filters"""

    def sepia(self, amount: float = 1.0) -> "Image":
        """Apply sepia filter."""
        return self.__class__(self._rust_image.sepia(amount))

    def grayscale_filter(self, amount: float = 1.0) -> "Image":
        """Apply grayscale filter."""
        return self.__class__(self._rust_image.grayscale_filter(amount))

    def invert(self, amount: float = 1.0) -> "Image":
        """Apply invert filter."""
        return self.__class__(self._rust_image.invert(amount))

    def hue_rotate(self, degrees: float) -> "Image":
        """Apply hue rotation filter."""
        return self.__class__(self._rust_image.hue_rotate(degrees))

    def saturate(self, amount: float = 1.0) -> "Image":
        """Apply saturation filter."""
        return self.__class__(self._rust_image.saturate(amount))

