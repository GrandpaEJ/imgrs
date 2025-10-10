"""Sharpening filter operations"""


class SharpenFiltersMixin:
    """Mixin for sharpening operations"""

    def unsharp_mask(self, radius: float, amount: float, threshold: int) -> "Image":
        """Apply unsharp mask sharpening."""
        return self.__class__(
            self._rust_image.unsharp_mask(radius, amount, threshold)
        )

    def high_pass(self, radius: float) -> "Image":
        """Apply high-pass filter."""
        return self.__class__(self._rust_image.high_pass(radius))

    def edge_enhance(self, strength: float) -> "Image":
        """Apply edge enhancement."""
        return self.__class__(self._rust_image.edge_enhance(strength))

    def edge_enhance_more(self) -> "Image":
        """Apply strong edge enhancement."""
        return self.__class__(self._rust_image.edge_enhance_more())

