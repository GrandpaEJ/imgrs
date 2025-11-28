"""
Blending operations mixin for advanced image compositing
"""

from typing import TYPE_CHECKING, Optional, Tuple

if TYPE_CHECKING:
    from .image import Image


class BlendingMixin:
    """Mixin for advanced blending and compositing operations"""

    def composite(self, other: "Image", mode: str = "over") -> "Image":
        """
        Composite this image with another using advanced blend modes.

        Args:
            other: Image to composite on top of this one
            mode: Blend mode to use. Supported modes:
                  - "clear", "source", "over", "in", "out", "atop"
                  - "dest", "dest_over", "dest_in", "dest_out", "dest_atop"
                  - "xor", "add", "saturate", "multiply", "screen"
                  - "overlay", "darken", "lighten", "color_dodge", "color_burn"
                  - "hard_light", "soft_light", "difference", "exclusion"
                  - "hsl_hue", "hsl_saturation", "hsl_color", "hsl_luminosity"

        Returns:
            New Image with the composite result

        Example:
            >>> base = Image.new('RGB', (100, 100), 'blue')
            >>> overlay = Image.new('RGBA', (50, 50), (255, 0, 0, 128))
            >>> result = base.composite(overlay, mode='multiply')
        """
        return self.__class__(self._rust_image.composite(other._rust_image, mode))

    def blend_over(self, other: "Image") -> "Image":
        """
        Blend with another image using 'over' mode (normal alpha blending).

        Args:
            other: Image to blend with

        Returns:
            New blended Image
        """
        return self.composite(other, "over")

    def blend_multiply(self, other: "Image") -> "Image":
        """
        Blend with another image using 'multiply' mode.

        Args:
            other: Image to blend with

        Returns:
            New blended Image
        """
        return self.composite(other, "multiply")

    def blend_screen(self, other: "Image") -> "Image":
        """
        Blend with another image using 'screen' mode.

        Args:
            other: Image to blend with

        Returns:
            New blended Image
        """
        return self.composite(other, "screen")

    def blend_overlay(self, other: "Image") -> "Image":
        """
        Blend with another image using 'overlay' mode.

        Args:
            other: Image to blend with

        Returns:
            New blended Image
        """
        return self.composite(other, "overlay")

    def blend_darken(self, other: "Image") -> "Image":
        """
        Blend with another image using 'darken' mode.

        Args:
            other: Image to blend with

        Returns:
            New blended Image
        """
        return self.composite(other, "darken")

    def blend_lighten(self, other: "Image") -> "Image":
        """
        Blend with another image using 'lighten' mode.

        Args:
            other: Image to blend with

        Returns:
            New blended Image
        """
        return self.composite(other, "lighten")

    def blend_difference(self, other: "Image") -> "Image":
        """
        Blend with another image using 'difference' mode.

        Args:
            other: Image to blend with

        Returns:
            New blended Image
        """
        return self.composite(other, "difference")

    def blend_exclusion(self, other: "Image") -> "Image":
        """
        Blend with another image using 'exclusion' mode.

        Args:
            other: Image to blend with

        Returns:
            New blended Image
        """
        return self.composite(other, "exclusion")

    def blend_hard_light(self, other: "Image") -> "Image":
        """
        Blend with another image using 'hard_light' mode.

        Args:
            other: Image to blend with

        Returns:
            New blended Image
        """
        return self.composite(other, "hard_light")

    def blend_soft_light(self, other: "Image") -> "Image":
        """
        Blend with another image using 'soft_light' mode.

        Args:
            other: Image to blend with

        Returns:
            New blended Image
        """
        return self.composite(other, "soft_light")

    def blend_color_dodge(self, other: "Image") -> "Image":
        """
        Blend with another image using 'color_dodge' mode.

        Args:
            other: Image to blend with

        Returns:
            New blended Image
        """
        return self.composite(other, "color_dodge")

    def blend_color_burn(self, other: "Image") -> "Image":
        """
        Blend with another image using 'color_burn' mode.

        Args:
            other: Image to blend with

        Returns:
            New blended Image
        """
        return self.composite(other, "color_burn")

    def blend_add(self, other: "Image") -> "Image":
        """
        Blend with another image using 'add' mode.

        Args:
            other: Image to blend with

        Returns:
            New blended Image
        """
        return self.composite(other, "add")

    def blend(
        self,
        mode: str,
        other: Optional["Image"] = None,
        mask: Optional["Image"] = None,
        position: Optional[Tuple[int, int]] = None,
    ) -> "Image":
        """
        Advanced blending with position and mask support.

        Args:
            mode: Blend mode to use (all modes from composite() supported)
            other: Image to blend on top (if None, returns copy of self)
            mask: Optional mask image for alpha control (grayscale or RGBA)
            position: Optional (x, y) position to place the blended image (default: (0, 0))

        Returns:
            New blended Image

        Example:
            >>> base = Image.new('RGB', (400, 300), 'blue')
            >>> overlay = Image.new('RGBA', (100, 100), (255, 0, 0, 128))
            >>> mask = Image.new('L', (100, 100), 200)  # Semi-transparent mask
            >>> result = base.blend('multiply', overlay, mask, (50, 50))
        """
        return self.__class__(
            self._rust_image.blend(
                mode,
                other._rust_image if other else None,
                mask._rust_image if mask else None,
                position,
            )
        )
