"""
Filter mixins organized by category
"""

from .basic_filters import BasicFiltersMixin
from .blur_filters import BlurFiltersMixin
from .edge_filters import EdgeFiltersMixin
from .sharpen_filters import SharpenFiltersMixin
from .stylistic_filters import StylisticFiltersMixin
from .noise_filters import NoiseFiltersMixin
from .morphological_filters import MorphologicalFiltersMixin
from .artistic_filters import ArtisticFiltersMixin
from .color_filters import ColorFiltersMixin
from .css_filters import CSSFiltersMixin
from .auto_enhance_filters import AutoEnhanceFiltersMixin

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
]

