"""
Filter operations mixin - all image filters organized by category
"""

from typing import Tuple


class FilterMixin:
    """Mixin for all filter operations"""

    # ========================================================================
    # BASIC FILTERS
    # ========================================================================

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

    # ========================================================================
    # ADVANCED BLUR EFFECTS
    # ========================================================================

    def box_blur(self, radius: int) -> "Image":
        """Apply box blur filter."""
        return self.__class__(self._rust_image.box_blur(radius))

    def motion_blur(self, size: int, angle: float) -> "Image":
        """Apply motion blur filter."""
        return self.__class__(self._rust_image.motion_blur(size, angle))

    def median_blur(self, radius: int) -> "Image":
        """Apply median blur filter."""
        return self.__class__(self._rust_image.median_blur(radius))

    def bilateral_blur(
        self, radius: int, sigma_color: float, sigma_space: float
    ) -> "Image":
        """Apply bilateral blur filter."""
        return self.__class__(
            self._rust_image.bilateral_blur(radius, sigma_color, sigma_space)
        )

    def radial_blur(self, strength: float) -> "Image":
        """Apply radial blur effect."""
        return self.__class__(self._rust_image.radial_blur(strength))

    def zoom_blur(self, strength: float) -> "Image":
        """Apply zoom blur effect."""
        return self.__class__(self._rust_image.zoom_blur(strength))

    # ========================================================================
    # ADVANCED EDGE DETECTION
    # ========================================================================

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

    # ========================================================================
    # ADVANCED SHARPENING
    # ========================================================================

    def unsharp_mask(self, radius: float, amount: float, threshold: int) -> "Image":
        """Apply unsharp mask sharpening."""
        return self.__class__(
            self._rust_image.unsharp_mask(radius, amount, threshold)
        )

    def high_pass(self, radius: float) -> "Image":
        """Apply high-pass filter."""
        return self.__class__(self._rust_image.high_pass(radius))

    def edge_enhance(self, strength: float) -> "Image":
        """Apply edge enhancement."""
        return self.__class__(self._rust_image.edge_enhance(strength))

    def edge_enhance_more(self) -> "Image":
        """Apply strong edge enhancement."""
        return self.__class__(self._rust_image.edge_enhance_more())

    # ========================================================================
    # STYLISTIC EFFECTS
    # ========================================================================

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

    # ========================================================================
    # NOISE EFFECTS
    # ========================================================================

    def add_gaussian_noise(self, mean: float, stddev: float) -> "Image":
        """Add Gaussian noise to the image."""
        return self.__class__(self._rust_image.add_gaussian_noise(mean, stddev))

    def add_salt_pepper_noise(self, amount: float) -> "Image":
        """Add salt & pepper noise to the image."""
        return self.__class__(self._rust_image.add_salt_pepper_noise(amount))

    def denoise(self, radius: int) -> "Image":
        """Apply denoising filter."""
        return self.__class__(self._rust_image.denoise(radius))

    # ========================================================================
    # MORPHOLOGICAL OPERATIONS
    # ========================================================================

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

    # ========================================================================
    # ARTISTIC EFFECTS
    # ========================================================================

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

    # ========================================================================
    # COLOR EFFECTS
    # ========================================================================

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

    # ========================================================================
    # CSS-LIKE FILTERS
    # ========================================================================

    def sepia(self, amount: float = 1.0) -> "Image":
        """Apply sepia filter."""
        return self.__class__(self._rust_image.sepia(amount))

    def grayscale_filter(self, amount: float = 1.0) -> "Image":
        """Apply grayscale filter."""
        return self.__class__(self._rust_image.grayscale_filter(amount))

    def invert(self, amount: float = 1.0) -> "Image":
        """Apply invert filter."""
        return self.__class__(self._rust_image.invert(amount))

    def hue_rotate(self, degrees: float) -> "Image":
        """Apply hue rotation filter."""
        return self.__class__(self._rust_image.hue_rotate(degrees))

    def saturate(self, amount: float = 1.0) -> "Image":
        """Apply saturation filter."""
        return self.__class__(self._rust_image.saturate(amount))

    # ========================================================================
    # AUTO-ENHANCEMENT FEATURES
    # ========================================================================

    def histogram_equalization(self) -> "Image":
        """
        Apply histogram equalization to enhance contrast automatically.
        
        Redistributes pixel intensities to use the full dynamic range,
        resulting in enhanced contrast and detail visibility.
        
        Returns:
            New Image with equalized histogram
        """
        return self.__class__(self._rust_image.histogram_equalization())

    def auto_contrast(self) -> "Image":
        """
        Automatically adjust contrast to optimal levels.
        
        Stretches the color range to use the full 0-255 range for each channel,
        maximizing contrast without manual adjustment.
        
        Returns:
            New Image with optimized contrast
        """
        return self.__class__(self._rust_image.auto_contrast())

    def auto_brightness(self) -> "Image":
        """
        Automatically adjust brightness to optimal level.
        
        Analyzes the image and adjusts brightness to achieve a balanced
        mid-range brightness level.
        
        Returns:
            New Image with optimized brightness
        """
        return self.__class__(self._rust_image.auto_brightness())

    def auto_enhance(self) -> "Image":
        """
        Automatically enhance image (contrast + brightness + histogram equalization).
        
        Combines multiple techniques for comprehensive automatic enhancement:
        - Auto-level for optimal dynamic range
        - Histogram equalization for better contrast
        - Auto-brightness for balanced exposure
        
        Returns:
            New Image with full automatic enhancement
        """
        return self.__class__(self._rust_image.auto_enhance())

    def exposure_adjust(self, exposure: float) -> "Image":
        """
        Adjust exposure (like camera exposure compensation).
        
        Args:
            exposure: Exposure adjustment in stops
                     > 0: Increase exposure (brighten)
                     < 0: Decrease exposure (darken)
                     0: No change
        
        Returns:
            New Image with adjusted exposure
        """
        return self.__class__(self._rust_image.exposure_adjust(exposure))

    def auto_level(self, black_clip: float = 0.01, white_clip: float = 0.01) -> "Image":
        """
        Automatically adjust levels for optimal dynamic range.
        
        Args:
            black_clip: Percentage of darkest pixels to clip (default: 0.01)
            white_clip: Percentage of brightest pixels to clip (default: 0.01)
        
        Returns:
            New Image with optimized levels
        """
        return self.__class__(self._rust_image.auto_level(black_clip, white_clip))

    def normalize(self) -> "Image":
        """
        Normalize image to use full dynamic range (0-255).
        
        Stretches the pixel values to span the full range without clipping.
        
        Returns:
            New normalized Image
        """
        return self.__class__(self._rust_image.normalize())

    def smart_enhance(self, strength: float = 1.0) -> "Image":
        """
        Smart enhancement with adjustable strength.
        
        Applies auto-contrast with controlled blending between original
        and enhanced versions for natural-looking results.
        
        Args:
            strength: Enhancement strength from 0.0 to 1.0
                     0.0: No enhancement
                     1.0: Full enhancement
        
        Returns:
            New Image with smart enhancement
        """
        return self.__class__(self._rust_image.smart_enhance(strength))

    def auto_white_balance(self) -> "Image":
        """
        Automatically correct white balance/color temperature.
        
        Uses gray world assumption to neutralize color casts and
        correct color temperature for more natural colors.
        
        Returns:
            New Image with corrected white balance
        """
        return self.__class__(self._rust_image.auto_white_balance())

