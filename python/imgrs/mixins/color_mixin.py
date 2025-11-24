"""
Enhanced color operations mixin - transparency, masking, and advanced color manipulation
"""

from typing import TYPE_CHECKING, List, Optional, Tuple

if TYPE_CHECKING:
    from .image import Image


class ColorMixin:
    """Mixin for advanced color operations with enhanced transparency and masking"""

    def set_alpha(self, alpha: float) -> "Image":
        """
        Set global alpha channel for the entire image.

        Args:
            alpha: Alpha value (0.0 = transparent, 1.0 = opaque)

        Returns:
            New Image instance with modified alpha
        """
        return self.__class__(self._rust_image.set_alpha(alpha))

    def get_alpha(self) -> float:
        """
        Get the average alpha channel value.

        Returns:
            Average alpha value (0.0-1.0)
        """
        return self._rust_image.get_alpha()

    def add_transparency(
        self, color: Tuple[int, int, int], tolerance: int = 0
    ) -> "Image":
        """
        Add transparency to specific colors.

        Args:
            color: Target color to make transparent (R, G, B)
            tolerance: Color matching tolerance (0-255)

        Returns:
            New Image with transparency added
        """
        return self.__class__(self._rust_image.add_transparency(color, tolerance))

    def remove_transparency(
        self, background_color: Optional[Tuple[int, int, int]] = None
    ) -> "Image":
        """
        Remove transparency by compositing on background.

        Args:
            background_color: Background color (R, G, B), defaults to white

        Returns:
            New opaque Image
        """
        return self.__class__(self._rust_image.remove_transparency(background_color))

    # Advanced Masking System
    def apply_mask(self, mask: "Image", invert: bool = False) -> "Image":
        """
        Apply a mask to the image using alpha channel.

        Args:
            mask: Mask image (grayscale or RGBA)
            invert: Invert the mask

        Returns:
            New masked Image
        """
        return self.__class__(self._rust_image.apply_mask(mask._rust_image, invert))

    def create_gradient_mask(
        self,
        direction: str = "vertical",
        start_opacity: float = 0.0,
        end_opacity: float = 1.0,
    ) -> "Image":
        """
        Create a gradient mask.

        Args:
            direction: "horizontal", "vertical", "radial", or "diagonal"
            start_opacity: Starting opacity (0.0-1.0)
            end_opacity: Ending opacity (0.0-1.0)

        Returns:
            New gradient mask Image
        """
        return self.__class__(
            self._rust_image.create_gradient_mask(direction, start_opacity, end_opacity)
        )

    def create_color_mask(
        self,
        target_color: Tuple[int, int, int],
        tolerance: int = 30,
        feather: int = 5,
    ) -> "Image":
        """
        Create a mask based on color similarity.

        Args:
            target_color: Target color to mask (R, G, B)
            tolerance: Color matching tolerance (0-255)
            feather: Feather amount for soft edges

        Returns:
            New color-based mask Image
        """
        return self.__class__(
            self._rust_image.create_color_mask(target_color, tolerance, feather)
        )

    def create_luminance_mask(self, invert: bool = False) -> "Image":
        """
        Create a mask based on image luminance.

        Args:
            invert: Invert the luminance mask

        Returns:
            New luminance mask Image
        """
        return self.__class__(self._rust_image.create_luminance_mask(invert))

    def combine_masks(
        self, masks: List["Image"], operation: str = "multiply"
    ) -> "Image":
        """
        Combine multiple masks using mathematical operations.

        Args:
            masks: List of mask images
            operation: "multiply", "add", "subtract", "overlay", "screen"

        Returns:
            New combined mask Image
        """
        rust_masks = [mask._rust_image for mask in masks]
        return self.__class__(self._rust_image.combine_masks(rust_masks, operation))

    # Enhanced Color Operations
    def extract_color(
        self, target_color: Tuple[int, int, int], tolerance: int = 30
    ) -> "Image":
        """
        Extract pixels matching a target color.

        Args:
            target_color: Target color to extract (R, G, B)
            tolerance: Color matching tolerance (0-255)

        Returns:
            New Image with matching pixels extracted
        """
        return self.__class__(self._rust_image.extract_color(target_color, tolerance))

    def color_quantize(self, levels: int = 16) -> "Image":
        """
        Quantize colors to reduce palette size.

        Args:
            levels: Number of color levels per channel

        Returns:
            New quantized Image
        """
        return self.__class__(self._rust_image.color_quantize(levels))

    def color_shift(self, shift_amount: float) -> "Image":
        """
        Shift all colors by a specified amount.

        Args:
            shift_amount: Color shift amount (-1.0 to 1.0)

        Returns:
            New color-shifted Image
        """
        return self.__class__(self._rust_image.color_shift(shift_amount))

    def selective_desaturate(
        self,
        target_color: Tuple[int, int, int],
        tolerance: int = 50,
        desaturate_factor: float = 0.0,
    ) -> "Image":
        """
        Selectively desaturate specific colors.

        Args:
            target_color: Target color for desaturation (R, G, B)
            tolerance: Color matching tolerance (0-255)
            desaturate_factor: Desaturation factor (0.0 = no change, 1.0 = full grayscale)

        Returns:
            New selectively desaturated Image
        """
        return self.__class__(
            self._rust_image.selective_desaturate(
                target_color, tolerance, desaturate_factor
            )
        )

    def color_match(self, reference_image: "Image", strength: float = 1.0) -> "Image":
        """
        Match colors to a reference image.

        Args:
            reference_image: Reference image for color matching
            strength: Matching strength (0.0-1.0)

        Returns:
            New color-matched Image
        """
        return self.__class__(
            self._rust_image.color_match(reference_image._rust_image, strength)
        )

    # Gradient and Pattern Operations
    def apply_gradient_overlay(
        self,
        color: Tuple[int, int, int, int],
        direction: str = "vertical",
        opacity: float = 1.0,
    ) -> "Image":
        """
        Apply a gradient color overlay.

        Args:
            color: Gradient color (R, G, B, A)
            direction: "horizontal", "vertical", "radial"
            opacity: Overlay opacity (0.0-1.0)

        Returns:
            New Image with gradient overlay
        """
        return self.__class__(
            self._rust_image.apply_gradient_overlay(color, direction, opacity)
        )

    def create_stripe_pattern(
        self,
        color: Tuple[int, int, int, int],
        width: int = 10,
        spacing: int = 5,
        angle: float = 0.0,
    ) -> "Image":
        """
        Create a stripe pattern overlay.

        Args:
            color: Stripe color (R, G, B, A)
            width: Stripe width in pixels
            spacing: Spacing between stripes
            angle: Rotation angle in degrees

        Returns:
            New Image with stripe pattern
        """
        return self.__class__(
            self._rust_image.create_stripe_pattern(color, width, spacing, angle)
        )

    def create_checker_pattern(
        self,
        color1: Tuple[int, int, int, int],
        color2: Tuple[int, int, int, int],
        size: int = 8,
    ) -> "Image":
        """
        Create a checkerboard pattern overlay.

        Args:
            color1: First checker color (R, G, B, A)
            color2: Second checker color (R, G, B, A)
            size: Checker size in pixels

        Returns:
            New Image with checker pattern
        """
        return self.__class__(
            self._rust_image.create_checker_pattern(color1, color2, size)
        )

    # Alpha Channel Operations
    def split_alpha(self) -> Tuple["Image", "Image"]:
        """
        Split image into RGB and alpha components.

        Returns:
            Tuple of (RGB Image, Alpha Image)
        """
        rust_rgb, rust_alpha = self._rust_image.split_alpha()
        return (self.__class__(rust_rgb), self.__class__(rust_alpha))

    def merge_alpha(self, alpha_image: "Image") -> "Image":
        """
        Merge alpha channel with image.

        Args:
            alpha_image: Alpha channel image (grayscale)

        Returns:
            New Image with merged alpha
        """
        return self.__class__(self._rust_image.merge_alpha(alpha_image._rust_image))

    def alpha_to_color(self, background_color: Tuple[int, int, int]) -> "Image":
        """
        Convert alpha channel to solid color.

        Args:
            background_color: Background color (R, G, B)

        Returns:
            New Image with alpha converted to color
        """
        return self.__class__(self._rust_image.alpha_to_color(background_color))

    # Advanced Blend Operations
    def blend_with(
        self,
        other: "Image",
        mode: str = "normal",
        opacity: float = 1.0,
    ) -> "Image":
        """
        Blend image with another using advanced blend modes.

        Args:
            other: Image to blend with
            mode: Blend mode ("normal", "multiply", "screen", "overlay", "soft_light",
                              "hard_light", "color_dodge", "color_burn", "darken",
                              "lighten", "difference", "exclusion")
            opacity: Blend opacity (0.0-1.0)

        Returns:
            New blended Image
        """
        return self.__class__(
            self._rust_image.blend_with(other._rust_image, mode, opacity)
        )

    def overlay_with(
        self,
        overlay: "Image",
        mode: str = "normal",
        opacity: float = 1.0,
        position: Optional[Tuple[int, int]] = None,
    ) -> "Image":
        """
        Overlay an image using advanced blending.

        Args:
            overlay: Image to overlay
            mode: Blend mode
            opacity: Overlay opacity
            position: Position to overlay at (x, y), defaults to center

        Returns:
            New overlaid Image
        """
        return self.__class__(
            self._rust_image.overlay_with(overlay._rust_image, mode, opacity, position)
        )

    # Color Analysis
    def get_color_palette(
        self, max_colors: int = 256
    ) -> List[Tuple[int, int, int, int]]:
        """
        Extract dominant colors from the image.

        Args:
            max_colors: Maximum number of colors to extract

        Returns:
            List of dominant colors with alpha
        """
        return self._rust_image.get_color_palette(max_colors)

    def analyze_color_distribution(self) -> dict:
        """
        Analyze color distribution in the image.

        Returns:
            Dictionary with color distribution statistics
        """
        return self._rust_image.analyze_color_distribution()

    def find_color_regions(
        self,
        target_color: Tuple[int, int, int],
        tolerance: int = 30,
    ) -> List[Tuple[int, int, int, int]]:
        """
        Find regions matching a target color.

        Args:
            target_color: Target color to find (R, G, B)
            tolerance: Color matching tolerance (0-255)

        Returns:
            List of bounding boxes (x, y, width, height) for matching regions
        """
        return self._rust_image.find_color_regions(target_color, tolerance)
