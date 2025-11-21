"""Color effect filter operations"""

from typing import TYPE_CHECKING, Tuple

if TYPE_CHECKING:
    from ..image import Image


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

    def chroma_key(
        self,
        key_color: Tuple[int, int, int],
        tolerance: float = 0.3,
        feather: float = 0.1,
    ) -> "Image":
        """Apply chroma key effect (green screen removal)."""
        return self.__class__(
            self._rust_image.chroma_key(key_color, tolerance, feather)
        )
