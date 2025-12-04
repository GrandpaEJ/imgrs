"""
Enumerations and constants matching Pillow's API
"""

from pathlib import Path
from typing import Dict, List, Optional


class ImageMode:
    """Image mode constants."""

    # Grayscale modes
    L = "L"  # 8-bit grayscale
    LA = "LA"  # 8-bit grayscale + alpha
    INTEGER = "I"  # 32-bit integer grayscale

    # Color modes
    RGB = "RGB"  # 8-bit RGB
    RGBA = "RGBA"  # 8-bit RGB + alpha
    CMYK = "CMYK"  # 8-bit CMYK
    YCbCr = "YCbCr"  # 8-bit YCbCr
    HSV = "HSV"  # 8-bit HSV

    # Binary mode
    BINARY = "1"  # 1-bit binary


class ImageFormat:
    """Image format constants."""

    # Lossy formats
    JPEG = "JPEG"
    JPG = "JPG"
    WEBP = "WEBP"
    AVIF = "AVIF"
    HEIF = "HEIF"
    
    # Lossless formats
    PNG = "PNG"
    GIF = "GIF"
    BMP = "BMP"
    TIFF = "TIFF"
    ICO = "ICO"
    TGA = "TGA"
    PNM = "PNM"
    DDS = "DDS"
    FARBFELD = "FARBFELD"
    
    # Vector formats
    SVG = "SVG"
    PDF = "PDF"
    EPS = "EPS"
    
    # Specialized formats
    PCX = "PCX"
    PSD = "PSD"
    TGA = "TGA"
    JPEG2000 = "JPEG2000"
    JXL = "JXL"
    
    # Format groups
    @classmethod
    def get_lossy_formats(cls) -> List[str]:
        """Get list of lossy compression formats."""
        return [cls.JPEG, cls.JPG, cls.WEBP, cls.AVIF, cls.HEIF]
    
    @classmethod
    def get_lossless_formats(cls) -> List[str]:
        """Get list of lossless compression formats."""
        return [cls.PNG, cls.GIF, cls.BMP, cls.TIFF, cls.ICO]
    
    @classmethod
    def get_supported_formats(cls) -> List[str]:
        """Get list of all supported formats."""
        return [
            cls.JPEG, cls.JPG, cls.PNG, cls.GIF, cls.BMP, cls.WEBP,
            cls.TIFF, cls.ICO, cls.AVIF, cls.HEIF, cls.SVG, cls.PDF
        ]
    
    @classmethod
    def get_extension_mapping(cls) -> Dict[str, str]:
        """Get mapping of file extensions to format names."""
        return {
            '.jpg': cls.JPG,
            '.jpeg': cls.JPEG,
            '.png': cls.PNG,
            '.gif': cls.GIF,
            '.bmp': cls.BMP,
            '.webp': cls.WEBP,
            '.tiff': cls.TIFF,
            '.tif': cls.TIFF,
            '.ico': cls.ICO,
            '.avif': cls.AVIF,
            '.heif': cls.HEIF,
            '.heic': cls.HEIF,
            '.svg': cls.SVG,
            '.pdf': cls.PDF,
            '.eps': cls.EPS,
            '.pcx': cls.PCX,
            '.psd': cls.PSD,
            '.jp2': cls.JPEG2000,
            '.j2k': cls.JPEG2000,
            '.jxl': cls.JXL,
            '.tga': cls.TGA,
        }
    
    @classmethod
    def detect_format_from_extension(cls, filename: str) -> Optional[str]:
        """Detect image format from file extension."""
        ext = Path(filename).suffix.lower()
        mapping = cls.get_extension_mapping()
        return mapping.get(ext)
    
    @classmethod
    def is_supported(cls, format_name: str) -> bool:
        """Check if a format is supported."""
        return format_name.upper() in cls.get_supported_formats()
    
    @classmethod
    def validate_format(cls, format_name: str) -> str:
        """Validate and normalize format name."""
        if not format_name:
            raise ValueError("Format name cannot be empty")
        
        format_upper = format_name.upper()
        
        # Handle common aliases
        aliases = {
            'JPG': cls.JPEG,
            'JPE': cls.JPEG,
            'JPEG2000': cls.JPEG2000,
            'JP2': cls.JPEG2000,
            'HEIC': cls.HEIF,
            'PDF': cls.PDF,
            'SVG': cls.SVG,
            'EPS': cls.EPS,
        }
        
        if format_upper in aliases:
            return aliases[format_upper]
        
        # Check if it's a valid format
        if not cls.is_supported(format_upper):
            supported = ", ".join(cls.get_supported_formats())
            raise ValueError(
                f"Unsupported format '{format_name}'. "
                f"Supported formats: {supported}"
            )
        
        return format_upper


