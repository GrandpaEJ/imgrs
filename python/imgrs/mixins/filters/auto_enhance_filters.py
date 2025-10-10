"""Auto-enhancement filter operations"""


class AutoEnhanceFiltersMixin:
    """Mixin for automatic enhancement operations"""

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

