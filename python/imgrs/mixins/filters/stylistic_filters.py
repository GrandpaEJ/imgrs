"""Stylistic effect filter operations"""


class StylisticFiltersMixin:
    """Mixin for stylistic effects"""

    def oil_painting(self, radius: int, intensity: int) -> "Image":
        """Apply oil painting effect."""
        return self.__class__(self._rust_image.oil_painting(radius, intensity))

    def pixelate(self, pixel_size: int) -> "Image":
        """Apply pixelate effect."""
        return self.__class__(self._rust_image.pixelate(pixel_size))

    def mosaic(self, tile_size: int) -> "Image":
        """Apply mosaic effect."""
        return self.__class__(self._rust_image.mosaic(tile_size))

    def cartoon(self, num_levels: int, edge_threshold: float) -> "Image":
        """Apply cartoon effect."""
        return self.__class__(self._rust_image.cartoon(num_levels, edge_threshold))

    def sketch(self, detail_level: float) -> "Image":
        """Apply sketch effect."""
        return self.__class__(self._rust_image.sketch(detail_level))

    def solarize(self, threshold: int) -> "Image":
        """Apply solarize effect."""
        return self.__class__(self._rust_image.solarize(threshold))

