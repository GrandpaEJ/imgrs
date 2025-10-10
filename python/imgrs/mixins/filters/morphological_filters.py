"""Morphological filter operations"""


class MorphologicalFiltersMixin:
    """Mixin for morphological operations"""

    def dilate(self, radius: int) -> "Image":
        """Apply morphological dilation."""
        return self.__class__(self._rust_image.dilate(radius))

    def erode(self, radius: int) -> "Image":
        """Apply morphological erosion."""
        return self.__class__(self._rust_image.erode(radius))

    def morphological_opening(self, radius: int) -> "Image":
        """Apply morphological opening."""
        return self.__class__(self._rust_image.morphological_opening(radius))

    def morphological_closing(self, radius: int) -> "Image":
        """Apply morphological closing."""
        return self.__class__(self._rust_image.morphological_closing(radius))

    def morphological_gradient(self, radius: int) -> "Image":
        """Apply morphological gradient."""
        return self.__class__(self._rust_image.morphological_gradient(radius))

