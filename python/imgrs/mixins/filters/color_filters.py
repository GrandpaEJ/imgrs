"""Color effect filter operations"""

from typing import Tuple


class ColorFiltersMixin:
    """Mixin for color effects"""

    def duotone(
        self, shadow: Tuple[int, int, int], highlight: Tuple[int, int, int]
    ) -> "Image":
        """Apply duotone effect."""
        return self.__class__(self._rust_image.duotone(shadow, highlight))

    def color_splash(self, target_hue: float, tolerance: float) -> "Image":
        """Apply color splash effect."""
        return self.__class__(self._rust_image.color_splash(target_hue, tolerance))

    def chromatic_aberration(self, strength: float) -> "Image":
        """Apply chromatic aberration effect."""
        return self.__class__(self._rust_image.chromatic_aberration(strength))