class Resampling:
    """Resampling filter constants."""

    NEAREST = "NEAREST"
    BILINEAR = "BILINEAR"
    BICUBIC = "BICUBIC"
    LANCZOS = "LANCZOS"

    # Pillow compatibility - numeric constants
    NEAREST_INT = 0
    BILINEAR_INT = 1
    BICUBIC_INT = 2
    LANCZOS_INT = 3

    @classmethod
    def from_int(cls, value: int) -> str:
        """Convert integer resampling constant to string."""
        mapping = {
            cls.NEAREST_INT: cls.NEAREST,
            cls.BILINEAR_INT: cls.BILINEAR,
            cls.BICUBIC_INT: cls.BICUBIC,
            cls.LANCZOS_INT: cls.LANCZOS,
        }
        return mapping.get(value, cls.BILINEAR)


class Transpose:
    """Transpose method constants."""

    FLIP_LEFT_RIGHT = "FLIP_LEFT_RIGHT"
    FLIP_TOP_BOTTOM = "FLIP_TOP_BOTTOM"
    ROTATE_90 = "ROTATE_90"
    ROTATE_180 = "ROTATE_180"
    ROTATE_270 = "ROTATE_270"
    TRANSPOSE = "TRANSPOSE"
    TRANSVERSE = "TRANSVERSE"

    # Pillow compatibility - numeric constants
    FLIP_LEFT_RIGHT_INT = 0
    FLIP_TOP_BOTTOM_INT = 1
    ROTATE_90_INT = 2
    ROTATE_180_INT = 3
    ROTATE_270_INT = 4
    TRANSPOSE_INT = 5
    TRANSVERSE_INT = 6

    @classmethod
    def from_int(cls, value: int) -> str:
        """Convert integer transpose constant to string."""
        mapping = {
            cls.FLIP_LEFT_RIGHT_INT: cls.FLIP_LEFT_RIGHT,
            cls.FLIP_TOP_BOTTOM_INT: cls.FLIP_TOP_BOTTOM,
            cls.ROTATE_90_INT: cls.ROTATE_90,
            cls.ROTATE_180_INT: cls.ROTATE_180,
            cls.ROTATE_270_INT: cls.ROTATE_270,
            cls.TRANSPOSE_INT: cls.TRANSPOSE,
            cls.TRANSVERSE_INT: cls.TRANSVERSE,
        }
        return mapping.get(value, cls.FLIP_LEFT_RIGHT)


# Enhanced Color System Enums


class BlendMode:
    """Blend mode constants for compositing operations."""

    NORMAL = "normal"
    MULTIPLY = "multiply"
    SCREEN = "screen"
    OVERLAY = "overlay"
    SOFT_LIGHT = "soft_light"
    HARD_LIGHT = "hard_light"
    COLOR_DODGE = "color_dodge"
    COLOR_BURN = "color_burn"
    DARKEN = "darken"
    LIGHTEN = "lighten"
    DIFFERENCE = "difference"
    EXCLUSION = "exclusion"


class MaskType:
    """Mask type constants for masking operations."""

    GRADIENT = "gradient"
    COLOR_BASED = "color_based"
    LUMINANCE = "luminance"
    SHAPE = "shape"
    TEXTURE = "texture"
    NOISE = "noise"


class ColorFormat:
    """Color format constants for color space operations."""

    RGB = "rgb"
    RGBA = "rgba"
    HSL = "hsl"
    HSV = "hsv"
    LAB = "lab"
    XYZ = "xyz"
    CMYK = "cmyk"
    YCBCR = "ycbcr"
    YUV = "yuv"
    HSL_PRECISE = "hsl_precise"
    HSV_PRECISE = "hsv_precise"


class GradientDirection:
    """Gradient direction constants."""

    HORIZONTAL = "horizontal"
    VERTICAL = "vertical"
    DIAGONAL = "diagonal"
    RADIAL = "radial"
    ANGULAR = "angular"
    CONICAL = "conical"


class MaskOperation:
    """Mask combination operation constants."""

    MULTIPLY = "multiply"
    ADD = "add"
    SUBTRACT = "subtract"
    OVERLAY = "overlay"
    SCREEN = "screen"
    DIFFERENCE = "difference"


class ColorSpace:
    """Color space constants for conversions."""

    SRGB = "srgb"
    ADOBE_RGB = "adobe_rgb"
    PROPHOTO_RGB = "prophoto_rgb"
    DCI_P3 = "dci_p3"
    REC2020 = "rec2020"
    DISPLAY_P3 = "display_p3"
