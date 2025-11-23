"""
Filter mixins for image processing operations.
Organized into specialized categories for better maintainability.
"""

from .artistic_filters import ArtisticFiltersMixin
from .auto_enhance_filters import AutoEnhanceFiltersMixin
from .basic_filters import BasicFiltersMixin
from .blur_filters import BlurFiltersMixin
from .color_filters import ColorFiltersMixin
from .composite_filters import CompositeFiltersMixin
from .css_filters import CSSFiltersMixin
from .edge_filters import EdgeFiltersMixin
from .morphological_filters import MorphologicalFiltersMixin
from .noise_filters import NoiseFiltersMixin
from .sharpen_filters import SharpenFiltersMixin
from .stylistic_filters import StylisticFiltersMixin

__all__ = [
    "BasicFiltersMixin",
    "BlurFiltersMixin",
    "EdgeFiltersMixin",
    "SharpenFiltersMixin",
    "StylisticFiltersMixin",
    "NoiseFiltersMixin",
    "MorphologicalFiltersMixin",
    "ArtisticFiltersMixin",
    "ColorFiltersMixin",
    "CSSFiltersMixin",
    "AutoEnhanceFiltersMixin",
    "CompositeFiltersMixin",
]
