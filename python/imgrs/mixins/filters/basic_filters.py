"""Basic filter operations"""


class BasicFiltersMixin:
    """Mixin for basic filter operations"""

    def blur(self, radius: float) -> "Image":
        """Apply Gaussian blur to the image."""
        return self.__class__(self._rust_image.blur(radius))

    def sharpen(self, strength: float = 1.0) -> "Image":
        """Apply sharpening filter to the image."""
        return self.__class__(self._rust_image.sharpen(strength))

    def edge_detect(self) -> "Image":
        """Apply edge detection filter (Sobel operator)."""
        return self.__class__(self._rust_image.edge_detect())

    def emboss(self) -> "Image":
        """Apply emboss filter to the image."""
        return self.__class__(self._rust_image.emboss())

    def brightness(self, adjustment: int) -> "Image":
        """Adjust image brightness."""
        return self.__class__(self._rust_image.brightness(adjustment))

    def contrast(self, factor: float) -> "Image":
        """Adjust image contrast."""
        return self.__class__(self._rust_image.contrast(factor))

