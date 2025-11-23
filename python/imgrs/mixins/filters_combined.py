"""
Combined filter mixin that includes all filter categories
This replaces the old monolithic filter_mixin.py
"""

from .filters import (
    ArtisticFiltersMixin,
    AutoEnhanceFiltersMixin,
    BasicFiltersMixin,
    BlurFiltersMixin,
    ColorFiltersMixin,
    CompositeFiltersMixin,
    CSSFiltersMixin,
    EdgeFiltersMixin,
    MorphologicalFiltersMixin,
    NoiseFiltersMixin,
    SharpenFiltersMixin,
    StylisticFiltersMixin,
)


class FilterMixin(
    BasicFiltersMixin,
    BlurFiltersMixin,
    EdgeFiltersMixin,
    SharpenFiltersMixin,
    StylisticFiltersMixin,
    NoiseFiltersMixin,
    MorphologicalFiltersMixin,
    ArtisticFiltersMixin,
    ColorFiltersMixin,
    CSSFiltersMixin,
    CompositeFiltersMixin,
    AutoEnhanceFiltersMixin,
):
    """
    Combined filter mixin providing all filter operations.

    Organized into 12 categories:
    - BasicFiltersMixin: blur, sharpen, edge_detect, emboss, brightness, contrast
    - BlurFiltersMixin: box_blur, motion_blur, median_blur, bilateral_blur, radial_blur, zoom_blur
    - EdgeFiltersMixin: prewitt, scharr, roberts_cross, laplacian, LoG, canny
    - SharpenFiltersMixin: unsharp_mask, high_pass, edge_enhance
    - StylisticFiltersMixin: oil_painting, pixelate, mosaic, cartoon, sketch, solarize
    - NoiseFiltersMixin: add_gaussian_noise, add_salt_pepper_noise, denoise
    - MorphologicalFiltersMixin: dilate, erode, opening, closing, gradient
    - ArtisticFiltersMixin: vignette, halftone, pencil_sketch, watercolor, glitch
    - ColorFiltersMixin: duotone, color_splash, chromatic_aberration, chroma_key
    - CSSFiltersMixin: sepia, grayscale_filter, invert, hue_rotate, saturate
    - CompositeFiltersMixin: composite (28 CSS composite/blending modes)
    - AutoEnhanceFiltersMixin: histogram_equalization, auto_contrast, auto_enhance, exposure_adjust, etc.
    """

    pass  # All functionality from component mixins
