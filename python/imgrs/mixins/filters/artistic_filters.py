"""Artistic effect filter operations"""


class ArtisticFiltersMixin:
    """Mixin for artistic effects"""

    def vignette(self, strength: float, radius: float) -> "Image":
        """Apply vignette effect."""
        return self.__class__(self._rust_image.vignette(strength, radius))

    def halftone(self, dot_size: int) -> "Image":
        """Apply halftone effect."""
        return self.__class__(self._rust_image.halftone(dot_size))

    def pencil_sketch(self, detail: float) -> "Image":
        """Apply pencil sketch effect."""
        return self.__class__(self._rust_image.pencil_sketch(detail))

    def watercolor(self, iterations: int) -> "Image":
        """Apply watercolor effect."""
        return self.__class__(self._rust_image.watercolor(iterations))

    def glitch(self, intensity: float) -> "Image":
        """Apply glitch effect."""
        return self.__class__(self._rust_image.glitch(intensity))

