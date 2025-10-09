"""
Effects mixin - shadows, glows, and special effects
"""

from typing import Tuple


class EffectsMixin:
    """Mixin for special effects (shadows, glows, etc.)"""

    def drop_shadow(
        self,
        offset_x: int,
        offset_y: int,
        blur_radius: int,
        shadow_color: Tuple[int, int, int, int],
    ) -> "Image":
        """
        Add a drop shadow effect to the image.

        Args:
            offset_x: Shadow horizontal offset
            offset_y: Shadow vertical offset
            blur_radius: Shadow blur radius
            shadow_color: (R, G, B, A) shadow color

        Returns:
            New Image instance with drop shadow
        """
        return self.__class__(
            self._rust_image.drop_shadow(
                offset_x, offset_y, blur_radius, shadow_color
            )
        )

    def inner_shadow(
        self,
        offset_x: int,
        offset_y: int,
        blur_radius: int,
        shadow_color: Tuple[int, int, int, int],
    ) -> "Image":
        """
        Add an inner shadow effect to the image.

        Args:
            offset_x: Shadow horizontal offset
            offset_y: Shadow vertical offset
            blur_radius: Shadow blur radius
            shadow_color: (R, G, B, A) shadow color

        Returns:
            New Image instance with inner shadow
        """
        return self.__class__(
            self._rust_image.inner_shadow(
                offset_x, offset_y, blur_radius, shadow_color
            )
        )

    def glow(
        self,
        blur_radius: int,
        glow_color: Tuple[int, int, int, int],
        intensity: float = 1.0,
    ) -> "Image":
        """
        Add a glow effect to the image.

        Args:
            blur_radius: Glow blur radius
            glow_color: (R, G, B, A) glow color
            intensity: Glow intensity (0.0 to 1.0+)

        Returns:
            New Image instance with glow effect
        """
        return self.__class__(
            self._rust_image.glow(blur_radius, glow_color, intensity)
        )

