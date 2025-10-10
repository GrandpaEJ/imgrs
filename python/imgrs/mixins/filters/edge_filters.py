"""Edge detection filter operations"""


class EdgeFiltersMixin:
    """Mixin for edge detection operations"""

    def prewitt_edge_detect(self) -> "Image":
        """Apply Prewitt edge detection."""
        return self.__class__(self._rust_image.prewitt_edge_detect())

    def scharr_edge_detect(self) -> "Image":
        """Apply Scharr edge detection."""
        return self.__class__(self._rust_image.scharr_edge_detect())

    def roberts_cross_edge_detect(self) -> "Image":
        """Apply Roberts Cross edge detection."""
        return self.__class__(self._rust_image.roberts_cross_edge_detect())

    def laplacian_edge_detect(self) -> "Image":
        """Apply Laplacian edge detection."""
        return self.__class__(self._rust_image.laplacian_edge_detect())

    def laplacian_of_gaussian(self, sigma: float) -> "Image":
        """Apply Laplacian of Gaussian edge detection."""
        return self.__class__(self._rust_image.laplacian_of_gaussian(sigma))

    def canny_edge_detect(self, low_threshold: float, high_threshold: float) -> "Image":
        """Apply Canny edge detection."""
        return self.__class__(
            self._rust_image.canny_edge_detect(low_threshold, high_threshold)
        )

